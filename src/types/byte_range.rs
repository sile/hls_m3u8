use std::fmt;
use std::str::FromStr;

use crate::Error;

/// Byte range.
///
/// See: [4.3.2.2. EXT-X-BYTERANGE]
///
/// [4.3.2.2. EXT-X-BYTERANGE]: https://tools.ietf.org/html/rfc8216#section-4.3.2.2
#[derive(Copy, Hash, Eq, Ord, Debug, PartialEq, Clone, PartialOrd)]
pub struct ByteRange {
    length: usize,
    start: Option<usize>,
}

impl ByteRange {
    /// Creates a new [ByteRange].
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// ByteRange::new(22, Some(12));
    /// ```
    pub const fn new(length: usize, start: Option<usize>) -> Self {
        Self { length, start }
    }

    /// Returns the length of the range.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// #
    /// assert_eq!(ByteRange::new(20, Some(3)).length(), 20);
    /// ```
    pub const fn length(&self) -> usize {
        self.length
    }

    /// Sets the length of the range.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// #
    /// let mut range = ByteRange::new(20, Some(3));
    ///
    /// # assert_eq!(range.length(), 20);
    /// range.set_length(10);
    /// assert_eq!(range.length(), 10);
    /// ```
    pub fn set_length(&mut self, value: usize) -> &mut Self {
        self.length = value;
        self
    }

    /// Returns the start of the range.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// #
    /// assert_eq!(ByteRange::new(20, Some(3)).start(), Some(3));
    /// ```
    pub const fn start(&self) -> Option<usize> {
        self.start
    }

    /// Sets the start of the range.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// #
    /// let mut range = ByteRange::new(20, None);
    ///
    /// # assert_eq!(range.start(), None);
    /// range.set_start(Some(3));
    /// assert_eq!(range.start(), Some(3));
    /// ```
    pub fn set_start(&mut self, value: Option<usize>) -> &mut Self {
        self.start = value;
        self
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
            if tokens.len() == 2 {
                Some(tokens[1].parse()?)
            } else {
                None
            }
        };
        Ok(Self::new(length, start))
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
    fn test_parser() {
        assert_eq!(
            ByteRange {
                length: 99999,
                start: Some(2),
            },
            "99999@2".parse::<ByteRange>().unwrap()
        );

        assert_eq!(
            ByteRange {
                length: 99999,
                start: Some(2),
            },
            "99999@2".parse::<ByteRange>().unwrap()
        );

        assert_eq!(
            ByteRange {
                length: 99999,
                start: None,
            },
            "99999".parse::<ByteRange>().unwrap()
        );

        assert!("".parse::<ByteRange>().is_err());
    }
}
