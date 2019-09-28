use strum::{Display, EnumString};

/// Specifies the media type.
#[allow(missing_docs)]
#[derive(Ord, PartialOrd, Display, EnumString, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[strum(serialize_all = "SCREAMING-KEBAB-CASE")]
pub enum MediaType {
    Audio,
    Video,
    Subtitles,
    ClosedCaptions,
}

#[cfg(test)]
mod tests {
    use super::*;

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
