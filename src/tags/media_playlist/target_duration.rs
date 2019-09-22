use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use crate::types::{ProtocolVersion, RequiredVersion};
use crate::utils::tag;
use crate::Error;

/// [4.3.3.1. EXT-X-TARGETDURATION]
///
/// [4.3.3.1. EXT-X-TARGETDURATION]: https://tools.ietf.org/html/rfc8216#section-4.3.3.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ExtXTargetDuration(Duration);

impl ExtXTargetDuration {
    pub(crate) const PREFIX: &'static str = "#EXT-X-TARGETDURATION:";

    /// Makes a new `ExtXTargetduration` tag.
    ///
    /// Note that the nanoseconds part of the `duration` will be discarded.
    pub const fn new(duration: Duration) -> Self {
        Self(Duration::from_secs(duration.as_secs()))
    }

    /// Returns the maximum media segment duration in the associated playlist.
    pub const fn duration(&self) -> Duration {
        self.0
    }
}

impl RequiredVersion for ExtXTargetDuration {
    fn required_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXTargetDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0.as_secs())
    }
}

impl FromStr for ExtXTargetDuration {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?.parse()?;
        Ok(Self::new(Duration::from_secs(input)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXTargetDuration::new(Duration::from_secs(5)).to_string(),
            "#EXT-X-TARGETDURATION:5".to_string()
        );
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXTargetDuration::new(Duration::from_secs(5)).required_version(),
            ProtocolVersion::V1
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            ExtXTargetDuration::new(Duration::from_secs(5)),
            "#EXT-X-TARGETDURATION:5".parse().unwrap()
        );
    }
}
