use crate::types::{ProtocolVersion, SingleLineString};
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;

/// [4.3.2.6. EXT-X-PROGRAM-DATE-TIME]
///
/// [4.3.2.6. EXT-X-PROGRAM-DATE-TIME]: https://tools.ietf.org/html/rfc8216#section-4.3.2.6
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXProgramDateTime {
    date_time: SingleLineString,
}

impl ExtXProgramDateTime {
    pub(crate) const PREFIX: &'static str = "#EXT-X-PROGRAM-DATE-TIME:";

    /// Makes a new `ExtXProgramDateTime` tag.
    pub fn new(date_time: SingleLineString) -> Self {
        ExtXProgramDateTime { date_time }
    }

    /// Returns the date-time of the first sample of the associated media segment.
    pub fn date_time(&self) -> &SingleLineString {
        &self.date_time
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXProgramDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.date_time)
    }
}

impl FromStr for ExtXProgramDateTime {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let suffix = s.split_at(Self::PREFIX.len()).1;
        Ok(ExtXProgramDateTime {
            date_time: track!(SingleLineString::new(suffix))?,
        })
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
