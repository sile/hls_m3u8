use std::fmt;
use std::str::FromStr;

use getset::{Getters, MutGetters, Setters};

use crate::attribute::AttributePairs;
use crate::types::ProtocolVersion;
use crate::utils::{quote, tag, unquote};
use crate::Error;

/// Session data.
///
/// See: [4.3.4.4. EXT-X-SESSION-DATA]
///
/// [4.3.4.4. EXT-X-SESSION-DATA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.4
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SessionData {
    Value(String),
    Uri(String),
}

/// [4.3.4.4. EXT-X-SESSION-DATA]
///
/// [4.3.4.4. EXT-X-SESSION-DATA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.4
#[derive(Getters, MutGetters, Setters, Debug, Clone, PartialEq, Eq, Hash)]
#[get = "pub"]
#[set = "pub"]
#[get_mut = "pub"]
pub struct ExtXSessionData {
    /// The identifier of the data.
    data_id: String,
    /// The session data.
    data: SessionData,
    /// The language of the data.
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

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
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
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let data_id = data_id.ok_or(Error::missing_value("EXT-X-DATA-ID"))?;
        let data = {
            if let Some(value) = session_value {
                if uri.is_some() {
                    return Err(Error::invalid_input());
                } else {
                    SessionData::Value(value)
                }
            } else if let Some(uri) = uri {
                SessionData::Uri(uri)
            } else {
                return Err(Error::invalid_input());
            }
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
    fn test_display() {
        let tag = ExtXSessionData::new("foo", SessionData::Value("bar".into()));
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",VALUE="bar""#;
        assert_eq!(tag.to_string(), text);

        let tag = ExtXSessionData::new("foo", SessionData::Uri("bar".into()));
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",URI="bar""#;
        assert_eq!(tag.to_string(), text);

        let tag = ExtXSessionData::with_language("foo", SessionData::Value("bar".into()), "baz");
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",VALUE="bar",LANGUAGE="baz""#;
        assert_eq!(tag.to_string(), text);
    }

    #[test]
    fn test_parser() {
        let tag = ExtXSessionData::new("foo", SessionData::Value("bar".into()));
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",VALUE="bar""#;
        assert_eq!(text.parse::<ExtXSessionData>().unwrap(), tag);

        let tag = ExtXSessionData::new("foo", SessionData::Uri("bar".into()));
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",URI="bar""#;
        assert_eq!(text.parse::<ExtXSessionData>().unwrap(), tag);

        let tag = ExtXSessionData::with_language("foo", SessionData::Value("bar".into()), "baz");
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",VALUE="bar",LANGUAGE="baz""#;
        assert_eq!(text.parse::<ExtXSessionData>().unwrap(), tag);
    }

    #[test]
    fn test_requires_version() {
        let tag = ExtXSessionData::new("foo", SessionData::Value("bar".into()));
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }
}
