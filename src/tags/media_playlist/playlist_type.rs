use std::fmt;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::Error;

/// [4.3.3.5. EXT-X-PLAYLIST-TYPE](https://tools.ietf.org/html/rfc8216#section-4.3.3.5)
///
/// The EXT-X-PLAYLIST-TYPE tag provides mutability information about the
/// Media Playlist. It applies to the entire Media Playlist.
/// It is OPTIONAL. Its format is:
///
/// ```text
/// #EXT-X-PLAYLIST-TYPE:<type-enum>
/// ```
///
/// # Note
/// If the EXT-X-PLAYLIST-TYPE tag is omitted from a Media Playlist, the
/// Playlist can be updated according to the rules in Section 6.2.1 with
/// no additional restrictions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtXPlaylistType {
    /// If the ExtXPlaylistType is Event, Media Segments can only be added to
    /// the end of the Media Playlist.
    Event,
    /// If the ExtXPlaylistType is Video On Demand (Vod),
    /// the Media Playlist cannot change.
    Vod,
}

impl ExtXPlaylistType {
    pub(crate) const PREFIX: &'static str = "#EXT-X-PLAYLIST-TYPE:";

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
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
    fn test_requires_version() {
        assert_eq!(
            ExtXPlaylistType::Vod.requires_version(),
            ProtocolVersion::V1
        );
        assert_eq!(
            ExtXPlaylistType::Event.requires_version(),
            ProtocolVersion::V1
        );
    }
}
