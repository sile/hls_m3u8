use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::{self, FromStr};
use trackable::error::ErrorKindExt;

/// Byte range.
///
/// See: [4.3.2.2. EXT-X-BYTERANGE]
///
/// [4.3.2.2. EXT-X-BYTERANGE]: https://tools.ietf.org/html/rfc8216#section-4.3.2.2
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ByteRange {
    pub length: usize,
    pub start: Option<usize>,
}

impl fmt::Display for ByteRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.length)?;
        if let Some(x) = self.start {
            write!(f, "@{}", x)?;
        }
        Ok(())
    }
}

impl FromStr for ByteRange {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut tokens = s.splitn(2, '@');
        let length = tokens.next().expect("Never fails");
        let start = if let Some(start) = tokens.next() {
            Some(track!(start
                .parse()
                .map_err(|e| ErrorKind::InvalidInput.cause(e)))?)
        } else {
            None
        };
        Ok(ByteRange {
            length: track!(length.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?,
            start,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let byte_range = ByteRange {
            length: 0,
            start: Some(5),
        };
        assert_eq!(byte_range.to_string(), "0@5".to_string());

        let byte_range = ByteRange {
            length: 99999,
            start: Some(2),
        };
        assert_eq!(byte_range.to_string(), "99999@2".to_string());

        let byte_range = ByteRange {
            length: 99999,
            start: None,
        };
        assert_eq!(byte_range.to_string(), "99999".to_string());
    }

    #[test]
    fn test_parse() {
        let byte_range = ByteRange {
            length: 99999,
            start: Some(2),
        };
        assert_eq!(byte_range, "99999@2".parse::<ByteRange>().unwrap());

        let byte_range = ByteRange {
            length: 99999,
            start: Some(2),
        };
        assert_eq!(byte_range, "99999@2".parse::<ByteRange>().unwrap());

        let byte_range = ByteRange {
            length: 99999,
            start: None,
        };
        assert_eq!(byte_range, "99999".parse::<ByteRange>().unwrap());
    }
}
