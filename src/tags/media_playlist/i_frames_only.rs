use std::fmt;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Error, RequiredVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ExtXIFramesOnly;

impl ExtXIFramesOnly {
    pub(crate) const PREFIX: &'static str = "#EXT-X-I-FRAMES-ONLY";
}

/// This tag requires [`ProtocolVersion::V4`].
impl RequiredVersion for ExtXIFramesOnly {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V4 }
}

impl fmt::Display for ExtXIFramesOnly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { Self::PREFIX.fmt(f) }
}

impl FromStr for ExtXIFramesOnly {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
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
        assert_eq!(
            ExtXIFramesOnly.to_string(),
            "#EXT-X-I-FRAMES-ONLY".to_string(),
        )
    }

    #[test]
    fn test_parser() { assert_eq!(ExtXIFramesOnly, "#EXT-X-I-FRAMES-ONLY".parse().unwrap(),) }

    #[test]
    fn test_required_version() {
        assert_eq!(ExtXIFramesOnly.required_version(), ProtocolVersion::V4)
    }
}
