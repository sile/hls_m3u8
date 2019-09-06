use crate::{Error, Result};
use std::fmt;
use std::str::{self, FromStr};
use crate::types::QuotedString;

/// The identifier of a closed captions group or its absence.
///
/// See: [4.3.4.2. EXT-X-STREAM-INF]
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClosedCaptions {
    GroupId(QuotedString),
    None,
}

impl fmt::Display for ClosedCaptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClosedCaptions::GroupId(ref x) => x.fmt(f),
            ClosedCaptions::None => "NONE".fmt(f),
        }
    }
}

impl FromStr for ClosedCaptions {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s == "NONE" {
            Ok(ClosedCaptions::None)
        } else {
            Ok(ClosedCaptions::GroupId(track!(s.parse())?))
        }
    }
}
