use std::fmt;
use std::iter;

use tags::{
    ExtInf, ExtXByteRange, ExtXDateRange, ExtXDiscontinuity, ExtXKey, ExtXMap, ExtXProgramDateTime,
    MediaSegmentTag,
};
use types::{ProtocolVersion, SingleLineString};
use {ErrorKind, Result};

/// Media segment builder.
#[derive(Debug, Clone)]
pub struct MediaSegmentBuilder {
    key_tags: Vec<ExtXKey>,
    map_tag: Option<ExtXMap>,
    byte_range_tag: Option<ExtXByteRange>,
    date_range_tag: Option<ExtXDateRange>,
    discontinuity_tag: Option<ExtXDiscontinuity>,
    program_date_time_tag: Option<ExtXProgramDateTime>,
    inf_tag: Option<ExtInf>,
    uri: Option<SingleLineString>,
}
impl MediaSegmentBuilder {
    /// Makes a new `MediaSegmentBuilder` instance.
    pub fn new() -> Self {
        MediaSegmentBuilder {
            key_tags: Vec::new(),
            map_tag: None,
            byte_range_tag: None,
            date_range_tag: None,
            discontinuity_tag: None,
            program_date_time_tag: None,
            inf_tag: None,
            uri: None,
        }
    }

    /// Sets the URI of the resulting media segment.
    pub fn uri(&mut self, uri: SingleLineString) -> &mut Self {
        self.uri = Some(uri);
        self
    }

    /// Sets the given tag to the resulting media segment.
    pub fn tag<T: Into<MediaSegmentTag>>(&mut self, tag: T) -> &mut Self {
        match tag.into() {
            MediaSegmentTag::ExtInf(t) => self.inf_tag = Some(t),
            MediaSegmentTag::ExtXByteRange(t) => self.byte_range_tag = Some(t),
            MediaSegmentTag::ExtXDateRange(t) => self.date_range_tag = Some(t),
            MediaSegmentTag::ExtXDiscontinuity(t) => self.discontinuity_tag = Some(t),
            MediaSegmentTag::ExtXKey(t) => self.key_tags.push(t),
            MediaSegmentTag::ExtXMap(t) => self.map_tag = Some(t),
            MediaSegmentTag::ExtXProgramDateTime(t) => self.program_date_time_tag = Some(t),
        }
        self
    }

    /// Builds a `MediaSegment` instance.
    pub fn finish(self) -> Result<MediaSegment> {
        let uri = track_assert_some!(self.uri, ErrorKind::InvalidInput);
        let inf_tag = track_assert_some!(self.inf_tag, ErrorKind::InvalidInput);
        Ok(MediaSegment {
            key_tags: self.key_tags,
            map_tag: self.map_tag,
            byte_range_tag: self.byte_range_tag,
            date_range_tag: self.date_range_tag,
            discontinuity_tag: self.discontinuity_tag,
            program_date_time_tag: self.program_date_time_tag,
            inf_tag,
            uri,
        })
    }
}
impl Default for MediaSegmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Media segment.
#[derive(Debug, Clone)]
pub struct MediaSegment {
    key_tags: Vec<ExtXKey>,
    map_tag: Option<ExtXMap>,
    byte_range_tag: Option<ExtXByteRange>,
    date_range_tag: Option<ExtXDateRange>,
    discontinuity_tag: Option<ExtXDiscontinuity>,
    program_date_time_tag: Option<ExtXProgramDateTime>,
    inf_tag: ExtInf,
    uri: SingleLineString,
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
        writeln!(f, "{}", self.inf_tag)?;
        writeln!(f, "{}", self.uri)?;
        Ok(())
    }
}
impl MediaSegment {
    /// Returns the URI of the media segment.
    pub fn uri(&self) -> &SingleLineString {
        &self.uri
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
            .expect("Never fails")
    }
}
