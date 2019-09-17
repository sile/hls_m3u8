use std::fmt;
use std::str::FromStr;

use crate::utils::{quote, unquote};
use crate::{Error, Result};

/// The identifier of a closed captions group or its absence.
///
/// See: [4.3.4.2. EXT-X-STREAM-INF]
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClosedCaptions {
    GroupId(String),
    None,
}

impl fmt::Display for ClosedCaptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ClosedCaptions::GroupId(value) => write!(f, "{}", quote(value)),
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
            Ok(ClosedCaptions::GroupId(unquote(s)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let closed_captions = ClosedCaptions::None;
        assert_eq!(closed_captions.to_string(), "NONE".to_string());

        let closed_captions = ClosedCaptions::GroupId("value".into());
        assert_eq!(closed_captions.to_string(), "\"value\"".to_string());
    }

    #[test]
    fn test_parse() {
        let closed_captions = ClosedCaptions::None;
        assert_eq!(closed_captions, "NONE".parse::<ClosedCaptions>().unwrap());

        let closed_captions = ClosedCaptions::GroupId("value".into());
        assert_eq!(
            closed_captions,
            "\"value\"".parse::<ClosedCaptions>().unwrap()
        );
    }
}
