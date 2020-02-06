use std::fmt;
use std::str::FromStr;

use derive_more::{Deref, DerefMut};

use crate::types::{ByteRange, ProtocolVersion};
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// # [4.4.2.2. EXT-X-BYTERANGE]
///
/// The [`ExtXByteRange`] tag indicates that a [`Media Segment`] is a sub-range
/// of the resource identified by its `URI`.
///
/// [`Media Segment`]: crate::MediaSegment
/// [4.4.2.2. EXT-X-BYTERANGE]:
/// https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-04#section-4.4.2.2
#[derive(Deref, DerefMut, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtXByteRange(ByteRange);

impl ExtXByteRange {
    pub(crate) const PREFIX: &'static str = "#EXT-X-BYTERANGE:";

    /// Makes a new [`ExtXByteRange`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXByteRange;
    /// let byte_range = ExtXByteRange::new(20, Some(5));
    /// ```
    pub const fn new(length: usize, start: Option<usize>) -> Self {
        Self(ByteRange::new(length, start))
    }

    /// Converts the [`ExtXByteRange`] to a [`ByteRange`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXByteRange;
    /// use hls_m3u8::types::ByteRange;
    ///
    /// let byte_range = ExtXByteRange::new(20, Some(5));
    /// let range: ByteRange = byte_range.to_range();
    /// ```
    pub const fn to_range(&self) -> ByteRange { self.0 }
}

/// This tag requires [`ProtocolVersion::V4`].
impl RequiredVersion for ExtXByteRange {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V4 }
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

        Ok(Self(ByteRange::from_str(input)?))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

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

    #[test]
    fn test_deref_mut() {
        let mut byte_range = ExtXByteRange::new(0, Some(22));

        byte_range.set_length(100);
        byte_range.set_start(Some(50));

        assert_eq!(byte_range.length(), 100);
        assert_eq!(byte_range.start(), Some(50));
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXByteRange::new(20, Some(5)).required_version(),
            ProtocolVersion::V4
        );
    }
}
