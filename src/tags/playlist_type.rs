use std::fmt;
use std::str::FromStr;

use trackable::error::ErrorKindExt;

use crate::error::{Error, ErrorKind};
use crate::types::{PlaylistType, ProtocolVersion};

/// [4.3.3.5. EXT-X-PLAYLIST-TYPE]
///
/// [4.3.3.5. EXT-X-PLAYLIST-TYPE]: https://tools.ietf.org/html/rfc8216#section-4.3.3.5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXPlaylistType(PlaylistType);

impl ExtXPlaylistType {
    pub(crate) const PREFIX: &'static str = "#EXT-X-PLAYLIST-TYPE:";

    /// Makes a new `ExtXPlaylistType` tag.
    pub fn new(playlist_type: PlaylistType) -> Self {
        Self(playlist_type)
    }

    /// Returns the type of the associated media playlist.
    pub fn playlist_type(self) -> PlaylistType {
        self.0
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn required_version(self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXPlaylistType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl FromStr for ExtXPlaylistType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let playlist_type = may_invalid!(s.split_at(Self::PREFIX.len()).1.parse())?;
        Ok(Self(playlist_type))
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
        assert_eq!(tag.required_version(), ProtocolVersion::V1);
    }
}
