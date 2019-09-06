use crate::types::ProtocolVersion;
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;

/// [4.3.3.6. EXT-X-I-FRAMES-ONLY]
///
/// [4.3.3.6. EXT-X-I-FRAMES-ONLY]: https://tools.ietf.org/html/rfc8216#section-4.3.3.6
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXIFramesOnly;

impl ExtXIFramesOnly {
    pub(crate) const PREFIX: &'static str = "#EXT-X-I-FRAMES-ONLY";

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(self) -> ProtocolVersion {
        ProtocolVersion::V4
    }
}

impl fmt::Display for ExtXIFramesOnly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Self::PREFIX.fmt(f)
    }
}

impl FromStr for ExtXIFramesOnly {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert_eq!(s, Self::PREFIX, ErrorKind::InvalidInput);
        Ok(ExtXIFramesOnly)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_i_frames_only() {
        let tag = ExtXIFramesOnly;
        let text = "#EXT-X-I-FRAMES-ONLY";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V4);
    }
}
