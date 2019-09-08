use crate::types::ProtocolVersion;
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;

/// [4.3.1.1. EXTM3U]
///
/// [4.3.1.1. EXTM3U]: https://tools.ietf.org/html/rfc8216#section-4.3.1.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtM3u;

impl ExtM3u {
    pub(crate) const PREFIX: &'static str = "#EXTM3U";

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtM3u {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Self::PREFIX.fmt(f)
    }
}

impl FromStr for ExtM3u {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        track_assert_eq!(s, Self::PREFIX, ErrorKind::InvalidInput);
        Ok(ExtM3u)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(ExtM3u.to_string(), "#EXTM3U".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!("#EXTM3U".parse::<ExtM3u>().ok(), Some(ExtM3u));
    }

    #[test]
    fn test_requires_version() {
        assert_eq!(ExtM3u.requires_version(), ProtocolVersion::V1);
    }
}
