use std::fmt;

use derive_builder::Builder;

use crate::tags::{
    ExtInf, ExtXByteRange, ExtXDateRange, ExtXDiscontinuity, ExtXKey, ExtXMap, ExtXProgramDateTime,
};
use crate::types::ProtocolVersion;
use crate::{Encrypted, RequiredVersion};

#[derive(Debug, Clone, Builder)]
#[builder(setter(into, strip_option))]
/// Media segment.
pub struct MediaSegment {
    #[builder(default)]
    /// Sets all [`ExtXKey`] tags.
    keys: Vec<ExtXKey>,
    #[builder(default)]
    /// Sets an [`ExtXMap`] tag.
    map_tag: Option<ExtXMap>,
    #[builder(default)]
    /// Sets an [`ExtXByteRange`] tag.
    byte_range_tag: Option<ExtXByteRange>,
    #[builder(default)]
    /// Sets an [`ExtXDateRange`] tag.
    date_range_tag: Option<ExtXDateRange>,
    #[builder(default)]
    /// Sets an [`ExtXDiscontinuity`] tag.
    discontinuity_tag: Option<ExtXDiscontinuity>,
    #[builder(default)]
    /// Sets an [`ExtXProgramDateTime`] tag.
    program_date_time_tag: Option<ExtXProgramDateTime>,
    /// Sets an [`ExtInf`] tag.
    inf_tag: ExtInf,
    /// Sets an `URI`.
    uri: String,
}

impl MediaSegment {
    /// Returns a Builder for a [`MasterPlaylist`].
    pub fn builder() -> MediaSegmentBuilder { MediaSegmentBuilder::default() }

    /// Returns the `URI` of the media segment.
    pub const fn uri(&self) -> &String { &self.uri }

    /// Sets the `URI` of the media segment.
    pub fn set_uri<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.uri = value.into();
        self
    }

    /// Returns the [`ExtInf`] tag associated with the media segment.
    pub const fn inf_tag(&self) -> &ExtInf { &self.inf_tag }

    /// Sets the [`ExtInf`] tag associated with the media segment.
    pub fn set_inf_tag<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<ExtInf>,
    {
        self.inf_tag = value.into();
        self
    }

    /// Returns the [`ExtXByteRange`] tag associated with the media segment.
    pub const fn byte_range_tag(&self) -> Option<ExtXByteRange> { self.byte_range_tag }

    /// Sets the [`ExtXByteRange`] tag associated with the media segment.
    pub fn set_byte_range_tag<T>(&mut self, value: Option<T>) -> &mut Self
    where
        T: Into<ExtXByteRange>,
    {
        self.byte_range_tag = value.map(Into::into);
        self
    }

    /// Returns the [`ExtXDateRange`] tag associated with the media segment.
    pub const fn date_range_tag(&self) -> &Option<ExtXDateRange> { &self.date_range_tag }

    /// Sets the [`ExtXDateRange`] tag associated with the media segment.
    pub fn set_date_range_tag<T>(&mut self, value: Option<T>) -> &mut Self
    where
        T: Into<ExtXDateRange>,
    {
        self.date_range_tag = value.map(Into::into);
        self
    }

    /// Returns the [`ExtXDiscontinuity`] tag associated with the media segment.
    pub const fn discontinuity_tag(&self) -> Option<ExtXDiscontinuity> { self.discontinuity_tag }

    /// Sets the [`ExtXDiscontinuity`] tag associated with the media segment.
    pub fn set_discontinuity_tag<T>(&mut self, value: Option<T>) -> &mut Self
    where
        T: Into<ExtXDiscontinuity>,
    {
        self.discontinuity_tag = value.map(Into::into);
        self
    }

    /// Returns the [`ExtXProgramDateTime`] tag associated with the media
    /// segment.
    pub const fn program_date_time_tag(&self) -> Option<ExtXProgramDateTime> {
        self.program_date_time_tag
    }

    /// Sets the [`ExtXProgramDateTime`] tag associated with the media
    /// segment.
    pub fn set_program_date_time_tag<T>(&mut self, value: Option<T>) -> &mut Self
    where
        T: Into<ExtXProgramDateTime>,
    {
        self.program_date_time_tag = value.map(Into::into);
        self
    }

    /// Returns the [`ExtXMap`] tag associated with the media segment.
    pub const fn map_tag(&self) -> &Option<ExtXMap> { &self.map_tag }

    /// Sets the [`ExtXMap`] tag associated with the media segment.
    pub fn set_map_tag<T>(&mut self, value: Option<T>) -> &mut Self
    where
        T: Into<ExtXMap>,
    {
        self.map_tag = value.map(Into::into);
        self
    }
}

impl MediaSegmentBuilder {
    /// Pushes an [`ExtXKey`] tag.
    pub fn push_key_tag<VALUE: Into<ExtXKey>>(&mut self, value: VALUE) -> &mut Self {
        if let Some(key_tags) = &mut self.keys {
            key_tags.push(value.into());
        } else {
            self.keys = Some(vec![value.into()]);
        }
        self
    }
}

impl fmt::Display for MediaSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for value in &self.keys {
            writeln!(f, "{}", value)?;
        }
        if let Some(value) = &self.map_tag {
            writeln!(f, "{}", value)?;
        }
        if let Some(value) = &self.byte_range_tag {
            writeln!(f, "{}", value)?;
        }
        if let Some(value) = &self.date_range_tag {
            writeln!(f, "{}", value)?;
        }
        if let Some(value) = &self.discontinuity_tag {
            writeln!(f, "{}", value)?;
        }
        if let Some(value) = &self.program_date_time_tag {
            writeln!(f, "{}", value)?;
        }
        writeln!(f, "{},", self.inf_tag)?;
        writeln!(f, "{}", self.uri)?;
        Ok(())
    }
}

impl RequiredVersion for MediaSegment {
    fn required_version(&self) -> ProtocolVersion {
        required_version![
            self.keys,
            self.map_tag,
            self.byte_range_tag,
            self.date_range_tag,
            self.discontinuity_tag,
            self.program_date_time_tag,
            self.inf_tag
        ]
    }
}

impl Encrypted for MediaSegment {
    fn keys(&self) -> &Vec<ExtXKey> { &self.keys }

    fn keys_mut(&mut self) -> &mut Vec<ExtXKey> { &mut self.keys }
}
