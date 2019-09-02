use crate::error::{Error, ErrorKind};
use std::fmt;
use std::str::FromStr;

/// HDCP level.
///
/// See: [4.3.4.2. EXT-X-STREAM-INF]
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HdcpLevel {
    Type0,
    None,
}

impl fmt::Display for HdcpLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HdcpLevel::Type0 => "TYPE-0".fmt(f),
            HdcpLevel::None => "NONE".fmt(f),
        }
    }
}

impl FromStr for HdcpLevel {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TYPE-0" => Ok(HdcpLevel::Type0),
            "NONE" => Ok(HdcpLevel::None),
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown HDCP level: {:?}", s),
        }
    }
}
