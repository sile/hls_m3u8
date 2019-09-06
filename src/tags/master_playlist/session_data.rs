use crate::attribute::AttributePairs;
use crate::types::{ProtocolVersion, QuotedString, SessionData};
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;


/// [4.3.4.4. EXT-X-SESSION-DATA]
///
/// [4.3.4.4. EXT-X-SESSION-DATA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.4
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXSessionData {
    data_id: QuotedString,
    data: SessionData,
    language: Option<QuotedString>,
}

impl ExtXSessionData {
    pub(crate) const PREFIX: &'static str = "#EXT-X-SESSION-DATA:";

    /// Makes a new `ExtXSessionData` tag.
    pub fn new(data_id: QuotedString, data: SessionData) -> Self {
        ExtXSessionData {
            data_id,
            data,
            language: None,
        }
    }

    /// Makes a new `ExtXSessionData` with the given language.
    pub fn with_language(data_id: QuotedString, data: SessionData, language: QuotedString) -> Self {
        ExtXSessionData {
            data_id,
            data,
            language: Some(language),
        }
    }

    /// Returns the identifier of the data.
    pub fn data_id(&self) -> &QuotedString {
        &self.data_id
    }

    /// Returns the session data.
    pub fn data(&self) -> &SessionData {
        &self.data
    }

    /// Returns the language of the data.
    pub fn language(&self) -> Option<&QuotedString> {
        self.language.as_ref()
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXSessionData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "DATA-ID={}", self.data_id)?;
        match self.data {
            SessionData::Value(ref x) => write!(f, ",VALUE={}", x)?,
            SessionData::Uri(ref x) => write!(f, ",URI={}", x)?,
        }
        if let Some(ref x) = self.language {
            write!(f, ",LANGUAGE={}", x)?;
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
                "DATA-ID" => data_id = Some(track!(value.parse())?),
                "VALUE" => session_value = Some(track!(value.parse())?),
                "URI" => uri = Some(track!(value.parse())?),
                "LANGUAGE" => language = Some(track!(value.parse())?),
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
        let tag = ExtXSessionData::new(
            quoted_string("foo"),
            SessionData::Value(quoted_string("bar")),
        );
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",VALUE="bar""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);

        let tag =
            ExtXSessionData::new(quoted_string("foo"), SessionData::Uri(quoted_string("bar")));
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",URI="bar""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);

        let tag = ExtXSessionData::with_language(
            quoted_string("foo"),
            SessionData::Value(quoted_string("bar")),
            quoted_string("baz"),
        );
        let text = r#"#EXT-X-SESSION-DATA:DATA-ID="foo",VALUE="bar",LANGUAGE="baz""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }

    fn quoted_string(s: &str) -> QuotedString {
        QuotedString::new(s).unwrap()
    }
}
