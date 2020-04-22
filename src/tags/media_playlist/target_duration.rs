use std::convert::TryFrom;
use std::fmt;
use std::time::Duration;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// Specifies the maximum `MediaSegment` duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub(crate) struct ExtXTargetDuration(pub Duration);

impl ExtXTargetDuration {
    pub(crate) const PREFIX: &'static str = "#EXT-X-TARGETDURATION:";
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXTargetDuration {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXTargetDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0.as_secs())
    }
}

impl TryFrom<&str> for ExtXTargetDuration {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input = tag(input, Self::PREFIX)?
            .parse()
            .map_err(|e| Error::parse_int(input, e))?;

        Ok(Self(Duration::from_secs(input)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXTargetDuration(Duration::from_secs(5)).to_string(),
            "#EXT-X-TARGETDURATION:5".to_string()
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXTargetDuration(Duration::from_secs(5)).required_version(),
            ProtocolVersion::V1
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXTargetDuration(Duration::from_secs(5)),
            ExtXTargetDuration::try_from("#EXT-X-TARGETDURATION:5").unwrap()
        );
    }
}
