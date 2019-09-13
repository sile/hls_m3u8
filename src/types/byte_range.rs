use std::fmt;
use std::str::FromStr;

use getset::{Getters, MutGetters, Setters};

use crate::Error;

/// Byte range.
///
/// See: [4.3.2.2. EXT-X-BYTERANGE]
///
/// [4.3.2.2. EXT-X-BYTERANGE]: https://tools.ietf.org/html/rfc8216#section-4.3.2.2
#[derive(Getters, Setters, MutGetters, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[get = "pub"]
#[set = "pub"]
#[get_mut = "pub"]
pub struct ByteRange {
    /// The length of the range.
    length: usize,
    /// The start of the range.
    start: Option<usize>,
}

impl ByteRange {
    /// Creates a new [ByteRange].
    pub const fn new(length: usize, start: Option<usize>) -> Self {
        Self { length, start }
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
        let tokens = s.splitn(2, '@').collect::<Vec<_>>();
        if tokens.is_empty() {
            return Err(Error::invalid_input());
        }

        let length = tokens[0].parse()?;

        let start = {
            let mut result = None;
            if tokens.len() == 2 {
                result = Some(tokens[1].parse()?);
            }
            result
        };
        Ok(ByteRange::new(length, start))
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
