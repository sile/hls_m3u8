use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;

use derive_builder::Builder;
use shorthand::ShortHand;

use crate::attribute::AttributePairs;
use crate::types::ProtocolVersion;
use crate::utils::{quote, tag, unquote};
use crate::{Error, RequiredVersion};

/// The data of [`ExtXSessionData`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SessionData<'a> {
    /// Contains the data identified by the [`ExtXSessionData::data_id`].
    ///
    /// If a [`language`] is specified, this variant should contain a
    /// human-readable string written in the specified language.
    ///
    /// [`data_id`]: ExtXSessionData::data_id
    /// [`language`]: ExtXSessionData::language
    Value(Cow<'a, str>),
    /// An [`URI`], which points to a [`json`] file.
    ///
    /// [`json`]: https://tools.ietf.org/html/rfc8259
    /// [`URI`]: https://tools.ietf.org/html/rfc3986
    Uri(Cow<'a, str>),
}

impl<'a> SessionData<'a> {
    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> SessionData<'static> {
        match self {
            Self::Value(v) => SessionData::Value(Cow::Owned(v.into_owned())),
            Self::Uri(v) => SessionData::Uri(Cow::Owned(v.into_owned())),
        }
    }
}

/// Allows arbitrary session data to be carried in a [`MasterPlaylist`].
///
/// [`MasterPlaylist`]: crate::MasterPlaylist
#[derive(ShortHand, Builder, Hash, Eq, Ord, Debug, PartialEq, Clone, PartialOrd)]
#[builder(setter(into))]
#[shorthand(enable(must_use, into))]
pub struct ExtXSessionData<'a> {
    /// This should conform to a [reverse DNS] naming convention, such as
    /// `com.example.movie.title`.
    ///
    /// # Note
    ///
    /// There is no central registration authority, so a value
    /// should be choosen, that is unlikely to collide with others.
    ///
    /// This field is required.
    ///
    /// [reverse DNS]: https://en.wikipedia.org/wiki/Reverse_domain_name_notation
    data_id: Cow<'a, str>,
    /// The [`SessionData`] associated with the
    /// [`data_id`](ExtXSessionData::data_id).
    ///
    /// # Note
    ///
    /// This field is required.
    #[shorthand(enable(skip))]
    pub data: SessionData<'a>,
    /// The `language` attribute identifies the language of the [`SessionData`].
    ///
    /// # Note
    ///
    /// This field is optional and the provided value should conform to
    /// [RFC5646].
    ///
    /// [RFC5646]: https://tools.ietf.org/html/rfc5646
    #[builder(setter(strip_option), default)]
    language: Option<Cow<'a, str>>,
}

impl<'a> ExtXSessionData<'a> {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SESSION-DATA:";

    /// Makes a new [`ExtXSessionData`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXSessionData;
    /// use hls_m3u8::tags::SessionData;
    ///
    /// let session_data = ExtXSessionData::new(
    ///     "com.example.movie.title",
    ///     SessionData::Uri("https://www.example.com/".into()),
    /// );
    /// ```
    #[must_use]
    pub fn new<T: Into<Cow<'a, str>>>(data_id: T, data: SessionData<'a>) -> Self {
        Self {
            data_id: data_id.into(),
            data,
            language: None,
        }
    }

    /// Returns a builder for [`ExtXSessionData`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXSessionData;
    /// use hls_m3u8::tags::SessionData;
    ///
    /// let session_data = ExtXSessionData::builder()
    ///     .data_id("com.example.movie.title")
    ///     .data(SessionData::Value("some data".into()))
    ///     .language("en")
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn builder() -> ExtXSessionDataBuilder<'a> { ExtXSessionDataBuilder::default() }

    /// Makes a new [`ExtXSessionData`] tag, with the given language.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXSessionData;
    /// use hls_m3u8::tags::SessionData;
    ///
    /// let session_data = ExtXSessionData::with_language(
    ///     "com.example.movie.title",
    ///     SessionData::Value("some data".into()),
    ///     "en",
    /// );
    /// ```
    #[must_use]
    pub fn with_language<T, K>(data_id: T, data: SessionData<'a>, language: K) -> Self
    where
        T: Into<Cow<'a, str>>,
        K: Into<Cow<'a, str>>,
    {
        Self {
            data_id: data_id.into(),
            data,
            language: Some(language.into()),
        }
    }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> ExtXSessionData<'static> {
        ExtXSessionData {
            data_id: Cow::Owned(self.data_id.into_owned()),
            data: self.data.into_owned(),
            language: self.language.map(|v| Cow::Owned(v.into_owned())),
        }
    }
}

/// This tag requires [`ProtocolVersion::V1`].
impl<'a> RequiredVersion for ExtXSessionData<'a> {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl<'a> fmt::Display for ExtXSessionData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "DATA-ID={}", quote(&self.data_id))?;

        match &self.data {
            SessionData::Value(value) => write!(f, ",VALUE={}", quote(value))?,
            SessionData::Uri(value) => write!(f, ",URI={}", quote(value))?,
        }

        if let Some(value) = &self.language {
            write!(f, ",LANGUAGE={}", quote(value))?;
        }

        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for ExtXSessionData<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let input = tag(input, Self::PREFIX)?;

        let mut data_id = None;
        let mut session_value = None;
        let mut uri = None;
        let mut language = None;

        for (key, value) in AttributePairs::new(input) {
            match key {
                "DATA-ID" => data_id = Some(unquote(value)),
                "VALUE" => session_value = Some(unquote(value)),
                "URI" => uri = Some(unquote(value)),
                "LANGUAGE" => language = Some(unquote(value)),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized
                    // AttributeName.
                }
            }
        }

        let data_id = data_id.ok_or_else(|| Error::missing_value("EXT-X-DATA-ID"))?;

        let data = {
            if let Some(value) = session_value {
                if uri.is_some() {
                    return Err(Error::custom("unexpected URI"));
                }

                SessionData::Value(value)
            } else if let Some(uri) = uri {
                SessionData::Uri(uri)
            } else {
                return Err(Error::custom(
                    "expected either `SessionData::Uri` or `SessionData::Value`",
                ));
            }
        };

        Ok(Self {
            data_id,
            data,
            language,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! generate_tests {
        ( $( { $struct:expr, $str:expr } ),+ $(,)* ) => {
            #[test]
            fn test_display() {
                $(
                    assert_eq!($struct.to_string(), $str.to_string());
                )+
            }

            #[test]
            fn test_parser() {
                $(
                    assert_eq!($struct, TryFrom::try_from($str).unwrap());
                )+

                assert!(
                    ExtXSessionData::try_from(concat!(
                        "#EXT-X-SESSION-DATA:",
                        "DATA-ID=\"foo\",",
                        "LANGUAGE=\"baz\""
                    ))
                    .is_err()
                );

                assert!(
                    ExtXSessionData::try_from(concat!(
                        "#EXT-X-SESSION-DATA:",
                        "DATA-ID=\"foo\",",
                        "LANGUAGE=\"baz\",",
                        "VALUE=\"VALUE\",",
                        "URI=\"https://www.example.com/\""
                    ))
                    .is_err()
                );
            }

        }
    }

    generate_tests! {
        {
            ExtXSessionData::new(
                "com.example.lyrics",
                SessionData::Uri("lyrics.json".into())
            ),
            concat!(
                "#EXT-X-SESSION-DATA:",
                "DATA-ID=\"com.example.lyrics\",",
                "URI=\"lyrics.json\""
            )
        },
        {
            ExtXSessionData::with_language(
                "com.example.title",
                SessionData::Value("This is an example".into()),
                "en"
            ),
            concat!(
                "#EXT-X-SESSION-DATA:",
                "DATA-ID=\"com.example.title\",",
                "VALUE=\"This is an example\",",
                "LANGUAGE=\"en\""
            )
        },
        {
            ExtXSessionData::with_language(
                "com.example.title",
                SessionData::Value("Este es un ejemplo".into()),
                "es"
            ),
            concat!(
                "#EXT-X-SESSION-DATA:",
                "DATA-ID=\"com.example.title\",",
                "VALUE=\"Este es un ejemplo\",",
                "LANGUAGE=\"es\""
            )
        }
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXSessionData::new("com.example.lyrics", SessionData::Uri("lyrics.json".into()))
                .required_version(),
            ProtocolVersion::V1
        );
    }
}
