use std::fmt;
use std::str::{self, FromStr};

use crate::Error;

/// Media type.
///
/// See: [4.3.4.1. EXT-X-MEDIA]
///
/// [4.3.4.1. EXT-X-MEDIA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.1
#[allow(missing_docs)]
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

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "AUDIO" => MediaType::Audio,
            "VIDEO" => MediaType::Video,
            "SUBTITLES" => MediaType::Subtitles,
            "CLOSED-CAPTIONS" => MediaType::ClosedCaptions,
            _ => {
                return Err(Error::invalid_input());
            }
        })
    }
}
