use std::fmt;
use std::str::FromStr;

use shorthand::ShortHand;

use crate::Error;

/// Byte range.
///
/// See: [4.3.2.2. EXT-X-BYTERANGE]
///
/// [4.3.2.2. EXT-X-BYTERANGE]: https://tools.ietf.org/html/rfc8216#section-4.3.2.2
#[derive(ShortHand, Copy, Hash, Eq, Ord, Debug, PartialEq, Clone, PartialOrd)]
#[shorthand(enable(must_use))]
pub struct ByteRange {
    /// The length of the range.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// #
    /// let mut range = ByteRange::new(20, Some(3));
    /// # assert_eq!(range.length(), 20);
    ///
    /// range.set_length(10);
    /// assert_eq!(range.length(), 10);
    /// ```
    length: usize,
    /// The start of the range.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// #
    /// let mut range = ByteRange::new(20, None);
    /// # assert_eq!(range.start(), None);
    ///
    /// range.set_start(Some(3));
    /// assert_eq!(range.start(), Some(3));
    /// ```
    //
    // this is a workaround until this issue is fixed:
    // https://github.com/Luro02/shorthand/issues/20
    #[shorthand(enable(copy), disable(option_as_ref))]
    start: Option<usize>,
}

impl ByteRange {
    /// Creates a new [`ByteRange`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::ByteRange;
    /// ByteRange::new(22, Some(12));
    /// ```
    pub const fn new(length: usize, start: Option<usize>) -> Self { Self { length, start } }
}

impl fmt::Display for ByteRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.length)?;

        if let Some(value) = self.start {
            write!(f, "@{}", value)?;
        }

        Ok(())
    }
}

impl FromStr for ByteRange {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut input = input.splitn(2, '@');

        let length = input
            .next()
            .ok_or_else(|| Error::custom("missing length for #EXT-X-BYTERANGE"))
            .and_then(|s| s.parse().map_err(Error::parse_int))?;

        let start = input.next().map(str::parse).transpose()?;

        Ok(Self::new(length, start))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            ByteRange {
                length: 0,
                start: Some(5),
            }
            .to_string(),
            "0@5".to_string()
        );

        assert_eq!(
            ByteRange {
                length: 99999,
                start: Some(2),
            }
            .to_string(),
            "99999@2".to_string()
        );

        assert_eq!(
            ByteRange {
                length: 99999,
                start: None,
            }
            .to_string(),
            "99999".to_string()
        );
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
