use core::iter;
use std::borrow::Cow;

use crate::Error;

/// This is an extension trait that adds the below method to `bool`.
/// Those methods are already planned for the standard library, but are not
/// stable at the time of writing this comment.
///
/// The current status can be seen here:
/// <https://github.com/rust-lang/rust/issues/64260>
///
/// This trait exists to allow publishing a new version (requires stable
/// release) and the functions are prefixed with an `a` to prevent naming
/// conflicts with the coming std functions.
// TODO: replace this trait with std version as soon as it is stabilized
pub(crate) trait BoolExt {
    #[must_use]
    fn athen_some<T>(self, t: T) -> Option<T>;

    #[must_use]
    fn athen<T, F: FnOnce() -> T>(self, f: F) -> Option<T>;
}

impl BoolExt for bool {
    #[inline]
    fn athen_some<T>(self, t: T) -> Option<T> {
        if self {
            Some(t)
        } else {
            None
        }
    }

    #[inline]
    fn athen<T, F: FnOnce() -> T>(self, f: F) -> Option<T> {
        if self {
            Some(f())
        } else {
            None
        }
    }
}

macro_rules! required_version {
    ( $( $tag:expr ),* ) => {
        ::core::iter::empty()
            $(
                .chain(::core::iter::once($tag.required_version()))
            )*
            .max()
            .unwrap_or_default()
    }
}

pub(crate) fn parse_yes_or_no<T: AsRef<str>>(s: T) -> crate::Result<bool> {
    match s.as_ref() {
        "YES" => Ok(true),
        "NO" => Ok(false),
        _ => Err(Error::invalid_input()),
    }
}

/// According to the documentation the following characters are forbidden
/// inside a quoted string:
/// - carriage return (`\r`)
/// - new line (`\n`)
/// - double quotes (`"`)
///
/// Therefore it is safe to simply remove any occurence of those characters.
/// [rfc8216#section-4.2](https://tools.ietf.org/html/rfc8216#section-4.2)
pub(crate) fn unquote(value: &str) -> Cow<'_, str> {
    if value.starts_with('"') && value.ends_with('"') {
        let result = Cow::Borrowed(&value[1..value.len() - 1]);

        if result
            .chars()
            .find(|c| *c == '"' || *c == '\n' || *c == '\r')
            .is_none()
        {
            return result;
        }
    }

    Cow::Owned(
        value
            .chars()
            .filter(|c| *c != '"' && *c != '\n' && *c != '\r')
            .collect(),
    )
}

/// Puts a string inside quotes.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn quote<T: ToString>(value: T) -> String {
    // the replace is for the case, that quote is called on an already quoted
    // string, which could cause problems!
    iter::once('"')
        .chain(value.to_string().chars().filter(|c| *c != '"'))
        .chain(iter::once('"'))
        .collect()
}

/// Checks, if the given tag is at the start of the input. If this is the case,
/// it will remove it and return the rest of the input.
///
/// # Error
///
/// This function will return `Error::MissingTag`, if the input doesn't start
/// with the tag, that has been passed to this function.
pub(crate) fn tag<T>(input: &str, tag: T) -> crate::Result<&str>
where
    T: AsRef<str>,
{
    if !input.trim().starts_with(tag.as_ref()) {
        return Err(Error::missing_tag(tag.as_ref(), input));
    }

    Ok(input.trim().split_at(tag.as_ref().len()).1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_yes_or_no() {
        assert!(parse_yes_or_no("YES").unwrap());
        assert!(!parse_yes_or_no("NO").unwrap());
        assert!(parse_yes_or_no("garbage").is_err());
    }

    #[test]
    fn test_unquote() {
        assert_eq!(unquote("\"TestValue\""), "TestValue".to_string());
        assert_eq!(unquote("\"TestValue\n\""), "TestValue".to_string());
        assert_eq!(unquote("\"TestValue\n\r\""), "TestValue".to_string());
    }

    #[test]
    fn test_quote() {
        assert_eq!(quote("value"), "\"value\"".to_string());
        assert_eq!(quote("\"value\""), "\"value\"".to_string());
    }

    #[test]
    fn test_tag() {
        let input = "HelloMyFriendThisIsASampleString";

        let input = tag(input, "Hello").unwrap();
        assert_eq!(input, "MyFriendThisIsASampleString");

        let input = tag(input, "My").unwrap();
        assert_eq!(input, "FriendThisIsASampleString");

        let input = tag(input, "FriendThisIs").unwrap();
        assert_eq!(input, "ASampleString");

        let input = tag(input, "A").unwrap();
        assert_eq!(input, "SampleString");

        assert!(tag(input, "B").is_err());

        assert_eq!(
            tag(
                concat!(
                    "\n    #EXTM3U\n",
                    "    #EXT-X-TARGETDURATION:5220\n",
                    "    #EXTINF:0,\n",
                    "    http://media.example.com/entire1.ts\n",
                    "    #EXTINF:5220,\n",
                    "    http://media.example.com/entire2.ts\n",
                    "    #EXT-X-ENDLIST"
                ),
                "#EXTM3U"
            )
            .unwrap(),
            concat!(
                "\n",
                "    #EXT-X-TARGETDURATION:5220\n",
                "    #EXTINF:0,\n",
                "    http://media.example.com/entire1.ts\n",
                "    #EXTINF:5220,\n",
                "    http://media.example.com/entire2.ts\n",
                "    #EXT-X-ENDLIST"
            )
        );
    }
}
