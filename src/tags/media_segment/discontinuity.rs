use crate::types::ProtocolVersion;
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;

/// [4.3.2.3. EXT-X-DISCONTINUITY]
///
/// [4.3.2.3. EXT-X-DISCONTINUITY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXDiscontinuity;
impl ExtXDiscontinuity {
    pub(crate) const PREFIX: &'static str = "#EXT-X-DISCONTINUITY";

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(self) -> ProtocolVersion {
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
    fn from_str(s: &str) -> Result<Self> {
        track_assert_eq!(s, Self::PREFIX, ErrorKind::InvalidInput);
        Ok(ExtXDiscontinuity)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_discontinuity() {
        let tag = ExtXDiscontinuity;
        assert_eq!("#EXT-X-DISCONTINUITY".parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), "#EXT-X-DISCONTINUITY");
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }
}
