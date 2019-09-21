use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use crate::types::{ByteRange, ProtocolVersion};
use crate::utils::tag;
use crate::Error;

/// [4.3.2.2. EXT-X-BYTERANGE]
///
/// [4.3.2.2. EXT-X-BYTERANGE]: https://tools.ietf.org/html/rfc8216#section-4.3.2.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXByteRange(ByteRange);

impl ExtXByteRange {
    pub(crate) const PREFIX: &'static str = "#EXT-X-BYTERANGE:";

    /// Makes a new `ExtXByteRange` tag.
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXByteRange;
    ///
    /// let byte_range = ExtXByteRange::new(20, Some(5));
    /// ```
    pub const fn new(length: usize, start: Option<usize>) -> Self {
        Self(ByteRange::new(length, start))
    }

    /// Converts the [ExtXByteRange] to a [ByteRange].
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXByteRange;
    /// use hls_m3u8::types::ByteRange;
    ///
    /// let byte_range = ExtXByteRange::new(20, Some(5));
    /// let range: ByteRange = byte_range.to_range();
    /// ```
    pub const fn to_range(&self) -> ByteRange {
        self.0
    }

    /// Returns the protocol compatibility version that this tag requires.
    /// # Example
    /// ```
    /// use hls_m3u8::tags::ExtXByteRange;
    /// use hls_m3u8::types::ProtocolVersion;
    ///
    /// let byte_range = ExtXByteRange::new(20, Some(5));
    /// assert_eq!(byte_range.requires_version(), ProtocolVersion::V4);
    /// ```
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V4
    }
}

impl Deref for ExtXByteRange {
    type Target = ByteRange;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ExtXByteRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl FromStr for ExtXByteRange {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        let tokens = input.splitn(2, '@').collect::<Vec<_>>();
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

        Ok(ExtXByteRange::new(length, start))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        let byte_range = ExtXByteRange::new(0, Some(5));
        assert_eq!(byte_range.to_string(), "#EXT-X-BYTERANGE:0@5".to_string());

        let byte_range = ExtXByteRange::new(99999, Some(2));
        assert_eq!(
            byte_range.to_string(),
            "#EXT-X-BYTERANGE:99999@2".to_string()
        );

        let byte_range = ExtXByteRange::new(99999, None);
        assert_eq!(byte_range.to_string(), "#EXT-X-BYTERANGE:99999".to_string());
    }

    #[test]
    fn test_parser() {
        let byte_range = ExtXByteRange::new(99999, Some(2));
        assert_eq!(
            byte_range,
            "#EXT-X-BYTERANGE:99999@2".parse::<ExtXByteRange>().unwrap()
        );

        let byte_range = ExtXByteRange::new(99999, None);
        assert_eq!(
            byte_range,
            "#EXT-X-BYTERANGE:99999".parse::<ExtXByteRange>().unwrap()
        );
    }

    #[test]
    fn test_deref() {
        let byte_range = ExtXByteRange::new(0, Some(22));

        assert_eq!(byte_range.length(), 0);
        assert_eq!(byte_range.start(), Some(22));
    }
}
