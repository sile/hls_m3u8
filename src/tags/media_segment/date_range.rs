use std::borrow::Cow;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt;
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
#[derive(ShortHand, Builder, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[builder(setter(into))]
#[shorthand(enable(must_use, into))]
pub struct ExtXDateRange<'a> {
    /// A string that uniquely identifies an [`ExtXDateRange`] in the playlist.
    ///
    /// ## Note
    ///
    /// This field is required.
    id: Cow<'a, str>,
    /// A client-defined string that specifies some set of attributes and their
    /// associated value semantics. All [`ExtXDateRange`]s with the same class
    /// attribute value must adhere to these semantics.
    ///
    /// ## Note
    ///
    /// This field is optional.
    #[builder(setter(strip_option), default)]
    class: Option<Cow<'a, str>>,
    /// The date at which the [`ExtXDateRange`] begins.
    ///
    /// ## Note
    ///
    /// This field is required by the spec wording, but optional in examples
    /// elsewhere in the same document.  Some implementations omit it in
    /// practise (e.g. for SCTE 'explicit-IN' markers) so it is optional
    /// here.
    #[cfg(feature = "chrono")]
    #[shorthand(enable(copy), disable(into))]
    #[builder(setter(strip_option), default)]
    start_date: Option<DateTime<FixedOffset>>,
    /// The date at which the [`ExtXDateRange`] begins.
    ///
    /// ## Note
    ///
    /// This field is required by the spec wording, but optional in examples
    /// elsewhere in the same document.  Some implementations omit it in
    /// practise (e.g. for SCTE 'explicit-IN' markers) so it is optional
    /// here.
    #[cfg(not(feature = "chrono"))]
    #[builder(setter(strip_option), default)]
    start_date: Option<Cow<'a, str>>,
    /// The date at which the [`ExtXDateRange`] ends. It must be equal to or
    /// later than the value of the [`start-date`] attribute.
    ///
    /// ## Note
    ///
    /// This field is optional.
    ///
    /// [`start-date`]: #method.start_date
    #[cfg(feature = "chrono")]
    #[shorthand(enable(copy), disable(into))]
    #[builder(setter(strip_option), default)]
    end_date: Option<DateTime<FixedOffset>>,
    /// The date at which the [`ExtXDateRange`] ends. It must be equal to or
    /// later than the value of the start-date field.
    ///
    /// ## Note
    ///
    /// This field is optional.
    ///
    /// [`start-date`]: #method.start_date
    #[cfg(not(feature = "chrono"))]
    #[builder(setter(strip_option), default)]
    end_date: Option<Cow<'a, str>>,
    /// The duration of the [`ExtXDateRange`]. A single instant in time (e.g.,
    /// crossing a finish line) should be represented with a duration of 0.
    ///
    /// ## Note
    ///
    /// This field is optional.
    #[builder(setter(strip_option), default)]
    #[shorthand(enable(skip))]
    pub duration: Option<Duration>,
    /// This field indicates the expected duration of an [`ExtXDateRange`],
    /// whose actual duration is not yet known.
    ///
    /// ## Note
    ///
    /// This field is optional.
    #[builder(setter(strip_option), default)]
    #[shorthand(enable(skip))]
    pub planned_duration: Option<Duration>,
    /// SCTE-35 (ANSI/SCTE 35 2013) is a joint ANSI/Society of Cable and
    /// Telecommunications Engineers standard that describes the inline
    /// insertion of cue tones in mpeg-ts streams.
    ///
    /// SCTE-35 was originally used in the US to signal a local ad insertion
    /// opportunity in the transport streams, and in Europe to insert local TV
    /// programs (e.g. local news transmissions). It is now used to signal all
    /// kinds of program and ad events in linear transport streams and in newer
    /// ABR delivery formats such as HLS and DASH.
    ///
    /// <https://en.wikipedia.org/wiki/SCTE-35>
    ///
    /// ## Note
    ///
    /// This field is optional.
    #[builder(setter(strip_option), default)]
    scte35_cmd: Option<Cow<'a, str>>,
    /// SCTE-35 (ANSI/SCTE 35 2013) is a joint ANSI/Society of Cable and
    /// Telecommunications Engineers standard that describes the inline
    /// insertion of cue tones in mpeg-ts streams.
    ///
    /// SCTE-35 was originally used in the US to signal a local ad insertion
    /// opportunity in the transport streams, and in Europe to insert local TV
    /// programs (e.g. local news transmissions). It is now used to signal all
    /// kinds of program and ad events in linear transport streams and in newer
    /// ABR delivery formats such as HLS and DASH.
    ///
    /// <https://en.wikipedia.org/wiki/SCTE-35>
    ///
    /// ## Note
    ///
    /// This field is optional.
    #[builder(setter(strip_option), default)]
    scte35_out: Option<Cow<'a, str>>,
    /// SCTE-35 (ANSI/SCTE 35 2013) is a joint ANSI/Society of Cable and
    /// Telecommunications Engineers standard that describes the inline
    /// insertion of cue tones in mpeg-ts streams.
    ///
    /// SCTE-35 was originally used in the US to signal a local ad insertion
    /// opportunity in the transport streams, and in Europe to insert local TV
    /// programs (e.g. local news transmissions). It is now used to signal all
    /// kinds of program and ad events in linear transport streams and in newer
    /// ABR delivery formats such as HLS and DASH.
    ///
    /// <https://en.wikipedia.org/wiki/SCTE-35>
    ///
    /// ## Note
    ///
    /// This field is optional.
    #[builder(setter(strip_option), default)]
    scte35_in: Option<Cow<'a, str>>,
    /// This field indicates that the [`ExtXDateRange::end_date`] is equal to
    /// the [`ExtXDateRange::start_date`] of the following range.
    ///
    /// The following range is the [`ExtXDateRange`] with the same class, that
    /// has the earliest start date after the start date of the range in
    /// question.
    ///
    /// ## Note
    ///
    /// This field is optional.
    #[builder(default)]
    #[shorthand(enable(skip))]
    pub end_on_next: bool,
    /// The `"X-"` prefix defines a namespace reserved for client-defined
    /// attributes.
    ///
    /// A client-attribute can only consist of uppercase characters (A-Z),
    /// numbers (0-9) and `-`.
    ///
    /// Clients should use a reverse-dns naming scheme, when defining
    /// their own attribute names to avoid collisions.
    ///
    /// An example of a client-defined attribute is
    /// `X-COM-EXAMPLE-AD-ID="XYZ123"`.
    ///
    /// ## Note
    ///
    /// This field is optional.
    #[builder(default)]
    #[shorthand(enable(collection_magic), disable(set, get))]
    pub client_attributes: BTreeMap<Cow<'a, str>, Value<'a>>,
}

impl<'a> ExtXDateRangeBuilder<'a> {
    /// Inserts a key value pair.
    pub fn insert_client_attribute<K: Into<Cow<'a, str>>, V: Into<Value<'a>>>(
        &mut self,
        key: K,
        value: V,
    ) -> &mut Self {
        let attrs = self.client_attributes.get_or_insert_with(BTreeMap::new);

        attrs.insert(key.into(), value.into());

        self
    }
}

impl<'a> ExtXDateRange<'a> {
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
    pub fn new<T: Into<Cow<'a, str>>, #[cfg(not(feature = "chrono"))] I: Into<Cow<'a, str>>>(
        id: T,
        #[cfg(feature = "chrono")] start_date: DateTime<FixedOffset>,
        #[cfg(not(feature = "chrono"))] start_date: I,
    ) -> Self {
        Self {
            id: id.into(),
            class: None,
            #[cfg(feature = "chrono")]
            start_date: Some(start_date),
            #[cfg(not(feature = "chrono"))]
            start_date: Some(start_date.into()),
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
    ///
    /// # Example
    #[cfg_attr(
        feature = "chrono",
        doc = r#"
```
# use hls_m3u8::tags::ExtXDateRange;
use std::time::Duration;
use chrono::{FixedOffset, TimeZone};
use hls_m3u8::types::Float;

let date_range = ExtXDateRange::builder()
    .id("test_id")
    .class("test_class")
    .start_date(FixedOffset::east(0).ymd(2014, 3, 5).and_hms(11, 15, 0))
    .end_date(FixedOffset::east(0).ymd(2014, 3, 5).and_hms(11, 16, 0))
    .duration(Duration::from_secs_f64(60.1))
    .planned_duration(Duration::from_secs_f64(59.993))
    .insert_client_attribute("X-CUSTOM", Float::new(45.3))
    .scte35_cmd("0xFC002F0000000000FF2")
    .scte35_out("0xFC002F0000000000FF0")
    .scte35_in("0xFC002F0000000000FF1")
    .end_on_next(true)
    .build()?;
# Ok::<(), String>(())
```
"#
    )]
    #[cfg_attr(
        not(feature = "chrono"),
        doc = r#"
```
# use hls_m3u8::tags::ExtXDateRange;
use std::time::Duration;
use hls_m3u8::types::Float;

let date_range = ExtXDateRange::builder()
    .id("test_id")
    .class("test_class")
    .start_date("2014-03-05T11:15:00Z")
    .end_date("2014-03-05T11:16:00Z")
    .duration(Duration::from_secs_f64(60.1))
    .planned_duration(Duration::from_secs_f64(59.993))
    .insert_client_attribute("X-CUSTOM", Float::new(45.3))
    .scte35_cmd("0xFC002F0000000000FF2")
    .scte35_out("0xFC002F0000000000FF0")
    .scte35_in("0xFC002F0000000000FF1")
    .end_on_next(true)
    .build()?;
# Ok::<(), String>(())
```
"#
    )]
    #[must_use]
    #[inline]
    pub fn builder() -> ExtXDateRangeBuilder<'a> { ExtXDateRangeBuilder::default() }

    /// Makes the struct independent of its lifetime, by taking ownership of all
    /// internal [`Cow`]s.
    ///
    /// # Note
    ///
    /// This is a relatively expensive operation.
    #[must_use]
    pub fn into_owned(self) -> ExtXDateRange<'static> {
        ExtXDateRange {
            id: Cow::Owned(self.id.into_owned()),
            class: self.class.map(|v| Cow::Owned(v.into_owned())),
            #[cfg(not(feature = "chrono"))]
            start_date: self.start_date.map(|v| Cow::Owned(v.into_owned())),
            #[cfg(feature = "chrono")]
            start_date: self.start_date,
            #[cfg(not(feature = "chrono"))]
            end_date: self.end_date.map(|v| Cow::Owned(v.into_owned())),
            #[cfg(feature = "chrono")]
            end_date: self.end_date,
            scte35_cmd: self.scte35_cmd.map(|v| Cow::Owned(v.into_owned())),
            scte35_out: self.scte35_out.map(|v| Cow::Owned(v.into_owned())),
            scte35_in: self.scte35_in.map(|v| Cow::Owned(v.into_owned())),
            client_attributes: {
                self.client_attributes
                    .into_iter()
                    .map(|(k, v)| (Cow::Owned(k.into_owned()), v.into_owned()))
                    .collect()
            },
            duration: self.duration,
            end_on_next: self.end_on_next,
            planned_duration: self.planned_duration,
        }
    }
}

/// This tag requires [`ProtocolVersion::V1`].
impl<'a> RequiredVersion for ExtXDateRange<'a> {
    fn required_version(&self) -> ProtocolVersion { ProtocolVersion::V1 }
}

impl<'a> TryFrom<&'a str> for ExtXDateRange<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
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
                        return Err(Error::custom("`END-ON-NEXT` must be `YES`"));
                    }
                    end_on_next = true;
                }
                _ => {
                    if key.starts_with("X-") {
                        if key.chars().any(|c| {
                            c.is_ascii_lowercase()
                                || !c.is_ascii()
                                || !(c.is_alphanumeric() || c == '-')
                        }) {
                            return Err(Error::custom(
                                "a client attribute can only consist of uppercase ascii characters, numbers or `-`",
                            ));
                        }

                        client_attributes.insert(Cow::Borrowed(key), Value::try_from(value)?);
                    } else {
                        // [6.3.1. General Client Responsibilities]
                        // > ignore any attribute/value pair with an
                        // unrecognized AttributeName.
                    }
                }
            }
        }

        let id = id.ok_or_else(|| Error::missing_value("ID"))?;

        if end_on_next && class.is_none() {
            return Err(Error::missing_attribute("CLASS"));
        } else if end_on_next && duration.is_some() {
            return Err(Error::unexpected_attribute("DURATION"));
        } else if end_on_next && end_date.is_some() {
            return Err(Error::unexpected_attribute("END-DATE"));
        }

        // TODO: verify this without chrono?
        // https://tools.ietf.org/html/rfc8216#section-4.3.2.7
        #[cfg(feature = "chrono")]
        {
            if let (Some(start_date), Some(Ok(duration)), Some(end_date)) = (
                start_date,
                duration.map(chrono::Duration::from_std),
                &end_date,
            ) {
                if start_date + duration != *end_date {
                    return Err(Error::custom(
                        "end_date must be equal to start_date + duration",
                    ));
                }
            }
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

impl<'a> fmt::Display for ExtXDateRange<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "ID={}", quote(&self.id))?;

        if let Some(value) = &self.class {
            write!(f, ",CLASS={}", quote(value))?;
        }

        if let Some(value) = &self.start_date {
            #[cfg(feature = "chrono")]
            {
                write!(
                    f,
                    ",START-DATE={}",
                    quote(&value.to_rfc3339_opts(SecondsFormat::AutoSi, true))
                )?;
            }

            #[cfg(not(feature = "chrono"))]
            {
                write!(f, ",START-DATE={}", quote(&value))?;
            }
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
                    assert_eq!($left, TryFrom::try_from($right).unwrap());
                )*
                assert!(ExtXDateRange::try_from("#EXT-X-DATERANGE:END-ON-NEXT=NO")
                    .is_err());

                assert!(ExtXDateRange::try_from("garbage").is_err());
                assert!(ExtXDateRange::try_from("").is_err());

                assert!(ExtXDateRange::try_from(concat!(
                    "#EXT-X-DATERANGE:",
                    "ID=\"test_id\",",
                    "START-DATE=\"2014-03-05T11:15:00Z\",",
                    "END-ON-NEXT=YES"
                ))
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
                        FixedOffset::east(0).ymd(2014, 3, 5).and_hms_milli(11, 16, 0, 100)
                    }
                    #[cfg(not(feature = "chrono"))]
                    {
                        "2014-03-05T11:16:00.100Z"
                    }
                })
                .duration(Duration::from_secs_f64(60.1))
                .planned_duration(Duration::from_secs_f64(59.993))
                .insert_client_attribute("X-CUSTOM", Float::new(45.3))
                .scte35_cmd("0xFC002F0000000000FF2")
                .scte35_out("0xFC002F0000000000FF0")
                .scte35_in("0xFC002F0000000000FF1")
                .build()
                .unwrap(),
            concat!(
                "#EXT-X-DATERANGE:",
                "ID=\"test_id\",",
                "CLASS=\"test_class\",",
                "START-DATE=\"2014-03-05T11:15:00Z\",",
                "END-DATE=\"2014-03-05T11:16:00.100Z\",",
                "DURATION=60.1,",
                "PLANNED-DURATION=59.993,",
                "SCTE35-CMD=0xFC002F0000000000FF2,",
                "SCTE35-OUT=0xFC002F0000000000FF0,",
                "SCTE35-IN=0xFC002F0000000000FF1,",
                "X-CUSTOM=45.3",
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
