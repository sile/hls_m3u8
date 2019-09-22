use std::fmt;
use std::str::FromStr;

use crate::types::{ProtocolVersion, RequiredVersion};
use crate::utils::tag;
use crate::Error;

/// [4.3.3.4. EXT-X-ENDLIST]
///
/// [4.3.3.4. EXT-X-ENDLIST]: https://tools.ietf.org/html/rfc8216#section-4.3.3.4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXEndList;

impl ExtXEndList {
    pub(crate) const PREFIX: &'static str = "#EXT-X-ENDLIST";
}

impl RequiredVersion for ExtXEndList {
    fn required_version(&self) -> ProtocolVersion {
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

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        tag(input, Self::PREFIX)?;
        Ok(ExtXEndList)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(ExtXEndList.to_string(), "#EXT-X-ENDLIST".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!(ExtXEndList, "#EXT-X-ENDLIST".parse().unwrap());
    }

    #[test]
    fn test_required_version() {
        assert_eq!(ExtXEndList.required_version(), ProtocolVersion::V1);
    }
}
