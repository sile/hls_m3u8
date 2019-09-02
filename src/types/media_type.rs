use std::fmt;
use std::str::FromStr;

use crate::error::{Error, ErrorKind};

/// Media type.
///
/// See: [4.3.4.1. EXT-X-MEDIA]
///
#[allow(missing_docs)]
/// [4.3.4.1. EXT-X-MEDIA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MediaType {
    Audio,
    Video,
    Subtitles,
    ClosedCaptions,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MediaType::Audio => "AUDIO".fmt(f),
            MediaType::Video => "VIDEO".fmt(f),
            MediaType::Subtitles => "SUBTITLES".fmt(f),
            MediaType::ClosedCaptions => "CLOSED-CAPTIONS".fmt(f),
        }
    }
}

impl FromStr for MediaType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "AUDIO" => MediaType::Audio,
            "VIDEO" => MediaType::Video,
            "SUBTITLES" => MediaType::Subtitles,
            "CLOSED-CAPTIONS" => MediaType::ClosedCaptions,
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown media type: {:?}", s),
        })
    }
}
