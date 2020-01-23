use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use chrono::{DateTime, FixedOffset, SecondsFormat};
use derive_builder::Builder;

use crate::attribute::AttributePairs;
use crate::types::{ProtocolVersion, Value};
use crate::utils::{quote, tag, unquote};
use crate::{Error, RequiredVersion};

/// # [4.3.2.7. EXT-X-DATERANGE]
/// The [`ExtXDateRange`] tag associates a date range (i.e., a range of
/// time defined by a starting and ending date) with a set of attribute/
/// value pairs.
///
/// [4.3.2.7. EXT-X-DATERANGE]: https://tools.ietf.org/html/rfc8216#section-4.3.2.7
#[derive(Builder, Debug, Clone, PartialEq, PartialOrd)]
#[builder(setter(into))]
pub struct ExtXDateRange {
    /// A string that uniquely identifies an [`ExtXDateRange`] in the Playlist.
    ///
    /// # Note
    /// This attribute is required.
    id: String,
    #[builder(setter(strip_option), default)]
    /// A client-defined string that specifies some set of attributes and their
    /// associated value semantics. All [`ExtXDateRange`]s with the same class
    /// attribute value must adhere to these semantics.
    ///
    /// # Note
    /// This attribute is optional.
    class: Option<String>,
    /// The date at which the [`ExtXDateRange`] begins.
    ///
    /// # Note
    /// This attribute is required.
    start_date: DateTime<FixedOffset>,
    #[builder(setter(strip_option), default)]
    /// The date at which the [`ExtXDateRange`] ends. It must be equal to or
    /// later than the value of the [`start-date`] attribute.
    ///
    /// # Note
    /// This attribute is optional.
    ///
    /// [`start-date`]: #method.start_date
    end_date: Option<DateTime<FixedOffset>>,
    #[builder(setter(strip_option), default)]
    /// The duration of the [`ExtXDateRange`]. A single instant in time (e.g.,
    /// crossing a finish line) should be represented with a duration of 0.
    ///
    /// # Note
    /// This attribute is optional.
    duration: Option<Duration>,
    #[builder(setter(strip_option), default)]
    /// The expected duration of the [`ExtXDateRange`].
    /// This attribute should be used to indicate the expected duration of a
    /// [`ExtXDateRange`] whose actual duration is not yet known.
    ///
    /// # Note
    /// This attribute is optional.
    planned_duration: Option<Duration>,
    #[builder(setter(strip_option), default)]
    /// https://tools.ietf.org/html/rfc8216#section-4.3.2.7.1
    ///
    /// # Note
    /// This attribute is optional.
    scte35_cmd: Option<String>,
    #[builder(setter(strip_option), default)]
    /// https://tools.ietf.org/html/rfc8216#section-4.3.2.7.1
    ///
    /// # Note
    /// This attribute is optional.
    scte35_out: Option<String>,
    #[builder(setter(strip_option), default)]
    /// https://tools.ietf.org/html/rfc8216#section-4.3.2.7.1
    ///
    /// # Note
    /// This attribute is optional.
    scte35_in: Option<String>,
    #[builder(default)]
    /// This attribute indicates that the end of the range containing it is
    /// equal to the [`start-date`] of its following range. The following range
    /// is the [`ExtXDateRange`] of the same class, that has the earliest
    /// [`start-date`] after the [`start-date`] of the range in question.
    ///
    /// # Note
    /// This attribute is optional.
    end_on_next: bool,
    #[builder(default)]
    /// The `"X-"` prefix defines a namespace reserved for client-defined
    /// attributes. The client-attribute must be a uppercase characters.
    /// Clients should use a reverse-DNS syntax when defining their own
    /// attribute names to avoid collisions. An example of a client-defined
    /// attribute is `X-COM-EXAMPLE-AD-ID="XYZ123"`.
    ///
    /// # Note
    /// This attribute is optional.
    client_attributes: BTreeMap<String, Value>,
}

impl ExtXDateRangeBuilder {
    /// Inserts a key value pair.
    pub fn insert_client_attribute<K: ToString, V: Into<Value>>(
        &mut self,
        key: K,
        value: V,
    ) -> &mut Self {
        if self.client_attributes.is_none() {
            self.client_attributes = Some(BTreeMap::new());
        }

        if let Some(client_attributes) = &mut self.client_attributes {
            client_attributes.insert(key.to_string(), value.into());
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
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// ```
    pub fn new<T: ToString>(id: T, start_date: DateTime<FixedOffset>) -> Self {
        Self {
            id: id.to_string(),
            class: None,
            start_date,
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
    pub fn builder() -> ExtXDateRangeBuilder { ExtXDateRangeBuilder::default() }

    /// A string that uniquely identifies an [`ExtXDateRange`] in the Playlist.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    ///
    /// assert_eq!(date_range.id(), &"id".to_string());
    /// ```
    pub const fn id(&self) -> &String { &self.id }

    /// A string that uniquely identifies an [`ExtXDateRange`] in the Playlist.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    ///
    /// date_range.set_id("new_id");
    /// assert_eq!(date_range.id(), &"new_id".to_string());
    /// ```
    pub fn set_id<T: ToString>(&mut self, value: T) -> &mut Self {
        self.id = value.to_string();
        self
    }

    /// A client-defined string that specifies some set of attributes and their
    /// associated value semantics. All [`ExtXDateRange`]s with the same class
    /// attribute value must adhere to these semantics.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// # assert_eq!(date_range.class(), &None);
    ///
    /// date_range.set_class(Some("example_class"));
    /// assert_eq!(date_range.class(), &Some("example_class".to_string()));
    /// ```
    pub const fn class(&self) -> &Option<String> { &self.class }

    /// A client-defined string that specifies some set of attributes and their
    /// associated value semantics. All [`ExtXDateRange`]s with the same class
    /// attribute value must adhere to these semantics.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// # assert_eq!(date_range.class(), &None);
    ///
    /// date_range.set_class(Some("example_class"));
    /// assert_eq!(date_range.class(), &Some("example_class".to_string()));
    /// ```
    pub fn set_class<T: ToString>(&mut self, value: Option<T>) -> &mut Self {
        self.class = value.map(|v| v.to_string());
        self
    }

    /// The date at which the [`ExtXDateRange`] begins.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    ///
    /// assert_eq!(
    ///     date_range.start_date(),
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31)
    /// );
    /// ```
    pub const fn start_date(&self) -> DateTime<FixedOffset> { self.start_date }

    /// The date at which the [`ExtXDateRange`] begins.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    ///
    /// date_range.set_start_date(
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 10, 10)
    ///         .and_hms_milli(10, 10, 10, 10),
    /// );
    /// assert_eq!(
    ///     date_range.start_date(),
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 10, 10)
    ///         .and_hms_milli(10, 10, 10, 10)
    /// );
    /// ```
    pub fn set_start_date<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<DateTime<FixedOffset>>,
    {
        self.start_date = value.into();
        self
    }

    /// The date at which the [`ExtXDateRange`] ends. It must be equal to or
    /// later than the value of the [`start-date`] attribute.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// # assert_eq!(date_range.end_date(), None);
    ///
    /// date_range.set_end_date(Some(
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 10, 10)
    ///         .and_hms_milli(10, 10, 10, 10),
    /// ));
    /// assert_eq!(
    ///     date_range.end_date(),
    ///     Some(
    ///         FixedOffset::east(8 * HOURS_IN_SECS)
    ///             .ymd(2010, 10, 10)
    ///             .and_hms_milli(10, 10, 10, 10)
    ///     )
    /// );
    /// ```
    pub const fn end_date(&self) -> Option<DateTime<FixedOffset>> { self.end_date }

    /// The date at which the [`ExtXDateRange`] ends. It must be equal to or
    /// later than the value of the [`start-date`] attribute.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// # assert_eq!(date_range.end_date(), None);
    ///
    /// date_range.set_end_date(Some(
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 10, 10)
    ///         .and_hms_milli(10, 10, 10, 10),
    /// ));
    /// assert_eq!(
    ///     date_range.end_date(),
    ///     Some(
    ///         FixedOffset::east(8 * HOURS_IN_SECS)
    ///             .ymd(2010, 10, 10)
    ///             .and_hms_milli(10, 10, 10, 10)
    ///     )
    /// );
    /// ```
    pub fn set_end_date<T>(&mut self, value: Option<T>) -> &mut Self
    where
        T: Into<DateTime<FixedOffset>>,
    {
        self.end_date = value.map(Into::into);
        self
    }

    /// The duration of the [`ExtXDateRange`]. A single instant in time (e.g.,
    /// crossing a finish line) should be represented with a duration of 0.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    /// use std::time::Duration;
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// # assert_eq!(date_range.duration(), None);
    ///
    /// date_range.set_duration(Some(Duration::from_secs_f64(1.234)));
    /// assert_eq!(date_range.duration(), Some(Duration::from_secs_f64(1.234)));
    /// ```
    pub const fn duration(&self) -> Option<Duration> { self.duration }

    /// The duration of the [`ExtXDateRange`]. A single instant in time (e.g.,
    /// crossing a finish line) should be represented with a duration of 0.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    /// use std::time::Duration;
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// # assert_eq!(date_range.duration(), None);
    ///
    /// date_range.set_duration(Some(Duration::from_secs_f64(1.234)));
    /// assert_eq!(date_range.duration(), Some(Duration::from_secs_f64(1.234)));
    /// ```
    pub fn set_duration(&mut self, value: Option<Duration>) -> &mut Self {
        self.duration = value;
        self
    }

    /// The expected duration of the [`ExtXDateRange`].
    /// This attribute should be used to indicate the expected duration of a
    /// [`ExtXDateRange`] whose actual duration is not yet known.
    /// The date at which the [`ExtXDateRange`] ends. It must be equal to or
    /// later than the value of the [`start-date`] attribute.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    /// use std::time::Duration;
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// # assert_eq!(date_range.planned_duration(), None);
    ///
    /// date_range.set_planned_duration(Some(Duration::from_secs_f64(1.2345)));
    /// assert_eq!(
    ///     date_range.planned_duration(),
    ///     Some(Duration::from_secs_f64(1.2345))
    /// );
    /// ```
    pub const fn planned_duration(&self) -> Option<Duration> { self.planned_duration }

    /// The expected duration of the [`ExtXDateRange`].
    /// This attribute should be used to indicate the expected duration of a
    /// [`ExtXDateRange`] whose actual duration is not yet known.
    /// The date at which the [`ExtXDateRange`] ends. It must be equal to or
    /// later than the value of the [`start-date`] attribute.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    /// use std::time::Duration;
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// # assert_eq!(date_range.planned_duration(), None);
    ///
    /// date_range.set_planned_duration(Some(Duration::from_secs_f64(1.2345)));
    /// assert_eq!(
    ///     date_range.planned_duration(),
    ///     Some(Duration::from_secs_f64(1.2345))
    /// );
    /// ```
    pub fn set_planned_duration(&mut self, value: Option<Duration>) -> &mut Self {
        self.planned_duration = value;
        self
    }

    /// See here for reference: https://www.scte.org/SCTEDocs/Standards/ANSI_SCTE%2035%202019r1.pdf
    pub const fn scte35_cmd(&self) -> &Option<String> { &self.scte35_cmd }

    /// See here for reference: https://www.scte.org/SCTEDocs/Standards/ANSI_SCTE%2035%202019r1.pdf
    pub const fn scte35_in(&self) -> &Option<String> { &self.scte35_in }

    /// See here for reference: https://www.scte.org/SCTEDocs/Standards/ANSI_SCTE%2035%202019r1.pdf
    pub const fn scte35_out(&self) -> &Option<String> { &self.scte35_out }

    /// This attribute indicates that the end of the range containing it is
    /// equal to the [`start-date`] of its following range. The following range
    /// is the [`ExtXDateRange`] of the same class, that has the earliest
    /// [`start-date`] after the [`start-date`] of the range in question.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    /// use std::time::Duration;
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// # assert_eq!(date_range.end_on_next(), false);
    ///
    /// date_range.set_end_on_next(true);
    /// assert_eq!(date_range.end_on_next(), true);
    /// ```
    pub const fn end_on_next(&self) -> bool { self.end_on_next }

    /// This attribute indicates that the end of the range containing it is
    /// equal to the [`start-date`] of its following range. The following range
    /// is the [`ExtXDateRange`] of the same class, that has the earliest
    /// [`start-date`] after the [`start-date`] of the range in question.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    /// use std::time::Duration;
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// # assert_eq!(date_range.end_on_next(), false);
    ///
    /// date_range.set_end_on_next(true);
    /// assert_eq!(date_range.end_on_next(), true);
    /// ```
    pub fn set_end_on_next(&mut self, value: bool) -> &mut Self {
        self.end_on_next = value;
        self
    }

    /// The "X-" prefix defines a namespace reserved for client-defined
    /// attributes. The client-attribute must be a uppercase characters.
    /// Clients should use a reverse-DNS syntax when defining their own
    /// attribute names to avoid collisions. An example of a client-defined
    /// attribute is `X-COM-EXAMPLE-AD-ID="XYZ123"`.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use std::collections::BTreeMap;
    ///
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    /// use hls_m3u8::types::Value;
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// # assert_eq!(date_range.client_attributes(), &BTreeMap::new());
    ///
    /// let mut attributes = BTreeMap::new();
    /// attributes.insert("X-COM-EXAMPLE-FLOAT".to_string(), Value::Float(1.1));
    ///
    /// date_range.set_client_attributes(attributes.clone());
    /// assert_eq!(date_range.client_attributes(), &attributes);
    /// ```
    pub const fn client_attributes(&self) -> &BTreeMap<String, Value> { &self.client_attributes }

    /// The "X-" prefix defines a namespace reserved for client-defined
    /// attributes. The client-attribute must be a uppercase characters.
    /// Clients should use a reverse-DNS syntax when defining their own
    /// attribute names to avoid collisions. An example of a client-defined
    /// attribute is `X-COM-EXAMPLE-AD-ID="XYZ123"`.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use std::collections::BTreeMap;
    ///
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    /// use hls_m3u8::types::Value;
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// # assert_eq!(date_range.client_attributes(), &BTreeMap::new());
    ///
    /// let mut attributes = BTreeMap::new();
    /// attributes.insert("X-COM-EXAMPLE-FLOAT".to_string(), Value::Float(1.1));
    ///
    /// date_range
    ///     .client_attributes_mut()
    ///     .insert("X-COM-EXAMPLE-FLOAT".to_string(), Value::Float(1.1));
    ///
    /// assert_eq!(date_range.client_attributes(), &attributes);
    /// ```
    pub fn client_attributes_mut(&mut self) -> &mut BTreeMap<String, Value> {
        &mut self.client_attributes
    }

    /// The "X-" prefix defines a namespace reserved for client-defined
    /// attributes. The client-attribute must be a uppercase characters.
    /// Clients should use a reverse-DNS syntax when defining their own
    /// attribute names to avoid collisions. An example of a client-defined
    /// attribute is `X-COM-EXAMPLE-AD-ID="XYZ123"`.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::tags::ExtXDateRange;
    /// use std::collections::BTreeMap;
    ///
    /// use chrono::offset::TimeZone;
    /// use chrono::{DateTime, FixedOffset};
    /// use hls_m3u8::types::Value;
    ///
    /// const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds
    ///
    /// let mut date_range = ExtXDateRange::new(
    ///     "id",
    ///     FixedOffset::east(8 * HOURS_IN_SECS)
    ///         .ymd(2010, 2, 19)
    ///         .and_hms_milli(14, 54, 23, 31),
    /// );
    /// # assert_eq!(date_range.client_attributes(), &BTreeMap::new());
    ///
    /// let mut attributes = BTreeMap::new();
    /// attributes.insert("X-COM-EXAMPLE-FLOAT".to_string(), Value::Float(1.1));
    ///
    /// date_range.set_client_attributes(attributes.clone());
    /// assert_eq!(date_range.client_attributes(), &attributes);
    /// ```
    pub fn set_client_attributes(&mut self, value: BTreeMap<String, Value>) -> &mut Self {
        self.client_attributes = value;
        self
    }
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

        for (key, value) in input.parse::<AttributePairs>()? {
            match key.as_str() {
                "ID" => id = Some(unquote(value)),
                "CLASS" => class = Some(unquote(value)),
                "START-DATE" => start_date = Some(unquote(value)),
                "END-DATE" => end_date = Some(unquote(value).parse().map_err(Error::chrono)?),
                "DURATION" => {
                    duration = Some(Duration::from_secs_f64(value.parse()?));
                }
                "PLANNED-DURATION" => {
                    planned_duration = Some(Duration::from_secs_f64(value.parse()?));
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
        let start_date = start_date
            .ok_or_else(|| Error::missing_value("START-DATE"))?
            .parse()
            .map_err(Error::chrono)?;

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

        write!(
            f,
            ",START-DATE={}",
            quote(&self.start_date.to_rfc3339_opts(SecondsFormat::AutoSi, true))
        )?;

        if let Some(value) = &self.end_date {
            write!(
                f,
                ",END-DATE={}",
                quote(value.to_rfc3339_opts(SecondsFormat::AutoSi, true))
            )?;
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
    use chrono::offset::TimeZone;
    use pretty_assertions::assert_eq;

    const HOURS_IN_SECS: i32 = 3600; // 1 hour = 3600 seconds

    #[test]
    fn test_parser() {
        assert_eq!(
            "#EXT-X-DATERANGE:\
             ID=\"splice-6FFFFFF0\",\
             START-DATE=\"2014-03-05T11:15:00Z\",\
             PLANNED-DURATION=59.993,\
             SCTE35-OUT=0xFC002F0000000000FF000014056F\
             FFFFF000E011622DCAFF000052636200000000000\
             A0008029896F50000008700000000"
                .parse::<ExtXDateRange>()
                .unwrap(),
            ExtXDateRange::builder()
                .id("splice-6FFFFFF0")
                .start_date(FixedOffset::east(0).ymd(2014, 3, 5).and_hms(11, 15, 0))
                .planned_duration(Duration::from_secs_f64(59.993))
                .scte35_out(
                    "0xFC002F0000000000FF00001\
                     4056FFFFFF000E011622DCAFF0\
                     00052636200000000000A00080\
                     29896F50000008700000000"
                )
                .build()
                .unwrap()
        );

        assert_eq!(
            "#EXT-X-DATERANGE:\
             ID=\"test_id\",\
             CLASS=\"test_class\",\
             START-DATE=\"2014-03-05T11:15:00Z\",\
             END-DATE=\"2014-03-05T11:16:00Z\",\
             DURATION=60.1,\
             PLANNED-DURATION=59.993,\
             X-CUSTOM=45.3,\
             SCTE35-CMD=0xFC002F0000000000FF2,\
             SCTE35-OUT=0xFC002F0000000000FF0,\
             SCTE35-IN=0xFC002F0000000000FF1,\
             END-ON-NEXT=YES,\
             UNKNOWN=PHANTOM"
                .parse::<ExtXDateRange>()
                .unwrap(),
            ExtXDateRange::builder()
                .id("test_id")
                .class("test_class")
                .start_date(FixedOffset::east(0).ymd(2014, 3, 5).and_hms(11, 15, 0))
                .end_date(FixedOffset::east(0).ymd(2014, 3, 5).and_hms(11, 16, 0))
                .duration(Duration::from_secs_f64(60.1))
                .planned_duration(Duration::from_secs_f64(59.993))
                .insert_client_attribute("X-CUSTOM", 45.3)
                .scte35_cmd("0xFC002F0000000000FF2")
                .scte35_out("0xFC002F0000000000FF0")
                .scte35_in("0xFC002F0000000000FF1")
                .end_on_next(true)
                .build()
                .unwrap()
        );

        assert!("#EXT-X-DATERANGE:END-ON-NEXT=NO"
            .parse::<ExtXDateRange>()
            .is_err());

        assert!("garbage".parse::<ExtXDateRange>().is_err());
        assert!("".parse::<ExtXDateRange>().is_err());

        assert!("#EXT-X-DATERANGE:\
                 ID=\"test_id\",\
                 START-DATE=\"2014-03-05T11:15:00Z\",\
                 END-ON-NEXT=YES"
            .parse::<ExtXDateRange>()
            .is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(
            ExtXDateRange::builder()
                .id("test_id")
                .class("test_class")
                .start_date(FixedOffset::east(0).ymd(2014, 3, 5).and_hms(11, 15, 0))
                .end_date(FixedOffset::east(0).ymd(2014, 3, 5).and_hms(11, 16, 0))
                .duration(Duration::from_secs_f64(60.1))
                .planned_duration(Duration::from_secs_f64(59.993))
                .insert_client_attribute("X-CUSTOM", 45.3)
                .scte35_cmd("0xFC002F0000000000FF2")
                .scte35_out("0xFC002F0000000000FF0")
                .scte35_in("0xFC002F0000000000FF1")
                .end_on_next(true)
                .build()
                .unwrap()
                .to_string(),
            "#EXT-X-DATERANGE:\
             ID=\"test_id\",\
             CLASS=\"test_class\",\
             START-DATE=\"2014-03-05T11:15:00Z\",\
             END-DATE=\"2014-03-05T11:16:00Z\",\
             DURATION=60.1,\
             PLANNED-DURATION=59.993,\
             SCTE35-CMD=0xFC002F0000000000FF2,\
             SCTE35-OUT=0xFC002F0000000000FF0,\
             SCTE35-IN=0xFC002F0000000000FF1,\
             X-CUSTOM=45.3,\
             END-ON-NEXT=YES"
        )
    }

    #[test]
    fn test_required_version() {
        assert_eq!(
            ExtXDateRange::new(
                "id",
                FixedOffset::east(8 * HOURS_IN_SECS)
                    .ymd(2010, 2, 19)
                    .and_hms_milli(14, 54, 23, 31)
            )
            .required_version(),
            ProtocolVersion::V1
        );
    }
}
