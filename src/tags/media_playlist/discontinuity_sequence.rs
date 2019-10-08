use std::fmt;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::RequiredVersion;

/// # [4.4.3.3. EXT-X-DISCONTINUITY-SEQUENCE]
///
/// The [`ExtXDiscontinuitySequence`] tag allows synchronization between
/// different Renditions of the same Variant Stream or different Variant
/// Streams that have [`ExtXDiscontinuity`] tags in their [`Media Playlist`]s.
///
/// Its format is:
/// ```text
/// #EXT-X-DISCONTINUITY-SEQUENCE:<number>
/// ```
/// where `number` is a [u64].
///
/// [`ExtXDiscontinuity`]: crate::tags::ExtXDiscontinuity
/// [`Media Playlist`]: crate::MediaPlaylist
/// [4.4.3.3. EXT-X-DISCONTINUITY-SEQUENCE]:
/// https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-04#section-4.4.3.3
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ExtXDiscontinuitySequence(u64);

impl ExtXDiscontinuitySequence {
    pub(crate) const PREFIX: &'static str = "#EXT-X-DISCONTINUITY-SEQUENCE:";

    /// Makes a new [ExtXDiscontinuitySequence] tag.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDiscontinuitySequence;
    /// let discontinuity_sequence = ExtXDiscontinuitySequence::new(5);
    /// ```
    pub const fn new(seq_num: u64) -> Self { Self(seq_num) }

    /// Returns the discontinuity sequence number of
    /// the first media segment that appears in the associated playlist.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDiscontinuitySequence;
    /// let discontinuity_sequence = ExtXDiscontinuitySequence::new(5);
    ///
    /// assert_eq!(discontinuity_sequence.seq_num(), 5);
    /// ```
    pub const fn seq_num(self) -> u64 { self.0 }

    /// Sets the sequence number.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDiscontinuitySequence;
    /// let mut discontinuity_sequence = ExtXDiscontinuitySequence::new(5);
    ///
    /// discontinuity_sequence.set_seq_num(10);
    /// assert_eq!(discontinuity_sequence.seq_num(), 10);
    /// ```
    pub fn set_seq_num(&mut self, value: u64) -> &mut Self {
        self.0 = value;
        self
    }
}

impl RequiredVersion for ExtXDiscontinuitySequence {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXDiscontinuitySequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}{}", Self::PREFIX, self.0) }
}

impl FromStr for ExtXDiscontinuitySequence {
    type Err = crate::Error;

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
            ExtXDiscontinuitySequence::new(123).to_string(),
            "#EXT-X-DISCONTINUITY-SEQUENCE:123".to_string()
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXDiscontinuitySequence::new(123).required_version(),
            ProtocolVersion::V1
        )
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXDiscontinuitySequence::new(123),
            "#EXT-X-DISCONTINUITY-SEQUENCE:123".parse().unwrap()
        );
    }

    #[test]
    fn test_seq_num() {
        let mut sequence = ExtXDiscontinuitySequence::new(123);
        assert_eq!(sequence.seq_num(), 123);
        sequence.set_seq_num(1);
        assert_eq!(sequence.seq_num(), 1);
    }
}
