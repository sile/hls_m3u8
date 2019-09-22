use std::fmt;
use std::str::FromStr;

use crate::types::{ProtocolVersion, RequiredVersion};
use crate::utils::tag;
use crate::Error;

/// # [4.3.1.1. EXTM3U]
/// The [ExtM3u] tag indicates that the file is an Extended [M3U]
/// Playlist file.
///
/// Its format is:
/// ```text
/// #EXTM3U
/// ```
///
/// [M3U]: https://en.wikipedia.org/wiki/M3U
/// [4.3.1.1. EXTM3U]: https://tools.ietf.org/html/rfc8216#section-4.3.1.1
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ExtM3u;

impl ExtM3u {
    pub(crate) const PREFIX: &'static str = "#EXTM3U";
}

impl RequiredVersion for ExtM3u {
    fn required_version(&self) -> ProtocolVersion {
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

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        tag(input, Self::PREFIX)?;
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
    fn test_required_version() {
        assert_eq!(ExtM3u.required_version(), ProtocolVersion::V1);
    }
}
