use std::convert::TryFrom;
use std::fmt;

use crate::types::ProtocolVersion;
use crate::{Error, RequiredVersion};

/// The `ExtXDiscontinuity` tag indicates a discontinuity between the
/// `MediaSegment` that follows it and the one that preceded it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ExtXDiscontinuity;

impl ExtXDiscontinuity {
    pub(crate) const PREFIX: &'static str = "#EXT-X-DISCONTINUITY";
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXDiscontinuity {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXDiscontinuity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { Self::PREFIX.fmt(f) }
}

impl TryFrom<&str> for ExtXDiscontinuity {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        // the parser assumes that only a single line is passed as input,
        // which should be "#EXT-X-DISCONTINUITY"
        if input == Self::PREFIX {
            Ok(Self)
        } else {
            Err(Error::unexpected_data(input))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXDiscontinuity.to_string(),
            "#EXT-X-DISCONTINUITY".to_string(),
        )
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXDiscontinuity,
            ExtXDiscontinuity::try_from("#EXT-X-DISCONTINUITY").unwrap()
        );

        assert!(ExtXDiscontinuity::try_from("#EXT-X-DISCONTINUITY:0").is_err());
    }

    #[test]
    fn test_required_version() {
        assert_eq!(ExtXDiscontinuity.required_version(), ProtocolVersion::V1)
    }
}
