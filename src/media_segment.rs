use std::fmt;
use std::iter;

use derive_builder::Builder;

use crate::tags::{
    ExtInf, ExtXByteRange, ExtXDateRange, ExtXDiscontinuity, ExtXKey, ExtXMap, ExtXProgramDateTime,
};
use crate::types::{ProtocolVersion, SingleLineString};

/// Media segment.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into, strip_option))]
pub struct MediaSegment {
    #[builder(default)]
    /// Sets all [ExtXKey] tags.
    key_tags: Vec<ExtXKey>,
    #[builder(default)]
    /// Sets an [ExtXMap] tag.
    map_tag: Option<ExtXMap>,
    #[builder(default)]
    /// Sets an [ExtXByteRange] tag.
    byte_range_tag: Option<ExtXByteRange>,
    #[builder(default)]
    /// Sets an [ExtXDateRange] tag.
    date_range_tag: Option<ExtXDateRange>,
    #[builder(default)]
    /// Sets an [ExtXDiscontinuity] tag.
    discontinuity_tag: Option<ExtXDiscontinuity>,
    #[builder(default)]
    /// Sets an [ExtXProgramDateTime] tag.
    program_date_time_tag: Option<ExtXProgramDateTime>,
    /// Sets an [ExtInf] tag.
    inf_tag: ExtInf,
    /// Sets an Uri.
    uri: SingleLineString,
}

impl MediaSegmentBuilder {
    /// Pushes an [ExtXKey] tag.
    pub fn push_key_tag<VALUE: Into<ExtXKey>>(&mut self, value: VALUE) -> &mut Self {
        if let Some(key_tags) = &mut self.key_tags {
            key_tags.push(value.into());
        } else {
            self.key_tags = Some(vec![value.into()]);
        }
        self
    }
}

impl fmt::Display for MediaSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for t in &self.key_tags {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.map_tag {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.byte_range_tag {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.date_range_tag {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.discontinuity_tag {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.program_date_time_tag {
            writeln!(f, "{}", t)?;
        }
        writeln!(f, "{},", self.inf_tag)?;
        writeln!(f, "{}", self.uri)?;
        Ok(())
    }
}

impl MediaSegment {
    /// Creates a [MediaSegmentBuilder].
    pub fn builder() -> MediaSegmentBuilder {
        MediaSegmentBuilder::default()
    }
    /// Returns the URI of the media segment.
    pub const fn uri(&self) -> &SingleLineString {
        &self.uri
    }

    /// Returns the `EXT-X-INF` tag associated with the media segment.
    pub const fn inf_tag(&self) -> &ExtInf {
        &self.inf_tag
    }

    /// Returns the `EXT-X-BYTERANGE` tag associated with the media segment.
    pub const fn byte_range_tag(&self) -> Option<ExtXByteRange> {
        self.byte_range_tag
    }

    /// Returns the `EXT-X-DATERANGE` tag associated with the media segment.
    pub fn date_range_tag(&self) -> Option<&ExtXDateRange> {
        self.date_range_tag.as_ref()
    }

    /// Returns the `EXT-X-DISCONTINUITY` tag associated with the media segment.
    pub const fn discontinuity_tag(&self) -> Option<ExtXDiscontinuity> {
        self.discontinuity_tag
    }

    /// Returns the `EXT-X-PROGRAM-DATE-TIME` tag associated with the media segment.
    pub fn program_date_time_tag(&self) -> Option<&ExtXProgramDateTime> {
        self.program_date_time_tag.as_ref()
    }

    /// Returns the `EXT-X-MAP` tag associated with the media segment.
    pub fn map_tag(&self) -> Option<&ExtXMap> {
        self.map_tag.as_ref()
    }

    /// Returns the `EXT-X-KEY` tags associated with the media segment.
    pub fn key_tags(&self) -> &[ExtXKey] {
        &self.key_tags
    }

    /// Returns the protocol compatibility version that this segment requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        iter::empty()
            .chain(self.key_tags.iter().map(|t| t.requires_version()))
            .chain(self.map_tag.iter().map(|t| t.requires_version()))
            .chain(self.byte_range_tag.iter().map(|t| t.requires_version()))
            .chain(self.date_range_tag.iter().map(|t| t.requires_version()))
            .chain(self.discontinuity_tag.iter().map(|t| t.requires_version()))
            .chain(
                self.program_date_time_tag
                    .iter()
                    .map(|t| t.requires_version()),
            )
            .chain(iter::once(self.inf_tag.requires_version()))
            .max()
            .unwrap_or(ProtocolVersion::V7)
    }
}
