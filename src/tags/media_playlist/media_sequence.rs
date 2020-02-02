use std::fmt;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// # [4.4.3.2. EXT-X-MEDIA-SEQUENCE]
///
/// The [`ExtXMediaSequence`] tag indicates the Media Sequence Number of
/// the first [`Media Segment`] that appears in a Playlist file.
///
/// [Media Segment]: crate::MediaSegment
/// [4.4.3.2. EXT-X-MEDIA-SEQUENCE]:
/// https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-04#section-4.4.3.2
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ExtXMediaSequence(u64);

impl ExtXMediaSequence {
    pub(crate) const PREFIX: &'static str = "#EXT-X-MEDIA-SEQUENCE:";

    /// Makes a new [`ExtXMediaSequence`] tag.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXMediaSequence;
    /// let media_sequence = ExtXMediaSequence::new(5);
    /// ```
    pub const fn new(seq_num: u64) -> Self { Self(seq_num) }

    /// Returns the sequence number of the first media segment,
    /// that appears in the associated playlist.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXMediaSequence;
    /// let media_sequence = ExtXMediaSequence::new(5);
    ///
    /// assert_eq!(media_sequence.seq_num(), 5);
    /// ```
    pub const fn seq_num(self) -> u64 { self.0 }

    /// Sets the sequence number.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::tags::ExtXMediaSequence;
    /// let mut media_sequence = ExtXMediaSequence::new(5);
    ///
    /// media_sequence.set_seq_num(10);
    /// assert_eq!(media_sequence.seq_num(), 10);
    /// ```
    pub fn set_seq_num(&mut self, value: u64) -> &mut Self {
        self.0 = value;
        self
    }
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXMediaSequence {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXMediaSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}{}", Self::PREFIX, self.0) }
}

impl FromStr for ExtXMediaSequence {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let seq_num = tag(input, Self::PREFIX)?.parse()?;
        Ok(Self::new(seq_num))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXMediaSequence::new(123).to_string(),
            "#EXT-X-MEDIA-SEQUENCE:123".to_string()
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXMediaSequence::new(123).required_version(),
            ProtocolVersion::V1
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXMediaSequence::new(123),
            "#EXT-X-MEDIA-SEQUENCE:123".parse().unwrap()
        );
    }

    #[test]
    fn test_seq_num() {
        let mut sequence = ExtXMediaSequence::new(123);
        assert_eq!(sequence.seq_num(), 123);
        sequence.set_seq_num(1);
        assert_eq!(sequence.seq_num(), 1);
    }
}
