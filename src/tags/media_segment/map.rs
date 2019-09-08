use crate::attribute::AttributePairs;
use crate::types::{ByteRange, ProtocolVersion};
use crate::utils::{quote, unquote};
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;

/// [4.3.2.5. EXT-X-MAP]
///
/// [4.3.2.5. EXT-X-MAP]: https://tools.ietf.org/html/rfc8216#section-4.3.2.5
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXMap {
    uri: String,
    range: Option<ByteRange>,
}

impl ExtXMap {
    pub(crate) const PREFIX: &'static str = "#EXT-X-MAP:";

    /// Makes a new `ExtXMap` tag.
    pub fn new<T: ToString>(uri: T) -> Self {
        ExtXMap {
            uri: uri.to_string(),
            range: None,
        }
    }

    /// Makes a new `ExtXMap` tag with the given range.
    pub fn with_range<T: ToString>(uri: T, range: ByteRange) -> Self {
        ExtXMap {
            uri: uri.to_string(),
            range: Some(range),
        }
    }

    /// Returns the URI that identifies a resource that contains the media initialization section.
    pub const fn uri(&self) -> &String {
        &self.uri
    }

    /// Returns the range of the media initialization section.
    pub const fn range(&self) -> Option<ByteRange> {
        self.range
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V6
    }
}

impl fmt::Display for ExtXMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "URI={}", quote(&self.uri))?;
        if let Some(ref x) = self.range {
            write!(f, ",BYTERANGE={}", quote(x))?;
        }
        Ok(())
    }
}

impl FromStr for ExtXMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut uri = None;
        let mut range = None;
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "URI" => uri = Some(unquote(value)),
                "BYTERANGE" => {
                    range = Some(track!(unquote(value).parse())?);
                }
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let uri = track_assert_some!(uri, ErrorKind::InvalidInput);
        Ok(ExtXMap { uri, range })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_map() {
        let tag = ExtXMap::new("foo");
        let text = r#"#EXT-X-MAP:URI="foo""#;
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V6);

        let tag = ExtXMap::with_range(
            "foo",
            ByteRange {
                length: 9,
                start: Some(2),
            },
        );
        let text = r#"#EXT-X-MAP:URI="foo",BYTERANGE="9@2""#;
        track_try_unwrap!(ExtXMap::from_str(text));
        assert_eq!(text.parse().ok(), Some(tag.clone()));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V6);
    }
}
