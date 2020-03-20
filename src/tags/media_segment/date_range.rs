use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

#[cfg(feature = "chrono")]
use chrono::{DateTime, FixedOffset, SecondsFormat};
use derive_builder::Builder;
use shorthand::ShortHand;

use crate::attribute::AttributePairs;
use crate::types::{ProtocolVersion, Value};
use crate::utils::{quote, tag, unquote};
use crate::{Error, RequiredVersion};

/// The [`ExtXDateRange`] tag associates a date range (i.e., a range of time
/// defined by a starting and ending date) with a set of attribute/value pairs.
#[derive(ShortHand, Builder, Debug, Clone, PartialEq, PartialOrd)]
#[builder(setter(into))]
#[shorthand(enable(must_use, into))]
pub struct ExtXDateRange {
    /// A string that uniquely identifies an [`ExtXDateRange`] in the Playlist.
    ///
    /// # Note
    ///
    /// This attribute is required.
    id: String,
    /// A client-defined string that specifies some set of attributes and their
    /// associated value semantics. All [`ExtXDateRange`]s with the same class
    /// attribute value must adhere to these semantics.
    ///
    /// # Note
    ///
    /// This attribute is optional.
    #[builder(setter(strip_option), default)]
    class: Option<String>,
    /// The date at which the [`ExtXDateRange`] begins.
    ///
    /// # Note
    ///
    /// This attribute is required.
    #[cfg(feature = "chrono")]
    start_date: DateTime<FixedOffset>,
    /// The date at which the [`ExtXDateRange`] ends. It must be equal to or
    /// later than the value of the [`start-date`] attribute.
    ///
    /// # Note
    ///
    /// This attribute is optional.
    ///
    /// [`start-date`]: #method.start_date
    #[cfg(feature = "chrono")]
    #[builder(setter(strip_option), default)]
    end_date: Option<DateTime<FixedOffset>>,
    #[cfg(not(feature = "chrono"))]
    #[builder(setter(strip_option), default)]
    end_date: Option<String>,
    /// The duration of the [`ExtXDateRange`]. A single instant in time (e.g.,
    /// crossing a finish line) should be represented with a duration of 0.
    ///
    /// # Note
    ///
    /// This attribute is optional.
    #[builder(setter(strip_option), default)]
    duration: Option<Duration>,
    /// The expected duration of the [`ExtXDateRange`].
    /// This attribute should be used to indicate the expected duration of a
    /// [`ExtXDateRange`] whose actual duration is not yet known.
    ///
    /// # Note
    ///
    /// This attribute is optional.
    #[builder(setter(strip_option), default)]
    planned_duration: Option<Duration>,
    /// You can read about this attribute here
    /// <https://tools.ietf.org/html/rfc8216#section-4.3.2.7.1>
    ///
    /// # Note
    ///
    /// This attribute is optional.
    #[builder(setter(strip_option), default)]
    scte35_cmd: Option<String>,
    /// You can read about this attribute here
    /// <https://tools.ietf.org/html/rfc8216#section-4.3.2.7.1>
    ///
    /// # Note
    ///
    /// This attribute is optional.
    #[builder(setter(strip_option), default)]
    scte35_out: Option<String>,
    /// You can read about this attribute here
    /// <https://tools.ietf.org/html/rfc8216#section-4.3.2.7.1>
    ///
    /// # Note
    ///
    /// This attribute is optional.
    #[builder(setter(strip_option), default)]
    scte35_in: Option<String>,
    /// This attribute indicates that the end of the range containing it is
    /// equal to the [`start-date`] of its following range. The following range
    /// is the [`ExtXDateRange`] of the same class, that has the earliest
    /// [`start-date`] after the [`start-date`] of the range in question.
    ///
    /// # Note
    ///
    /// This attribute is optional.
    #[builder(default)]
    end_on_next: bool,
    /// The `"X-"` prefix defines a namespace reserved for client-defined
    /// attributes. The client-attribute must be a uppercase characters.
    /// Clients should use a reverse-DNS syntax when defining their own
    /// attribute names to avoid collisions. An example of a client-defined
    /// attribute is `X-COM-EXAMPLE-AD-ID="XYZ123"`.
    ///
    /// # Note
    ///
    /// This attribute is optional.
    #[builder(default)]
    #[shorthand(enable(collection_magic, get_mut))]
    client_attributes: BTreeMap<String, Value>,
}

impl ExtXDateRangeBuilder {
    /// Inserts a key value pair.
    pub fn insert_client_attribute<K: Into<String>, V: Into<Value>>(
        &mut self,
        key: K,
        value: V,
    ) -> &mut Self {
        if self.client_attributes.is_none() {
            self.client_attributes = Some(BTreeMap::new());
        }

        if let Some(client_attributes) = &mut self.client_attributes {
            client_attributes.insert(key.into(), value.into());
        } else {
            unreachable!();
        }
        self
    }
}

impl ExtXDateRange {
    pub(crate) const PREFIX: &'static str = "#EXT-X-DATERANGE:";

    /// Makes a new [`ExtXDateRange`] tag.
    ///
    /// # Example
    #[cfg_attr(
        feature = "chrono",
        doc = r#"
```
# use hls_m3u8::tags::ExtXDateRange;
use chrono::offset::TimeZone;
use chrono::{DateTime, FixedOffset};

const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds

let date_range = ExtXDateRange::new(
    "id",
    FixedOffset::east(8 * HOURS_IN_SECS)
        .ymd(2010, 2, 19)
        .and_hms_milli(14, 54, 23, 31),
);
```
"#
    )]
    #[cfg_attr(
        not(feature = "chrono"),
        doc = r#"
```
# use hls_m3u8::tags::ExtXDateRange;

let date_range = ExtXDateRange::new("id", "2010-02-19T14:54:23.031+08:00");
```
    "#
    )]
    #[must_use]
    pub fn new<T: Into<String>, #[cfg(not(feature = "chrono"))] I: Into<String>>(
        id: T,
        #[cfg(feature = "chrono")] start_date: DateTime<FixedOffset>,
        #[cfg(not(feature = "chrono"))] start_date: I,
    ) -> Self {
        Self {
            id: id.into(),
            class: None,
            #[cfg(feature = "chrono")]
            start_date,
            #[cfg(not(feature = "chrono"))]
            start_date: start_date.into(),
            end_date: None,
            duration: None,
            planned_duration: None,
            scte35_cmd: None,
            scte35_out: None,
            scte35_in: None,
            end_on_next: false,
            client_attributes: BTreeMap::new(),
        }
    }

    /// Returns a builder for [`ExtXDateRange`].
    #[must_use]
    pub fn builder() -> ExtXDateRangeBuilder { ExtXDateRangeBuilder::default() }
}

/// This tag requires [`ProtocolVersion::V1`].
impl RequiredVersion for ExtXDateRange {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
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

        for (key, value) in AttributePairs::new(input) {
            match key {
                "ID" => id = Some(unquote(value)),
                "CLASS" => class = Some(unquote(value)),
                "START-DATE" => {
                    #[cfg(feature = "chrono")]
                    {
                        start_date = Some(unquote(value).parse().map_err(Error::chrono)?)
                    }
                    #[cfg(not(feature = "chrono"))]
                    {
                        start_date = Some(unquote(value))
                    }
                }
                "END-DATE" => {
                    #[cfg(feature = "chrono")]
                    {
                        end_date = Some(unquote(value).parse().map_err(Error::chrono)?)
                    }
                    #[cfg(not(feature = "chrono"))]
                    {
                        end_date = Some(unquote(value))
                    }
                }
                "DURATION" => {
                    duration = Some(Duration::from_secs_f64(
                        value.parse().map_err(|e| Error::parse_float(value, e))?,
                    ));
                }
                "PLANNED-DURATION" => {
                    planned_duration = Some(Duration::from_secs_f64(
                        value.parse().map_err(|e| Error::parse_float(value, e))?,
                    ));
                }
                "SCTE35-CMD" => scte35_cmd = Some(unquote(value)),
                "SCTE35-OUT" => scte35_out = Some(unquote(value)),
                "SCTE35-IN" => scte35_in = Some(unquote(value)),
                "END-ON-NEXT" => {
                    if value != "YES" {
                        return Err(Error::custom("The value of `END-ON-NEXT` has to be `YES`!"));
                    }
                    end_on_next = true;
                }
                _ => {
                    if key.starts_with("X-") {
                        client_attributes.insert(key.to_ascii_uppercase(), value.parse()?);
                    } else {
                        // [6.3.1. General Client Responsibilities]
                        // > ignore any attribute/value pair with an
                        // unrecognized AttributeName.
                    }
                }
            }
        }

        let id = id.ok_or_else(|| Error::missing_value("ID"))?;
        let start_date = start_date.ok_or_else(|| Error::missing_value("START-DATE"))?;

        if end_on_next && class.is_none() {
            return Err(Error::invalid_input());
        }
        Ok(Self {
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

impl fmt::Display for ExtXDateRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "ID={}", quote(&self.id))?;

        if let Some(value) = &self.class {
            write!(f, ",CLASS={}", quote(value))?;
        }

        #[cfg(feature = "chrono")]
        {
            write!(
                f,
                ",START-DATE={}",
                quote(&self.start_date.to_rfc3339_opts(SecondsFormat::AutoSi, true))
            )?;
        }

        #[cfg(not(feature = "chrono"))]
        {
            write!(f, ",START-DATE={}", quote(&self.start_date))?;
        }

        if let Some(value) = &self.end_date {
            #[cfg(feature = "chrono")]
            {
                write!(
                    f,
                    ",END-DATE={}",
                    quote(&value.to_rfc3339_opts(SecondsFormat::AutoSi, true))
                )?;
            }

            #[cfg(not(feature = "chrono"))]
            {
                write!(f, ",END-DATE={}", quote(&value))?;
            }
        }

        if let Some(value) = &self.duration {
            write!(f, ",DURATION={}", value.as_secs_f64())?;
        }

        if let Some(value) = &self.planned_duration {
            write!(f, ",PLANNED-DURATION={}", value.as_secs_f64())?;
        }

        if let Some(value) = &self.scte35_cmd {
            write!(f, ",SCTE35-CMD={}", value)?;
        }

        if let Some(value) = &self.scte35_out {
            write!(f, ",SCTE35-OUT={}", value)?;
        }

        if let Some(value) = &self.scte35_in {
            write!(f, ",SCTE35-IN={}", value)?;
        }

        for (k, v) in &self.client_attributes {
            write!(f, ",{}={}", k, v)?;
        }

        if self.end_on_next {
            write!(f, ",END-ON-NEXT=YES",)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::Float;
    #[cfg(feature = "chrono")]
    use chrono::offset::TimeZone;
    use pretty_assertions::assert_eq;

    #[cfg(feature = "chrono")]
    const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds

    macro_rules! generate_tests {
        ( $( { $left:expr, $right:expr } ),* $(,)* ) => {
            #[test]
            fn test_display() {
                $(
                    assert_eq!($left.to_string(), $right.to_string());
                )*
            }

            #[test]
            fn test_parser() {
                $(
                    assert_eq!($left, $right.parse().unwrap());
                )*
                assert!("#EXT-X-DATERANGE:END-ON-NEXT=NO"
                    .parse::<ExtXDateRange>()
                    .is_err());

                assert!("garbage".parse::<ExtXDateRange>().is_err());
                assert!("".parse::<ExtXDateRange>().is_err());

                assert!(concat!(
                    "#EXT-X-DATERANGE:",
                    "ID=\"test_id\",",
                    "START-DATE=\"2014-03-05T11:15:00Z\",",
                    "END-ON-NEXT=YES"
                )
                .parse::<ExtXDateRange>()
                .is_err());
            }
        }
    }

    generate_tests! {
        {
            ExtXDateRange::builder()
                .id("splice-6FFFFFF0")
                .start_date({
                    #[cfg(feature = "chrono")]
                    {
                        FixedOffset::east(0).ymd(2014, 3, 5).and_hms(11, 15, 0)
                    }
                    #[cfg(not(feature = "chrono"))]
                    {
                        "2014-03-05T11:15:00Z"
                    }
                })
                .planned_duration(Duration::from_secs_f64(59.993))
                .scte35_out(concat!(
                    "0xFC002F0000000000FF00001",
                    "4056FFFFFF000E011622DCAFF0",
                    "00052636200000000000A00080",
                    "29896F50000008700000000"
                ))
                .build()
                .unwrap(),
            concat!(
                "#EXT-X-DATERANGE:",
                "ID=\"splice-6FFFFFF0\",",
                "START-DATE=\"2014-03-05T11:15:00Z\",",
                "PLANNED-DURATION=59.993,",
                "SCTE35-OUT=0xFC002F0000000000FF000014056F",
                "FFFFF000E011622DCAFF000052636200000000000",
                "A0008029896F50000008700000000"
            )
        },
        {
            ExtXDateRange::builder()
                .id("test_id")
                .class("test_class")
                .start_date({
                    #[cfg(feature = "chrono")]
                    {
                        FixedOffset::east(0).ymd(2014, 3, 5).and_hms(11, 15, 0)
                    }
                    #[cfg(not(feature = "chrono"))]
                    {
                        "2014-03-05T11:15:00Z"
                    }
                })
                .end_date({
                    #[cfg(feature = "chrono")]
                    {
                        FixedOffset::east(0).ymd(2014, 3, 5).and_hms(11, 16, 0)
                    }
                    #[cfg(not(feature = "chrono"))]
                    {
                        "2014-03-05T11:16:00Z"
                    }
                })
                .duration(Duration::from_secs_f64(60.1))
                .planned_duration(Duration::from_secs_f64(59.993))
                .insert_client_attribute("X-CUSTOM", Float::new(45.3))
                .scte35_cmd("0xFC002F0000000000FF2")
                .scte35_out("0xFC002F0000000000FF0")
                .scte35_in("0xFC002F0000000000FF1")
                .end_on_next(true)
                .build()
                .unwrap(),
            concat!(
                "#EXT-X-DATERANGE:",
                "ID=\"test_id\",",
                "CLASS=\"test_class\",",
                "START-DATE=\"2014-03-05T11:15:00Z\",",
                "END-DATE=\"2014-03-05T11:16:00Z\",",
                "DURATION=60.1,",
                "PLANNED-DURATION=59.993,",
                "SCTE35-CMD=0xFC002F0000000000FF2,",
                "SCTE35-OUT=0xFC002F0000000000FF0,",
                "SCTE35-IN=0xFC002F0000000000FF1,",
                "X-CUSTOM=45.3,",
                "END-ON-NEXT=YES"
            )
        },
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXDateRange::new("id", {
                #[cfg(feature = "chrono")]
                {
                    FixedOffset::east(8 * HOURS_IN_SECS)
                        .ymd(2010, 2, 19)
                        .and_hms_milli(14, 54, 23, 31)
                }
                #[cfg(not(feature = "chrono"))]
                {
                    "2010-02-19T14:54:23.031+08:00"
                }
            })
            .required_version(),
            ProtocolVersion::V1
        );
    }
}
