use std::fmt;
use std::str::FromStr;

use crate::Error;

/// Specifies the media type.
#[non_exhaustive]
#[expect(missing_docs)]
#[derive(Ord, PartialOrd, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MediaType {
    Audio,
    Video,
    Subtitles,
    ClosedCaptions,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Audio => "AUDIO",
            Self::Video => "VIDEO",
            Self::Subtitles => "SUBTITLES",
            Self::ClosedCaptions => "CLOSED-CAPTIONS",
        })
    }
}

impl FromStr for MediaType {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "AUDIO" => Ok(Self::Audio),
            "VIDEO" => Ok(Self::Video),
            "SUBTITLES" => Ok(Self::Subtitles),
            "CLOSED-CAPTIONS" => Ok(Self::ClosedCaptions),
            _ => Err(Error::custom(format!("invalid media type: {input:?}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parser() {
        assert_eq!(MediaType::Audio, "AUDIO".parse().unwrap());
        assert_eq!(MediaType::Video, "VIDEO".parse().unwrap());
        assert_eq!(MediaType::Subtitles, "SUBTITLES".parse().unwrap());
        assert_eq!(
            MediaType::ClosedCaptions,
            "CLOSED-CAPTIONS".parse().unwrap()
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(MediaType::Audio.to_string(), "AUDIO".to_string());
        assert_eq!(MediaType::Video.to_string(), "VIDEO".to_string());
        assert_eq!(MediaType::Subtitles.to_string(), "SUBTITLES".to_string());
        assert_eq!(
            MediaType::ClosedCaptions.to_string(),
            "CLOSED-CAPTIONS".to_string()
        );
    }
}
