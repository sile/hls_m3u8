use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::{self, FromStr};

/// Playlist type.
///
/// See: [4.3.3.5. EXT-X-PLAYLIST-TYPE]
///
/// [4.3.3.5. EXT-X-PLAYLIST-TYPE]: https://tools.ietf.org/html/rfc8216#section-4.3.3.5
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaylistType {
    Event,
    Vod,
}

impl fmt::Display for PlaylistType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlaylistType::Event => write!(f, "EVENT"),
            PlaylistType::Vod => write!(f, "VOD"),
        }
    }
}

impl FromStr for PlaylistType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "EVENT" => Ok(PlaylistType::Event),
            "VOD" => Ok(PlaylistType::Vod),
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown playlist type: {:?}", s),
        }
    }
}
