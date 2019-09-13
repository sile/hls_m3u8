use std::fmt;
use std::str::FromStr;

use crate::types::{PlaylistType, ProtocolVersion};
use crate::utils::tag;
use crate::Error;

/// [4.3.3.5. EXT-X-PLAYLIST-TYPE]
///
/// [4.3.3.5. EXT-X-PLAYLIST-TYPE]: https://tools.ietf.org/html/rfc8216#section-4.3.3.5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXPlaylistType {
    playlist_type: PlaylistType,
}

impl ExtXPlaylistType {
    pub(crate) const PREFIX: &'static str = "#EXT-X-PLAYLIST-TYPE:";

    /// Makes a new `ExtXPlaylistType` tag.
    pub const fn new(playlist_type: PlaylistType) -> Self {
        ExtXPlaylistType { playlist_type }
    }

    /// Returns the type of the associated media playlist.
    pub const fn playlist_type(self) -> PlaylistType {
        self.playlist_type
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXPlaylistType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.playlist_type)
    }
}

impl FromStr for ExtXPlaylistType {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?.parse()?;

        Ok(ExtXPlaylistType::new(input))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_playlist_type() {
        let tag = ExtXPlaylistType::new(PlaylistType::Vod);
        let text = "#EXT-X-PLAYLIST-TYPE:VOD";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }
}
