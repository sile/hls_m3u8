use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::Error;

/// [4.3.3.1. EXT-X-TARGETDURATION]
///
/// [4.3.3.1. EXT-X-TARGETDURATION]: https://tools.ietf.org/html/rfc8216#section-4.3.3.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ExtXTargetDuration {
    duration: Duration,
}

impl ExtXTargetDuration {
    pub(crate) const PREFIX: &'static str = "#EXT-X-TARGETDURATION:";

    /// Makes a new `ExtXTargetduration` tag.
    ///
    /// Note that the nanoseconds part of the `duration` will be discarded.
    pub const fn new(duration: Duration) -> Self {
        let duration = Duration::from_secs(duration.as_secs());
        ExtXTargetDuration { duration }
    }

    /// Returns the maximum media segment duration in the associated playlist.
    pub const fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXTargetDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.duration.as_secs())
    }
}

impl FromStr for ExtXTargetDuration {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?.parse()?;
        Ok(ExtXTargetDuration {
            duration: Duration::from_secs(input),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_targetduration() {
        let tag = ExtXTargetDuration::new(Duration::from_secs(5));
        let text = "#EXT-X-TARGETDURATION:5";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }
}
