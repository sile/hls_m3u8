use std::fmt;
use std::str::FromStr;

use crate::utils::{quote, unquote};
use crate::Error;

/// The identifier of a closed captions group or its absence.
///
/// See: [4.3.4.2. EXT-X-STREAM-INF]
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[allow(missing_docs)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ClosedCaptions {
    GroupId(String),
    None,
}

impl fmt::Display for ClosedCaptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::GroupId(value) => write!(f, "{}", quote(value)),
            Self::None => write!(f, "NONE"),
        }
    }
}

impl FromStr for ClosedCaptions {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.trim() == "NONE" {
            Ok(Self::None)
        } else {
            Ok(Self::GroupId(unquote(input)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(ClosedCaptions::None.to_string(), "NONE".to_string());

        assert_eq!(
            ClosedCaptions::GroupId("value".into()).to_string(),
            "\"value\"".to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ClosedCaptions::None,
            "NONE".parse::<ClosedCaptions>().unwrap()
        );

        assert_eq!(
            ClosedCaptions::GroupId("value".into()),
            "\"value\"".parse::<ClosedCaptions>().unwrap()
        );
    }
}
