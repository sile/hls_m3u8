use std::convert::TryFrom;
use std::fmt;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// Indicates the Media Sequence Number of the first `MediaSegment` that
/// appears in a `MediaPlaylist`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ExtXMediaSequence(pub usize);

impl ExtXMediaSequence {
    pub(crate) const PREFIX: &'static str = "#EXT-X-MEDIA-SEQUENCE:";
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXMediaSequence {
    fn required_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXMediaSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl TryFrom<&str> for ExtXMediaSequence {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input = tag(input, Self::PREFIX)?;
        let seq_num = input.parse().map_err(|e| Error::parse_int(input, e))?;

        Ok(Self(seq_num))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXMediaSequence(123).to_string(),
            "#EXT-X-MEDIA-SEQUENCE:123".to_string()
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXMediaSequence(123).required_version(),
            ProtocolVersion::V1
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXMediaSequence(123),
            ExtXMediaSequence::try_from("#EXT-X-MEDIA-SEQUENCE:123").unwrap()
        );
    }
}
