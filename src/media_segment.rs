use std::fmt;
use std::iter;

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

impl MediaSegment {
    /// Creates a [`MediaSegmentBuilder`].
    pub fn builder() -> MediaSegmentBuilder { MediaSegmentBuilder::default() }

    /// Returns the `URI` of the media segment.
    pub const fn uri(&self) -> &String { &self.uri }

    /// Returns the [`ExtInf`] tag associated with the media segment.
    pub const fn inf_tag(&self) -> &ExtInf { &self.inf_tag }

    /// Returns the [`ExtXByteRange`] tag associated with the media segment.
    pub const fn byte_range_tag(&self) -> Option<ExtXByteRange> { self.byte_range_tag }

    /// Returns the [`ExtXDateRange`] tag associated with the media segment.
    pub const fn date_range_tag(&self) -> &Option<ExtXDateRange> { &self.date_range_tag }

    /// Returns the [`ExtXDiscontinuity`] tag associated with the media segment.
    pub const fn discontinuity_tag(&self) -> Option<ExtXDiscontinuity> { self.discontinuity_tag }

    /// Returns the [`ExtXProgramDateTime`] tag associated with the media
    /// segment.
    pub const fn program_date_time_tag(&self) -> Option<ExtXProgramDateTime> {
        self.program_date_time_tag
    }

    /// Returns the [`ExtXMap`] tag associated with the media segment.
    pub const fn map_tag(&self) -> &Option<ExtXMap> { &self.map_tag }
}

impl RequiredVersion for MediaSegment {
    fn required_version(&self) -> ProtocolVersion {
        iter::empty()
            .chain(self.keys.iter().map(|t| t.required_version()))
            .chain(self.map_tag.iter().map(|t| t.required_version()))
            .chain(self.byte_range_tag.iter().map(|t| t.required_version()))
            .chain(self.date_range_tag.iter().map(|t| t.required_version()))
            .chain(self.discontinuity_tag.iter().map(|t| t.required_version()))
            .chain(
                self.program_date_time_tag
                    .iter()
                    .map(|t| t.required_version()),
            )
            .chain(iter::once(self.inf_tag.required_version()))
            .max()
            .unwrap_or_else(ProtocolVersion::latest)
    }
}

impl Encrypted for MediaSegment {
    fn keys(&self) -> &Vec<ExtXKey> { &self.keys }

    fn keys_mut(&mut self) -> &mut Vec<ExtXKey> { &mut self.keys }
}
