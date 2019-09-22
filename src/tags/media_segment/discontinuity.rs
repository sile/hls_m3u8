use std::fmt;
use std::str::FromStr;

use crate::types::{ProtocolVersion, RequiredVersion};
use crate::utils::tag;
use crate::Error;

/// [4.3.2.3. EXT-X-DISCONTINUITY]
///
/// [4.3.2.3. EXT-X-DISCONTINUITY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXDiscontinuity;

impl ExtXDiscontinuity {
    pub(crate) const PREFIX: &'static str = "#EXT-X-DISCONTINUITY";
}

impl RequiredVersion for ExtXDiscontinuity {
    fn required_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXDiscontinuity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Self::PREFIX.fmt(f)
    }
}

impl FromStr for ExtXDiscontinuity {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        tag(input, Self::PREFIX)?;
        Ok(ExtXDiscontinuity)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXDiscontinuity.to_string(),
            "#EXT-X-DISCONTINUITY".to_string(),
        )
    }

    #[test]
    fn test_parser() {
        assert_eq!(ExtXDiscontinuity, "#EXT-X-DISCONTINUITY".parse().unwrap(),)
    }

    #[test]
    fn test_required_version() {
        assert_eq!(ExtXDiscontinuity.required_version(), ProtocolVersion::V1)
    }
}
