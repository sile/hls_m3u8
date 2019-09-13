use std::fmt;
use std::str::{self, FromStr};

use crate::Error;

/// [7. Protocol Version Compatibility]
///
/// [7. Protocol Version Compatibility]: https://tools.ietf.org/html/rfc8216#section-7
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProtocolVersion {
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
}
impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = match *self {
            ProtocolVersion::V1 => 1,
            ProtocolVersion::V2 => 2,
            ProtocolVersion::V3 => 3,
            ProtocolVersion::V4 => 4,
            ProtocolVersion::V5 => 5,
            ProtocolVersion::V6 => 6,
            ProtocolVersion::V7 => 7,
        };
        write!(f, "{}", n)
    }
}
impl FromStr for ProtocolVersion {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "1" => ProtocolVersion::V1,
            "2" => ProtocolVersion::V2,
            "3" => ProtocolVersion::V3,
            "4" => ProtocolVersion::V4,
            "5" => ProtocolVersion::V5,
            "6" => ProtocolVersion::V6,
            "7" => ProtocolVersion::V7,
            _ => return Err(Error::unknown_protocol_version(input)),
        })
    }
}
