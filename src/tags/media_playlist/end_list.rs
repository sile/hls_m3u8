use std::convert::TryFrom;
use std::fmt;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// Indicates that no more [`MediaSegment`]s will be added to the
/// [`MediaPlaylist`] file.
///
/// [`MediaSegment`]: crate::MediaSegment
/// [`MediaPlaylist`]: crate::MediaPlaylist
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ExtXEndList;

impl ExtXEndList {
    pub(crate) const PREFIX: &'static str = "#EXT-X-ENDLIST";
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXEndList {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl fmt::Display for ExtXEndList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { Self::PREFIX.fmt(f) }
}

impl TryFrom<&str> for ExtXEndList {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        tag(input, Self::PREFIX)?;
        Ok(Self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(ExtXEndList.to_string(), "#EXT-X-ENDLIST".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXEndList,
            ExtXEndList::try_from("#EXT-X-ENDLIST").unwrap()
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(ExtXEndList.required_version(), ProtocolVersion::V1);
    }
}
