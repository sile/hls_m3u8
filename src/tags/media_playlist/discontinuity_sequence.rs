use std::convert::TryFrom;
use std::fmt;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// Allows synchronization between different renditions of the same
/// [`VariantStream`].
///
/// [`VariantStream`]: crate::tags::VariantStream
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub(crate) struct ExtXDiscontinuitySequence(pub usize);

impl ExtXDiscontinuitySequence {
    pub(crate) const PREFIX: &'static str = "#EXT-X-DISCONTINUITY-SEQUENCE:";
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXDiscontinuitySequence {
    fn required_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXDiscontinuitySequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl TryFrom<&str> for ExtXDiscontinuitySequence {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input = tag(input, Self::PREFIX)?;
        let seq_num = input.parse().map_err(|e| Error::parse_int(input, e))?;

        Ok(Self(seq_num))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXDiscontinuitySequence(123).to_string(),
            "#EXT-X-DISCONTINUITY-SEQUENCE:123".to_string()
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXDiscontinuitySequence(123).required_version(),
            ProtocolVersion::V1
        )
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXDiscontinuitySequence(123),
            ExtXDiscontinuitySequence::try_from("#EXT-X-DISCONTINUITY-SEQUENCE:123").unwrap()
        );

        assert_eq!(
            ExtXDiscontinuitySequence::try_from("#EXT-X-DISCONTINUITY-SEQUENCE:12A"),
            Err(Error::parse_int("12A", "12A".parse::<u64>().expect_err("")))
        );
    }
}
