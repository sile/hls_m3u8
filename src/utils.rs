use crate::Error;

pub(crate) fn parse_yes_or_no<T: AsRef<str>>(s: T) -> crate::Result<bool> {
    match s.as_ref() {
        "YES" => Ok(true),
        "NO" => Ok(false),
        _ => Err(Error::invalid_input()),
    }
}

pub(crate) fn parse_u64<T: AsRef<str>>(s: T) -> crate::Result<u64> {
    let n = s.as_ref().parse().map_err(Error::unknown)?; // TODO: Error::number
    Ok(n)
}

/// According to the documentation the following characters are forbidden
/// inside a quoted string:
/// - carriage return (`\r`)
/// - new line (`\n`)
/// - double quotes (`"`)
///
/// Therefore it is safe to simply remove any occurence of those characters.
/// [rfc8216#section-4.2](https://tools.ietf.org/html/rfc8216#section-4.2)
pub(crate) fn unquote<T: ToString>(value: T) -> String {
    value
        .to_string()
        .replace("\"", "")
        .replace("\n", "")
        .replace("\r", "")
}

/// Puts a string inside quotes.
pub(crate) fn quote<T: ToString>(value: T) -> String {
    // the replace is for the case, that quote is called on an already quoted string, which could
    // cause problems!
    format!("\"{}\"", value.to_string().replace("\"", ""))
}

/// Checks, if the given tag is at the start of the input. If this is the case, it will remove it
/// and return the rest of the input.
///
/// # Error
/// This function will return `Error::MissingTag`, if the input doesn't start with the tag, that
/// has been passed to this function.
pub(crate) fn tag<T>(input: &str, tag: T) -> crate::Result<&str>
where
    T: AsRef<str>,
{
    if !input.trim().starts_with(tag.as_ref()) {
        return Err(Error::missing_tag(tag.as_ref(), input));
    }
    let result = input.split_at(tag.as_ref().len()).1;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yes_or_no() {
        assert!(parse_yes_or_no("YES").unwrap());
        assert!(!parse_yes_or_no("NO").unwrap());
        // TODO: test for error
    }

    #[test]
    fn test_parse_u64() {
        assert_eq!(parse_u64("1").unwrap(), 1);
        assert_eq!(parse_u64("25").unwrap(), 25);
        // TODO: test for error
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
    }
}
