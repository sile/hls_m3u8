use std::fmt;
use std::str::FromStr;

use derive_builder::Builder;

use crate::attribute::AttributePairs;
use crate::types::{ProtocolVersion, RequiredVersion};
use crate::utils::{quote, tag, unquote};
use crate::Error;

/// The data of an [ExtXSessionData] tag.
#[derive(Hash, Eq, Ord, Debug, PartialEq, Clone, PartialOrd)]
pub enum SessionData {
    /// A String, that contains the data identified by
    /// [`data_id`](ExtXSessionData::data_id).
    /// If a [`language`](ExtXSessionData::language) is specified, the value
    /// should contain a human-readable string written in the specified
    /// language.
    Value(String),
    /// An [`uri`], which points to a [`json`].
    ///
    /// [`json`]: https://tools.ietf.org/html/rfc8259
    /// [`uri`]: https://tools.ietf.org/html/rfc3986
    Uri(String),
}

/// # [4.3.4.4. EXT-X-SESSION-DATA]
///
/// The [`ExtXSessionData`] tag allows arbitrary session data to be
/// carried in a [`Master Playlist`].
///
/// [`Master Playlist`]: crate::MasterPlaylist
/// [4.3.4.4. EXT-X-SESSION-DATA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.4
#[derive(Builder, Hash, Eq, Ord, Debug, PartialEq, Clone, PartialOrd)]
#[builder(setter(into))]
pub struct ExtXSessionData {
    /// The identifier of the data.
    /// For more information look [`here`](ExtXSessionData::set_data_id).
    ///
    /// # Note
    /// This field is required.
    data_id: String,
    /// The data associated with the
    /// [`data_id`](ExtXSessionDataBuilder::data_id).
    /// For more information look [`here`](SessionData).
    ///
    /// # Note
    /// This field is required.
    data: SessionData,
    /// The language of the [`data`](ExtXSessionDataBuilder::data).
    #[builder(setter(into, strip_option), default)]
    language: Option<String>,
}

impl ExtXSessionData {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SESSION-DATA:";

    /// Makes a new [`ExtXSessionData`] tag.
    ///
    /// # Example
    /// ```
    /// use hls_m3u8::tags::{ExtXSessionData, SessionData};
    ///
    /// ExtXSessionData::new(
    ///     "com.example.movie.title",
    ///     SessionData::Uri("https://www.example.com/".to_string()),
    /// );
    /// ```
    pub fn new<T: ToString>(data_id: T, data: SessionData) -> Self {
        Self {
            data_id: data_id.to_string(),
            data,
            language: None,
        }
    }

    /// Returns a new Builder for [`ExtXSessionData`].
    ///
    /// # Example
    /// ```
    /// use hls_m3u8::tags::{ExtXSessionData, SessionData};
    ///
    /// let session_data = ExtXSessionData::builder()
    ///     .data_id("com.example.movie.title")
    ///     .data(SessionData::Value("some data".to_string()))
    ///     .language("english")
    ///     .build()
    ///     .expect("Failed to build an ExtXSessionData tag.");
    ///
    /// assert_eq!(
    ///     session_data,
    ///     ExtXSessionData::with_language(
    ///         "com.example.movie.title",
    ///         SessionData::Value("some data".to_string()),
    ///         "english"
    ///     )
    /// );
    /// ```
    pub fn builder() -> ExtXSessionDataBuilder { ExtXSessionDataBuilder::default() }

    /// Makes a new [`ExtXSessionData`] tag, with the given language.
    ///
    /// # Example
    /// ```
    /// use hls_m3u8::tags::{ExtXSessionData, SessionData};
    ///
    /// let session_data = ExtXSessionData::with_language(
    ///     "com.example.movie.title",
    ///     SessionData::Value("some data".to_string()),
    ///     "english",
    /// );
    /// ```
    pub fn with_language<T: ToString>(data_id: T, data: SessionData, language: T) -> Self {
        Self {
            data_id: data_id.to_string(),
            data,
            language: Some(language.to_string()),
        }
    }

    /// Returns the `data_id`, that identifies a `data_value`.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::{ExtXSessionData, SessionData};
    /// #
    /// let data = ExtXSessionData::new(
    ///     "com.example.movie.title",
    ///     SessionData::Value("some data".to_string())
    /// );
    ///
    /// assert_eq!(
    ///     data.data_id(),
    ///     &"com.example.movie.title".to_string()
    /// )
    /// ```
    pub const fn data_id(&self) -> &String { &self.data_id }

    /// Returns the `data`.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::{ExtXSessionData, SessionData};
    /// #
    /// let data = ExtXSessionData::new(
    ///     "com.example.movie.title",
    ///     SessionData::Value("some data".to_string())
    /// );
    ///
    /// assert_eq!(
    ///     data.data(),
    ///     &SessionData::Value("some data".to_string())
    /// )
    /// ```
    pub const fn data(&self) -> &SessionData { &self.data }

    /// Returns the `language` tag, that identifies the language of
    /// [`SessionData`].
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::{ExtXSessionData, SessionData};
    /// #
    /// let data = ExtXSessionData::with_language(
    ///     "com.example.movie.title",
    ///     SessionData::Value("some data".to_string()),
    ///     "english"
    /// );
    ///
    /// assert_eq!(
    ///     data.language(),
    ///     &Some("english".to_string())
    /// )
    /// ```
    pub const fn language(&self) -> &Option<String> { &self.language }

    /// Sets the `language` attribute, that identifies the language of
    /// [`SessionData`]. See [rfc5646](https://tools.ietf.org/html/rfc5646).
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::{ExtXSessionData, SessionData};
    /// #
    /// let mut data = ExtXSessionData::new(
    ///     "com.example.movie.title",
    ///     SessionData::Value("some data".to_string()),
    /// );
    ///
    /// assert_eq!(data.language(), &None);
    ///
    /// data.set_language(Some("english"));
    /// assert_eq!(data.language(), &Some("english".to_string()));
    /// ```
    pub fn set_language<T: ToString>(&mut self, value: Option<T>) -> &mut Self {
        self.language = value.map(|v| v.to_string());
        self
    }

    /// Sets the `data_id` attribute, that should conform to a [reverse DNS]
    /// naming convention, such as `com.example.movie.title`.
    ///
    /// # Note:
    /// There is no central registration authority, so a value
    /// should be choosen, that is unlikely to collide with others.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::{ExtXSessionData, SessionData};
    /// #
    /// let mut data = ExtXSessionData::new(
    ///     "com.example.movie.title",
    ///     SessionData::Value("some data".to_string()),
    /// );
    ///
    /// assert_eq!(data.data_id(), &"com.example.movie.title".to_string());
    ///
    /// data.set_data_id("com.other.movie.title");
    /// assert_eq!(data.data_id(), &"com.other.movie.title".to_string());
    /// ```
    /// [reverse DNS]: https://en.wikipedia.org/wiki/Reverse_domain_name_notation
    pub fn set_data_id<T: ToString>(&mut self, value: T) -> &mut Self {
        self.data_id = value.to_string();
        self
    }

    /// Sets the [`data`](ExtXSessionData::data) of this tag.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::{ExtXSessionData, SessionData};
    /// #
    /// let mut data = ExtXSessionData::new(
    ///     "com.example.movie.title",
    ///     SessionData::Value("some data".to_string()),
    /// );
    ///
    /// assert_eq!(data.data(), &SessionData::Value("some data".to_string()));
    ///
    /// data.set_data(SessionData::Value("new data".to_string()));
    /// assert_eq!(data.data(), &SessionData::Value("new data".to_string()));
    /// ```
    pub fn set_data(&mut self, value: SessionData) -> &mut Self {
        self.data = value;
        self
    }
}

impl RequiredVersion for ExtXSessionData {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXSessionData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl FromStr for ExtXSessionData {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        let mut data_id = None;
        let mut session_value = None;
        let mut uri = None;
        let mut language = None;

        for (key, value) in input.parse::<AttributePairs>()? {
            match key.as_str() {
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
                    return Err(Error::custom("Unexpected URI"));
                } else {
                    SessionData::Value(value)
                }
            } else if let Some(uri) = uri {
                SessionData::Uri(uri)
            } else {
                return Err(Error::invalid_input());
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

    #[test]
    fn test_display() {
        assert_eq!(
            "#EXT-X-SESSION-DATA:\
             DATA-ID=\"com.example.lyrics\",\
             URI=\"lyrics.json\""
                .to_string(),
            ExtXSessionData::new(
                "com.example.lyrics",
                SessionData::Uri("lyrics.json".to_string())
            )
            .to_string()
        );

        assert_eq!(
            "#EXT-X-SESSION-DATA:\
             DATA-ID=\"com.example.title\",\
             VALUE=\"This is an example\",\
             LANGUAGE=\"en\""
                .to_string(),
            ExtXSessionData::with_language(
                "com.example.title",
                SessionData::Value("This is an example".to_string()),
                "en"
            )
            .to_string()
        );

        assert_eq!(
            "#EXT-X-SESSION-DATA:\
             DATA-ID=\"com.example.title\",\
             VALUE=\"Este es un ejemplo\",\
             LANGUAGE=\"es\""
                .to_string(),
            ExtXSessionData::with_language(
                "com.example.title",
                SessionData::Value("Este es un ejemplo".to_string()),
                "es"
            )
            .to_string()
        );

        assert_eq!(
            "#EXT-X-SESSION-DATA:\
             DATA-ID=\"foo\",\
             VALUE=\"bar\""
                .to_string(),
            ExtXSessionData::new("foo", SessionData::Value("bar".into())).to_string()
        );

        assert_eq!(
            "#EXT-X-SESSION-DATA:\
             DATA-ID=\"foo\",\
             URI=\"bar\""
                .to_string(),
            ExtXSessionData::new("foo", SessionData::Uri("bar".into())).to_string()
        );

        assert_eq!(
            "#EXT-X-SESSION-DATA:\
             DATA-ID=\"foo\",\
             VALUE=\"bar\",\
             LANGUAGE=\"baz\""
                .to_string(),
            ExtXSessionData::with_language("foo", SessionData::Value("bar".into()), "baz")
                .to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            "#EXT-X-SESSION-DATA:\
             DATA-ID=\"com.example.lyrics\",\
             URI=\"lyrics.json\""
                .parse::<ExtXSessionData>()
                .unwrap(),
            ExtXSessionData::new(
                "com.example.lyrics",
                SessionData::Uri("lyrics.json".to_string())
            )
        );

        assert_eq!(
            "#EXT-X-SESSION-DATA:\
             DATA-ID=\"com.example.title\",\
             LANGUAGE=\"en\",\
             VALUE=\"This is an example\""
                .parse::<ExtXSessionData>()
                .unwrap(),
            ExtXSessionData::with_language(
                "com.example.title",
                SessionData::Value("This is an example".to_string()),
                "en"
            )
        );

        assert_eq!(
            "#EXT-X-SESSION-DATA:\
             DATA-ID=\"com.example.title\",\
             LANGUAGE=\"es\",\
             VALUE=\"Este es un ejemplo\""
                .parse::<ExtXSessionData>()
                .unwrap(),
            ExtXSessionData::with_language(
                "com.example.title",
                SessionData::Value("Este es un ejemplo".to_string()),
                "es"
            )
        );

        assert_eq!(
            "#EXT-X-SESSION-DATA:\
             DATA-ID=\"foo\",\
             VALUE=\"bar\""
                .parse::<ExtXSessionData>()
                .unwrap(),
            ExtXSessionData::new("foo", SessionData::Value("bar".into()))
        );

        assert_eq!(
            "#EXT-X-SESSION-DATA:\
             DATA-ID=\"foo\",\
             URI=\"bar\""
                .parse::<ExtXSessionData>()
                .unwrap(),
            ExtXSessionData::new("foo", SessionData::Uri("bar".into()))
        );

        assert_eq!(
            "#EXT-X-SESSION-DATA:\
             DATA-ID=\"foo\",\
             VALUE=\"bar\",\
             LANGUAGE=\"baz\",\
             UNKNOWN=TAG"
                .parse::<ExtXSessionData>()
                .unwrap(),
            ExtXSessionData::with_language("foo", SessionData::Value("bar".into()), "baz")
        );

        assert!("#EXT-X-SESSION-DATA:\
                 DATA-ID=\"foo\",\
                 LANGUAGE=\"baz\""
            .parse::<ExtXSessionData>()
            .is_err());

        assert!("#EXT-X-SESSION-DATA:\
                 DATA-ID=\"foo\",\
                 LANGUAGE=\"baz\",\
                 VALUE=\"VALUE\",\
                 URI=\"https://www.example.com/\""
            .parse::<ExtXSessionData>()
            .is_err());
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXSessionData::new(
                "com.example.lyrics",
                SessionData::Uri("lyrics.json".to_string())
            )
            .required_version(),
            ProtocolVersion::V1
        );
    }
}
