use std::fmt;
use std::str::FromStr;

use crate::types::{ProtocolVersion, RequiredVersion};
use crate::utils::tag;

/// [4.3.3.3. EXT-X-DISCONTINUITY-SEQUENCE]
///
/// [4.3.3.3. EXT-X-DISCONTINUITY-SEQUENCE]: https://tools.ietf.org/html/rfc8216#section-4.3.3.3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ExtXDiscontinuitySequence(u64);

impl ExtXDiscontinuitySequence {
    pub(crate) const PREFIX: &'static str = "#EXT-X-DISCONTINUITY-SEQUENCE:";

    /// Makes a new `ExtXDiscontinuitySequence` tag.
    pub const fn new(seq_num: u64) -> Self {
        Self(seq_num)
    }

    /// Returns the discontinuity sequence number of
    /// the first media segment that appears in the associated playlist.
    pub const fn seq_num(&self) -> u64 {
        self.0
    }
}

impl RequiredVersion for ExtXDiscontinuitySequence {
    fn required_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXDiscontinuitySequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
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
}
