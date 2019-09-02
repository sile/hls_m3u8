use std::fmt;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::{Error, ErrorKind};

/// [4.3.1.2. EXT-X-VERSION]
///
/// [4.3.1.2. EXT-X-VERSION]: https://tools.ietf.org/html/rfc8216#section-4.3.1.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXVersion(ProtocolVersion);

impl ExtXVersion {
    pub(crate) const PREFIX: &'static str = "#EXT-X-VERSION:";

    /// Makes a new `ExtXVersion` tag.
    pub const fn new(version: ProtocolVersion) -> Self {
        Self(version)
    }

    /// Returns the protocol compatibility version of the playlist containing this tag.
    pub const fn version(&self) -> ProtocolVersion {
        self.0
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn required_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl FromStr for ExtXVersion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let suffix = s.split_at(Self::PREFIX.len()).1;
        let version = track!(suffix.parse())?;
        Ok(Self(version))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            "#EXT-X-VERSION:6".parse().ok(),
            Some(ExtXVersion::new(ProtocolVersion::V6))
        );
    }

    #[test]
    fn test_parser_err() {
        assert!("#EXTX-X-VERSION:6".parse::<ExtXVersion>().is_err());
        assert!("#EXT-X-VERSION:T".parse::<ExtXVersion>().is_err());
        assert!("#EXT-X-VERSION:".parse::<ExtXVersion>().is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXVersion::new(ProtocolVersion::V6).to_string(),
            "#EXT-X-VERSION:6"
        );
        assert_eq!(
            ExtXVersion::new(ProtocolVersion::V4).to_string(),
            "#EXT-X-VERSION:4"
        );
    }

    #[test]
    fn test_required_vesion() {
        assert_eq!(
            ExtXVersion::new(ProtocolVersion::V6).required_version(),
            ProtocolVersion::V1
        );
    }
}
