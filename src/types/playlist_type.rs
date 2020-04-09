use std::fmt;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// Provides mutability information about the [`MediaPlaylist`].
///
/// It applies to the entire [`MediaPlaylist`].
///
/// [`MediaPlaylist`]: crate::MediaPlaylist
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlaylistType {
    /// If the [`PlaylistType`] is Event, [`MediaSegment`]s
    /// can only be added to the end of the [`MediaPlaylist`].
    ///
    /// [`MediaSegment`]: crate::MediaSegment
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    Event,
    /// If the [`PlaylistType`] is Video On Demand (Vod),
    /// the [`MediaPlaylist`] cannot change.
    ///
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    Vod,
}

impl PlaylistType {
    pub(crate) const PREFIX: &'static str = "#EXT-X-PLAYLIST-TYPE:";
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for PlaylistType {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for PlaylistType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Event => write!(f, "{}EVENT", Self::PREFIX),
            Self::Vod => write!(f, "{}VOD", Self::PREFIX),
        }
    }
}

impl FromStr for PlaylistType {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;
        match input {
            "EVENT" => Ok(Self::Event),
            "VOD" => Ok(Self::Vod),
            _ => Err(Error::custom(format!("unknown playlist type: {:?}", input))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parser() {
        assert_eq!(
            "#EXT-X-PLAYLIST-TYPE:VOD".parse::<PlaylistType>().unwrap(),
            PlaylistType::Vod,
        );

        assert_eq!(
            "#EXT-X-PLAYLIST-TYPE:EVENT"
                .parse::<PlaylistType>()
                .unwrap(),
            PlaylistType::Event,
        );

        assert!("#EXT-X-PLAYLIST-TYPE:H".parse::<PlaylistType>().is_err());

        assert!("garbage".parse::<PlaylistType>().is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(
            "#EXT-X-PLAYLIST-TYPE:VOD".to_string(),
            PlaylistType::Vod.to_string(),
        );

        assert_eq!(
            "#EXT-X-PLAYLIST-TYPE:EVENT".to_string(),
            PlaylistType::Event.to_string(),
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(PlaylistType::Vod.required_version(), ProtocolVersion::V1);
        assert_eq!(PlaylistType::Event.required_version(), ProtocolVersion::V1);
    }
}
