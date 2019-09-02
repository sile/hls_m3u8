use std::borrow::Cow;
use std::fmt;
use std::iter;

use derive_builder::Builder;

use crate::tags::{
    ExtInf, ExtXByteRange, ExtXDateRange, ExtXDiscontinuity, ExtXKey, ExtXMap, ExtXProgramDateTime,
};
use crate::types::ProtocolVersion;

/// Media segment.
#[derive(Builder, Debug, Clone)]
#[builder(setter(into, strip_option))]
pub struct MediaSegment {
    #[builder(default)]
    key_tags: Vec<ExtXKey>,
    #[builder(default)]
    map_tag: Option<ExtXMap>,
    #[builder(default)]
    byte_range_tag: Option<ExtXByteRange>,
    #[builder(default)]
    date_range_tag: Option<ExtXDateRange>,
    #[builder(default)]
    discontinuity_tag: Option<ExtXDiscontinuity>,
    #[builder(default)]
    program_date_time_tag: Option<ExtXProgramDateTime>,
    inf_tag: ExtInf,
    /// Sets the URI of the resulting media segment.
    uri: String,
}

impl fmt::Display for MediaSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for t in &self.key_tags {
            writeln!(f, "{}", t)?;
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
    pub fn builder() -> MediaSegmentBuilder {
        MediaSegmentBuilder::default()
    }

    /// Returns the URI of the media segment.
    pub fn uri(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.uri)
    }

    /// Returns the `EXT-X-INF` tag associated with the media segment.
    pub fn inf_tag(&self) -> &ExtInf {
        &self.inf_tag
    }

    /// Returns the `EXT-X-BYTERANGE` tag associated with the media segment.
    pub fn byte_range_tag(&self) -> Option<ExtXByteRange> {
        self.byte_range_tag
    }

    /// Returns the `EXT-X-DATERANGE` tag associated with the media segment.
    pub fn date_range_tag(&self) -> Option<&ExtXDateRange> {
        self.date_range_tag.as_ref()
    }

    /// Returns the `EXT-X-DISCONTINUITY` tag associated with the media segment.
    pub fn discontinuity_tag(&self) -> Option<ExtXDiscontinuity> {
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
    pub fn required_version(&self) -> ProtocolVersion {
        iter::empty()
            .chain(self.key_tags.iter().map(|t| t.required_version()))
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
            .expect("Never fails")
    }
}
