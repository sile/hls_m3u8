use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use chrono::{DateTime, FixedOffset};

use crate::attribute::AttributePairs;
use crate::types::{DecimalFloatingPoint, ProtocolVersion, RequiredVersion};
use crate::utils::{quote, tag, unquote};
use crate::Error;

/// [4.3.2.7.  EXT-X-DATERANGE]
///
/// [4.3.2.7.  EXT-X-DATERANGE]: https://tools.ietf.org/html/rfc8216#section-4.3.2.7
///
/// TODO: Implement properly
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXDateRange {
    /// A string that uniquely identifies a [ExtXDateRange] in the Playlist.
    /// This attribute is required.
    id: String,
    /// A client-defined string that specifies some set of attributes and their associated value
    /// semantics. All Date Ranges with the same CLASS attribute value MUST adhere to these
    /// semantics. This attribute is OPTIONAL.
    class: Option<String>,
    /// The date at which the Date Range begins. This attribute is REQUIRED.
    start_date: DateTime<FixedOffset>,
    /// The date at which the Date Range ends. It MUST be equal to or later than the value of the
    /// START-DATE attribute.  This attribute is OPTIONAL.
    end_date: Option<DateTime<FixedOffset>>,
    /// The duration of the Date Range. It MUST NOT be negative. A single
    /// instant in time (e.g., crossing a finish line) SHOULD be
    /// represented with a duration of 0.  This attribute is OPTIONAL.
    duration: Option<Duration>,
    /// The expected duration of the Date Range. It MUST NOT be negative. This
    /// attribute SHOULD be used to indicate the expected duration of a
    /// Date Range whose actual duration is not yet known.
    /// It is OPTIONAL.
    planned_duration: Option<Duration>,
    ///
    scte35_cmd: Option<String>,
    ///
    scte35_out: Option<String>,
    ///
    scte35_in: Option<String>,
    /// This attribute indicates that the end of the range containing it is equal to the
    /// START-DATE of its Following Range. The Following Range is the
    /// Date Range of the same CLASS, that has the earliest START-DATE
    /// after the START-DATE of the range in question. This attribute is
    /// OPTIONAL.
    end_on_next: bool,
    /// The "X-" prefix defines a namespace reserved for client-defined
    /// attributes. The client-attribute MUST be a legal AttributeName.
    /// Clients SHOULD use a reverse-DNS syntax when defining their own
    /// attribute names to avoid collisions. The attribute value MUST be
    /// a quoted-string, a hexadecimal-sequence, or a decimal-floating-
    /// point. An example of a client-defined attribute is X-COM-EXAMPLE-
    /// AD-ID="XYZ123". These attributes are OPTIONAL.
    client_attributes: BTreeMap<String, String>,
}

impl ExtXDateRange {
    pub(crate) const PREFIX: &'static str = "#EXT-X-DATERANGE:";
}

impl RequiredVersion for ExtXDateRange {
    fn required_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXDateRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "ID={}", quote(&self.id))?;
        if let Some(value) = &self.class {
            write!(f, ",CLASS={}", quote(value))?;
        }
        write!(f, ",START-DATE={}", quote(&self.start_date))?;
        if let Some(value) = &self.end_date {
            write!(f, ",END-DATE={}", quote(value))?;
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
        if let Some(value) = &self.scte35_cmd {
            write!(f, ",SCTE35-CMD={}", quote(value))?;
        }
        if let Some(value) = &self.scte35_out {
            write!(f, ",SCTE35-OUT={}", quote(value))?;
        }
        if let Some(value) = &self.scte35_in {
            write!(f, ",SCTE35-IN={}", quote(value))?;
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

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

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

        for (key, value) in input.parse::<AttributePairs>()? {
            match key.as_str() {
                "ID" => id = Some(unquote(value)),
                "CLASS" => class = Some(unquote(value)),
                "START-DATE" => start_date = Some(unquote(value)),
                "END-DATE" => end_date = Some(unquote(value).parse()?),
                "DURATION" => {
                    let seconds: DecimalFloatingPoint = (value.parse())?;
                    duration = Some(seconds.to_duration());
                }
                "PLANNED-DURATION" => {
                    let seconds: DecimalFloatingPoint = (value.parse())?;
                    planned_duration = Some(seconds.to_duration());
                }
                "SCTE35-CMD" => scte35_cmd = Some(unquote(value)),
                "SCTE35-OUT" => scte35_out = Some(unquote(value)),
                "SCTE35-IN" => scte35_in = Some(unquote(value)),
                "END-ON-NEXT" => {
                    if value != "YES" {
                        return Err(Error::invalid_input());
                    }
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

        let id = id.ok_or(Error::missing_value("EXT-X-ID"))?;
        let start_date = start_date
            .ok_or(Error::missing_value("EXT-X-START-DATE"))?
            .parse()?;

        if end_on_next {
            if class.is_none() {
                return Err(Error::invalid_input());
            }
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
    fn it_works() {}
}
