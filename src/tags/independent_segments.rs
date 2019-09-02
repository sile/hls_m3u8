use std::fmt;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::{Error, ErrorKind};

/// [4.3.5.1. EXT-X-INDEPENDENT-SEGMENTS]
///
/// [4.3.5.1. EXT-X-INDEPENDENT-SEGMENTS]: https://tools.ietf.org/html/rfc8216#section-4.3.5.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXIndependentSegments;

impl ExtXIndependentSegments {
    pub(crate) const PREFIX: &'static str = "#EXT-X-INDEPENDENT-SEGMENTS";

    /// Returns the protocol compatibility version that this tag requires.
    pub fn required_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXIndependentSegments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)
    }
}
impl FromStr for ExtXIndependentSegments {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        track_assert_eq!(s, Self::PREFIX, ErrorKind::InvalidInput);
        Ok(ExtXIndependentSegments)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_independent_segments() {
        let tag = ExtXIndependentSegments;
        let text = "#EXT-X-INDEPENDENT-SEGMENTS";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.required_version(), ProtocolVersion::V1);
    }
}
