use crate::types::ProtocolVersion;
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;

/// [4.3.3.4. EXT-X-ENDLIST]
///
/// [4.3.3.4. EXT-X-ENDLIST]: https://tools.ietf.org/html/rfc8216#section-4.3.3.4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXEndList;
impl ExtXEndList {
    pub(crate) const PREFIX: &'static str = "#EXT-X-ENDLIST";

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}
impl fmt::Display for ExtXEndList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Self::PREFIX.fmt(f)
    }
}
impl FromStr for ExtXEndList {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert_eq!(s, Self::PREFIX, ErrorKind::InvalidInput);
        Ok(ExtXEndList)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_endlist() {
        let tag = ExtXEndList;
        let text = "#EXT-X-ENDLIST";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }
}
