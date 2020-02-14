use std::fmt;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// # [4.3.3.5. EXT-X-PLAYLIST-TYPE]
///
/// The [`ExtXPlaylistType`] tag provides mutability information about the
/// [`MediaPlaylist`]. It applies to the entire [`MediaPlaylist`].
///
/// [`MediaPlaylist`]: crate::MediaPlaylist
/// [4.3.3.5. EXT-X-PLAYLIST-TYPE]: https://tools.ietf.org/html/rfc8216#section-4.3.3.5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExtXPlaylistType {
    /// If the [`ExtXPlaylistType`] is Event, [`MediaSegment`]s
    /// can only be added to the end of the [`MediaPlaylist`].
    ///
    /// [`MediaSegment`]: crate::MediaSegment
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    Event,
    /// If the [`ExtXPlaylistType`] is Video On Demand (Vod),
    /// the [`MediaPlaylist`] cannot change.
    ///
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    Vod,
}

impl ExtXPlaylistType {
    pub(crate) const PREFIX: &'static str = "#EXT-X-PLAYLIST-TYPE:";
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXPlaylistType {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXPlaylistType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Event => write!(f, "{}EVENT", Self::PREFIX),
            Self::Vod => write!(f, "{}VOD", Self::PREFIX),
        }
    }
}

impl FromStr for ExtXPlaylistType {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;
        match input {
            "EVENT" => Ok(Self::Event),
            "VOD" => Ok(Self::Vod),
            _ => Err(Error::custom(format!("Unknown playlist type: {:?}", input))),
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
            "#EXT-X-PLAYLIST-TYPE:VOD"
                .parse::<ExtXPlaylistType>()
                .unwrap(),
            ExtXPlaylistType::Vod,
        );

        assert_eq!(
            "#EXT-X-PLAYLIST-TYPE:EVENT"
                .parse::<ExtXPlaylistType>()
                .unwrap(),
            ExtXPlaylistType::Event,
        );

        assert!("#EXT-X-PLAYLIST-TYPE:H"
            .parse::<ExtXPlaylistType>()
            .is_err());

        assert!("garbage".parse::<ExtXPlaylistType>().is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(
            "#EXT-X-PLAYLIST-TYPE:VOD".to_string(),
            ExtXPlaylistType::Vod.to_string(),
        );

        assert_eq!(
            "#EXT-X-PLAYLIST-TYPE:EVENT".to_string(),
            ExtXPlaylistType::Event.to_string(),
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXPlaylistType::Vod.required_version(),
            ProtocolVersion::V1
        );
        assert_eq!(
            ExtXPlaylistType::Event.required_version(),
            ProtocolVersion::V1
        );
    }
}
