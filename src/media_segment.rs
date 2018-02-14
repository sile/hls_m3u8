use std::fmt;
use std::iter;

use {ErrorKind, Result};
use tag::{ExtInf, ExtXByteRange, ExtXDateRange, ExtXDiscontinuity, ExtXKey, ExtXMap,
          ExtXProgramDateTime, MediaSegmentTag};
use types::{ProtocolVersion, SingleLineString};

#[derive(Debug, Clone)]
pub struct MediaSegmentBuilder {
    uri: Option<SingleLineString>,
    ext_inf: Option<ExtInf>,
    ext_x_byterange: Option<ExtXByteRange>,
    ext_x_daterange: Option<ExtXDateRange>,
    ext_x_discontinuity: Option<ExtXDiscontinuity>,
    ext_x_key: Option<ExtXKey>, // TODO: vec
    ext_x_map: Option<ExtXMap>,
    ext_x_program_date_time: Option<ExtXProgramDateTime>,
}
impl MediaSegmentBuilder {
    pub fn new() -> Self {
        MediaSegmentBuilder {
            uri: None,
            ext_inf: None,
            ext_x_byterange: None,
            ext_x_daterange: None,
            ext_x_discontinuity: None,
            ext_x_key: None,
            ext_x_map: None,
            ext_x_program_date_time: None,
        }
    }
    pub fn uri(&mut self, uri: SingleLineString) -> &mut Self {
        self.uri = Some(uri);
        self
    }
    pub fn tag<T: Into<MediaSegmentTag>>(&mut self, tag: T) -> &mut Self {
        match tag.into() {
            MediaSegmentTag::ExtInf(t) => self.ext_inf = Some(t),
            MediaSegmentTag::ExtXByteRange(t) => self.ext_x_byterange = Some(t),
            MediaSegmentTag::ExtXDateRange(t) => self.ext_x_daterange = Some(t),
            MediaSegmentTag::ExtXDiscontinuity(t) => self.ext_x_discontinuity = Some(t),
            MediaSegmentTag::ExtXKey(t) => self.ext_x_key = Some(t),
            MediaSegmentTag::ExtXMap(t) => self.ext_x_map = Some(t),
            MediaSegmentTag::ExtXProgramDateTime(t) => self.ext_x_program_date_time = Some(t),
        }
        self
    }
    pub fn finish(self) -> Result<MediaSegment> {
        let uri = track_assert_some!(self.uri, ErrorKind::InvalidInput);
        let ext_inf = track_assert_some!(self.ext_inf, ErrorKind::InvalidInput);
        let tags = iter::empty()
            .chain(self.ext_x_byterange.into_iter().map(From::from))
            .chain(self.ext_x_daterange.into_iter().map(From::from))
            .chain(self.ext_x_discontinuity.into_iter().map(From::from))
            .chain(self.ext_x_key.into_iter().map(From::from))
            .chain(self.ext_x_map.into_iter().map(From::from))
            .chain(self.ext_x_program_date_time.into_iter().map(From::from))
            .collect();
        Ok(MediaSegment { uri, ext_inf, tags })
    }
}

#[derive(Debug, Clone)]
pub struct MediaSegment {
    pub uri: SingleLineString,
    pub ext_inf: ExtInf,
    pub tags: Vec<MediaSegmentTag>,
}
impl fmt::Display for MediaSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for tag in &self.tags {
            writeln!(f, "{}", tag)?;
        }
        writeln!(f, "{}", self.ext_inf)?;
        writeln!(f, "{}", self.uri)?;
        Ok(())
    }
}
impl MediaSegment {
    pub fn requires_version(&self) -> ProtocolVersion {
        // TODO:
        ProtocolVersion::V1
    }
    pub fn uri(&self) -> &str {
        &self.uri
    }
    pub fn inf(&self) -> &ExtInf {
        &self.ext_inf
    }
    pub fn byte_range_tag(&self) -> Option<&ExtXByteRange> {
        self.tags.iter().filter_map(|t| t.as_byte_range()).nth(0)
    }
    pub fn date_range(&self) -> Option<&ExtXDateRange> {
        self.tags.iter().filter_map(|t| t.as_date_range()).nth(0)
    }
    pub fn discontinuity(&self) -> Option<&ExtXDiscontinuity> {
        self.tags.iter().filter_map(|t| t.as_discontinuity()).nth(0)
    }
    pub fn key(&self) -> Option<&ExtXKey> {
        self.tags.iter().filter_map(|t| t.as_key()).nth(0)
    }
    pub fn map(&self) -> Option<&ExtXMap> {
        self.tags.iter().filter_map(|t| t.as_map()).nth(0)
    }
    pub fn program_date_time(&self) -> Option<&ExtXProgramDateTime> {
        self.tags
            .iter()
            .filter_map(|t| t.as_program_date_time())
            .nth(0)
    }
}
