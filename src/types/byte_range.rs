use std::fmt;
use std::str::FromStr;

use trackable::error::ErrorKindExt;

use crate::error::{Error, ErrorKind};

/// Byte range.
///
/// See: [4.3.2.2. EXT-X-BYTERANGE]
///
/// [4.3.2.2. EXT-X-BYTERANGE]: https://tools.ietf.org/html/rfc8216#section-4.3.2.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ByteRange {
    length: usize,
    start: Option<usize>,
}

impl ByteRange {
    /// Create a new [ByteRange].
    pub const fn new(length: usize, start: Option<usize>) -> Self {
        Self { length, start }
    }

    /// Returns the length of the [ByteRange].
    pub const fn length(&self) -> usize {
        self.length
    }

    /// Returns the start of the [ByteRange], if there is any.
    pub const fn start(&self) -> Option<usize> {
        self.start
    }
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.splitn(2, '@');
        let length = tokens.next().ok_or(ErrorKind::InvalidInput)?;

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
        assert_eq!(ByteRange::new(5, Some(20)).to_string(), "5@20");
        assert_eq!(ByteRange::new(5, None).to_string(), "5");
    }

    #[test]
    fn test_parser() {
        assert_eq!("45".parse::<ByteRange>().unwrap(), ByteRange::new(45, None));
        assert_eq!(
            "108@16".parse::<ByteRange>().unwrap(),
            ByteRange::new(108, Some(16))
        );
    }

    #[test]
    fn test_parser_err() {
        assert!("45E".parse::<ByteRange>().is_err());
        assert!("45E@1".parse::<ByteRange>().is_err());
        assert!("45E@23E".parse::<ByteRange>().is_err());
        assert!("45@23E".parse::<ByteRange>().is_err());
    }
}
