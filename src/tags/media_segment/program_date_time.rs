use std::fmt;
use std::str::FromStr;

use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::Error;

/// [4.3.2.6. EXT-X-PROGRAM-DATE-TIME]
///
/// [4.3.2.6. EXT-X-PROGRAM-DATE-TIME]: https://tools.ietf.org/html/rfc8216#section-4.3.2.6
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXProgramDateTime(String);

impl ExtXProgramDateTime {
    pub(crate) const PREFIX: &'static str = "#EXT-X-PROGRAM-DATE-TIME:";

    /// Makes a new `ExtXProgramDateTime` tag.
    pub fn new<T: ToString>(date_time: T) -> Self {
        Self(date_time.to_string())
    }

    /// Returns the date-time of the first sample of the associated media segment.
    pub const fn date_time(&self) -> &String {
        &self.0
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXProgramDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl FromStr for ExtXProgramDateTime {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        // TODO: parse with chrono

        Ok(Self::new(input))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_program_date_time() {
        let text = "#EXT-X-PROGRAM-DATE-TIME:2010-02-19T14:54:23.031+08:00";
        assert!(text.parse::<ExtXProgramDateTime>().is_ok());

        let tag = text.parse::<ExtXProgramDateTime>().unwrap();
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }
}
