use std::convert::TryFrom;
use std::fmt;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// The [`ExtM3u`] tag indicates that the file is an **Ext**ended **[`M3U`]**
/// Playlist file.
/// It is the at the start of every [`MediaPlaylist`] and [`MasterPlaylist`].
///
/// [`MediaPlaylist`]: crate::MediaPlaylist
/// [`MasterPlaylist`]: crate::MasterPlaylist
/// [`M3U`]: https://en.wikipedia.org/wiki/M3U
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(crate) struct ExtM3u;

impl ExtM3u {
    pub(crate) const PREFIX: &'static str = "#EXTM3U";
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtM3u {
    fn required_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtM3u {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)
    }
}

impl TryFrom<&str> for ExtM3u {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        tag(input, Self::PREFIX)?;
        Ok(Self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(ExtM3u.to_string(), "#EXTM3U".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!(ExtM3u::try_from("#EXTM3U").unwrap(), ExtM3u);
        assert!(ExtM3u::try_from("#EXTM2U").is_err());
    }

    #[test]
    fn test_required_version() {
        assert_eq!(ExtM3u.required_version(), ProtocolVersion::V1);
    }
}
