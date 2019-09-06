use crate::attribute::AttributePairs;
use crate::types::{DecimalFloatingPoint, ProtocolVersion, QuotedString};
use crate::{Error, ErrorKind, Result};
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

/// [4.3.2.7.  EXT-X-DATERANGE]
///
/// [4.3.2.7.  EXT-X-DATERANGE]: https://tools.ietf.org/html/rfc8216#section-4.3.2.7
///
/// TODO: Implement properly
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXDateRange {
    pub id: QuotedString,
    pub class: Option<QuotedString>,
    pub start_date: QuotedString,
    pub end_date: Option<QuotedString>,
    pub duration: Option<Duration>,
    pub planned_duration: Option<Duration>,
    pub scte35_cmd: Option<QuotedString>,
    pub scte35_out: Option<QuotedString>,
    pub scte35_in: Option<QuotedString>,
    pub end_on_next: bool,
    pub client_attributes: BTreeMap<String, String>,
}

impl ExtXDateRange {
    pub(crate) const PREFIX: &'static str = "#EXT-X-DATERANGE:";

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXDateRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "ID={}", self.id)?;
        if let Some(ref x) = self.class {
            write!(f, ",CLASS={}", x)?;
        }
        write!(f, ",START-DATE={}", self.start_date)?;
        if let Some(ref x) = self.end_date {
            write!(f, ",END-DATE={}", x)?;
        }
        if let Some(x) = self.duration {
            write!(f, ",DURATION={}", DecimalFloatingPoint::from_duration(x))?;
        }
        if let Some(x) = self.planned_duration {
            write!(
                f,
                ",PLANNED-DURATION={}",
                DecimalFloatingPoint::from_duration(x)
            )?;
        }
        if let Some(ref x) = self.scte35_cmd {
            write!(f, ",SCTE35-CMD={}", x)?;
        }
        if let Some(ref x) = self.scte35_out {
            write!(f, ",SCTE35-OUT={}", x)?;
        }
        if let Some(ref x) = self.scte35_in {
            write!(f, ",SCTE35-IN={}", x)?;
        }
        if self.end_on_next {
            write!(f, ",END-ON-NEXT=YES",)?;
        }
        for (k, v) in &self.client_attributes {
            write!(f, ",{}={}", k, v)?;
        }
        Ok(())
    }
}

impl FromStr for ExtXDateRange {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut id = None;
        let mut class = None;
        let mut start_date = None;
        let mut end_date = None;
        let mut duration = None;
        let mut planned_duration = None;
        let mut scte35_cmd = None;
        let mut scte35_out = None;
        let mut scte35_in = None;
        let mut end_on_next = false;
        let mut client_attributes = BTreeMap::new();
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "ID" => id = Some(track!(value.parse())?),
                "CLASS" => class = Some(track!(value.parse())?),
                "START-DATE" => start_date = Some(track!(value.parse())?),
                "END-DATE" => end_date = Some(track!(value.parse())?),
                "DURATION" => {
                    let seconds: DecimalFloatingPoint = track!(value.parse())?;
                    duration = Some(seconds.to_duration());
                }
                "PLANNED-DURATION" => {
                    let seconds: DecimalFloatingPoint = track!(value.parse())?;
                    planned_duration = Some(seconds.to_duration());
                }
                "SCTE35-CMD" => scte35_cmd = Some(track!(value.parse())?),
                "SCTE35-OUT" => scte35_out = Some(track!(value.parse())?),
                "SCTE35-IN" => scte35_in = Some(track!(value.parse())?),
                "END-ON-NEXT" => {
                    track_assert_eq!(value, "YES", ErrorKind::InvalidInput);
                    end_on_next = true;
                }
                _ => {
                    if key.starts_with("X-") {
                        client_attributes.insert(key.split_at(2).1.to_owned(), value.to_owned());
                    } else {
                        // [6.3.1. General Client Responsibilities]
                        // > ignore any attribute/value pair with an unrecognized AttributeName.
                    }
                }
            }
        }

        let id = track_assert_some!(id, ErrorKind::InvalidInput);
        let start_date = track_assert_some!(start_date, ErrorKind::InvalidInput);
        if end_on_next {
            track_assert!(class.is_some(), ErrorKind::InvalidInput);
        }
        Ok(ExtXDateRange {
            id,
            class,
            start_date,
            end_date,
            duration,
            planned_duration,
            scte35_cmd,
            scte35_out,
            scte35_in,
            end_on_next,
            client_attributes,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test] // TODO; write some tests
    fn it_works() {

    }
}
