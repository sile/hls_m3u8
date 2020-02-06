use std::fmt;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// # [4.3.1.1. EXTM3U]
///
/// The [`ExtM3u`] tag indicates that the file is an **Ext**ended **[`M3U`]**
/// Playlist file.
/// It is the at the start of every [`Media Playlist`] and [`Master Playlist`].
///
/// [`Media Playlist`]: crate::MediaPlaylist
/// [`Master Playlist`]: crate::MasterPlaylist
/// [`M3U`]: https://en.wikipedia.org/wiki/M3U
/// [4.3.1.1. EXTM3U]: https://tools.ietf.org/html/rfc8216#section-4.3.1.1
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(crate) struct ExtM3u;

impl ExtM3u {
    pub(crate) const PREFIX: &'static str = "#EXTM3U";
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtM3u {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtM3u {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", Self::PREFIX) }
}

impl FromStr for ExtM3u {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
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
        assert_eq!("#EXTM3U".parse::<ExtM3u>().unwrap(), ExtM3u);
        assert!("#EXTM2U".parse::<ExtM3u>().is_err());
    }

    #[test]
    fn test_required_version() {
        assert_eq!(ExtM3u.required_version(), ProtocolVersion::V1);
    }
}
