use crate::attribute::AttributePairs;
use crate::types::{ProtocolVersion, SessionData};
use crate::utils::{quote, unquote};
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;

/// [4.3.4.4. EXT-X-SESSION-DATA]
///
/// [4.3.4.4. EXT-X-SESSION-DATA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.4
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXSessionData {
    data_id: String,
    data: SessionData,
    language: Option<String>,
}

impl ExtXSessionData {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SESSION-DATA:";

    /// Makes a new `ExtXSessionData` tag.
    pub fn new<T: ToString>(data_id: T, data: SessionData) -> Self {
        ExtXSessionData {
            data_id: data_id.to_string(),
            data,
            language: None,
        }
    }

    /// Makes a new `ExtXSessionData` with the given language.
    pub fn with_language<T: ToString>(data_id: T, data: SessionData, language: T) -> Self {
        ExtXSessionData {
            data_id: data_id.to_string(),
            data,
            language: Some(language.to_string()),
        }
    }

    /// Returns the identifier of the data.
    pub const fn data_id(&self) -> &String {
        &self.data_id
    }

    /// Returns the session data.
    pub const fn data(&self) -> &SessionData {
        &self.data
    }

    /// Returns the language of the data.
    pub fn language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXSessionData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "DATA-ID={}", quote(&self.data_id))?;
        match self.data {
            SessionData::Value(ref x) => write!(f, ",VALUE={}", quote(x))?,
            SessionData::Uri(ref x) => write!(f, ",URI={}", quote(x))?,
        }
        if let Some(ref x) = self.language {
            write!(f, ",LANGUAGE={}", quote(x))?;
        }
        Ok(())
    }
}

impl FromStr for ExtXSessionData {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut data_id = None;
        let mut session_value = None;
        let mut uri = None;
        let mut language = None;
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "DATA-ID" => data_id = Some(unquote(value)),
                "VALUE" => session_value = Some(unquote(value)),
                "URI" => uri = Some(unquote(value)),
                "LANGUAGE" => language = Some(unquote(value)),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let data_id = track_assert_some!(data_id, ErrorKind::InvalidInput);
        let data = if let Some(value) = session_value {
            track_assert_eq!(uri, None, ErrorKind::InvalidInput);
            SessionData::Value(value)
        } else if let Some(uri) = uri {
            SessionData::Uri(uri)
        } else {
            track_panic!(ErrorKind::InvalidInput);
        };
        Ok(ExtXSessionData {
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
    fn ext_x_session_data() {
        let tag = ExtXSessionData::new("foo", SessionData::Value("bar".into()));
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",VALUE="bar""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);

        let tag = ExtXSessionData::new("foo", SessionData::Uri("bar".into()));
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",URI="bar""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);

        let tag = ExtXSessionData::with_language("foo", SessionData::Value("bar".into()), "baz");
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",VALUE="bar",LANGUAGE="baz""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }
}
