use std::fmt;
use std::str::FromStr;

use crate::attribute::AttributePairs;
use crate::types::{ProtocolVersion, SignedDecimalFloatingPoint};
use crate::utils::parse_yes_or_no;
use crate::{Error, ErrorKind};

/// [4.3.5.2. EXT-X-START]
///
/// [4.3.5.2. EXT-X-START]: https://tools.ietf.org/html/rfc8216#section-4.3.5.2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtXStart {
    time_offset: SignedDecimalFloatingPoint,
    precise: bool,
}

impl ExtXStart {
    pub(crate) const PREFIX: &'static str = "#EXT-X-START:";

    /// Makes a new `ExtXStart` tag.
    pub fn new(time_offset: f64) -> crate::Result<Self> {
        Ok(Self {
            time_offset: SignedDecimalFloatingPoint::new(time_offset)?,
            precise: false,
        })
    }

    /// Makes a new `ExtXStart` tag with the given `precise` flag.
    pub fn with_precise(time_offset: f64, precise: bool) -> crate::Result<Self> {
        Ok(Self {
            time_offset: SignedDecimalFloatingPoint::new(time_offset)?,
            precise,
        })
    }

    /// Returns the time offset of the media segments in the playlist.
    pub fn time_offset(&self) -> f64 {
        self.time_offset.as_f64()
    }

    /// Returns whether clients should not render media stream whose presentation times are
    /// prior to the specified time offset.
    pub fn precise(&self) -> bool {
        self.precise
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn required_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXStart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "TIME-OFFSET={}", self.time_offset)?;
        if self.precise {
            write!(f, ",PRECISE=YES")?;
        }
        Ok(())
    }
}

impl FromStr for ExtXStart {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut time_offset = None;
        let mut precise = false;
        let attrs = (s.split_at(Self::PREFIX.len()).1).parse::<AttributePairs>()?;
        for (key, value) in attrs {
            match key.as_str() {
                "TIME-OFFSET" => time_offset = Some(track!(value.parse())?),
                "PRECISE" => precise = track!(parse_yes_or_no(value))?,
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let time_offset = track_assert_some!(time_offset, ErrorKind::InvalidInput);
        Ok(ExtXStart {
            time_offset,
            precise,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_start() {
        let tag = ExtXStart::new(-1.23).unwrap();
        let text = "#EXT-X-START:TIME-OFFSET=-1.23";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.required_version(), ProtocolVersion::V1);

        let tag = ExtXStart::with_precise(1.23, true).unwrap();
        let text = "#EXT-X-START:TIME-OFFSET=1.23,PRECISE=YES";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.required_version(), ProtocolVersion::V1);
    }
}
