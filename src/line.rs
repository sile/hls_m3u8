use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use url::Url;

use crate::tags;
use crate::Error;

#[derive(Debug, Default)]
pub struct Lines(Vec<Line>);

impl Lines {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FromStr for Lines {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut result = Lines::new();

        let mut stream_inf = false;
        let mut stream_inf_line = None;

        for l in input.lines() {
            let line = l.trim();

            // ignore empty lines
            if line.len() == 0 {
                continue;
            }

            let pline = {
                if line.starts_with(tags::ExtXStreamInf::PREFIX) {
                    stream_inf = true;
                    stream_inf_line = Some(line);

                    continue;
                } else if line.starts_with("#EXT") {
                    Line::Tag(line.parse()?)
                } else if line.starts_with("#") {
                    continue; // ignore comments
                } else {
                    // stream inf line needs special treatment
                    if stream_inf {
                        stream_inf = false;
                        if let Some(first_line) = stream_inf_line {
                            let res = Line::Tag(format!("{}\n{}", first_line, line).parse()?);
                            stream_inf_line = None;
                            res
                        } else {
                            continue;
                        }
                    } else {
                        Line::Uri(line.trim().parse()?)
                    }
                }
            };

            result.push(pline);
        }

        Ok(result)
    }
}

impl IntoIterator for Lines {
    type Item = Line;
    type IntoIter = ::std::vec::IntoIter<Line>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for Lines {
    type Target = Vec<Line>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Lines {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Line {
    Tag(Tag),
    Uri(Url),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    ExtM3u(tags::ExtM3u),
    ExtXVersion(tags::ExtXVersion),
    ExtInf(tags::ExtInf),
    ExtXByteRange(tags::ExtXByteRange),
    ExtXDiscontinuity(tags::ExtXDiscontinuity),
    ExtXKey(tags::ExtXKey),
    ExtXMap(tags::ExtXMap),
    ExtXProgramDateTime(tags::ExtXProgramDateTime),
    ExtXDateRange(tags::ExtXDateRange),
    ExtXTargetDuration(tags::ExtXTargetDuration),
    ExtXMediaSequence(tags::ExtXMediaSequence),
    ExtXDiscontinuitySequence(tags::ExtXDiscontinuitySequence),
    ExtXEndList(tags::ExtXEndList),
    ExtXPlaylistType(tags::ExtXPlaylistType),
    ExtXIFramesOnly(tags::ExtXIFramesOnly),
    ExtXMedia(tags::ExtXMedia),
    ExtXStreamInf(tags::ExtXStreamInf),
    ExtXIFrameStreamInf(tags::ExtXIFrameStreamInf),
    ExtXSessionData(tags::ExtXSessionData),
    ExtXSessionKey(tags::ExtXSessionKey),
    ExtXIndependentSegments(tags::ExtXIndependentSegments),
    ExtXStart(tags::ExtXStart),
    Unknown(String),
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Tag::ExtM3u(value) => value.fmt(f),
            Tag::ExtXVersion(value) => value.fmt(f),
            Tag::ExtInf(value) => value.fmt(f),
            Tag::ExtXByteRange(value) => value.fmt(f),
            Tag::ExtXDiscontinuity(value) => value.fmt(f),
            Tag::ExtXKey(value) => value.fmt(f),
            Tag::ExtXMap(value) => value.fmt(f),
            Tag::ExtXProgramDateTime(value) => value.fmt(f),
            Tag::ExtXDateRange(value) => value.fmt(f),
            Tag::ExtXTargetDuration(value) => value.fmt(f),
            Tag::ExtXMediaSequence(value) => value.fmt(f),
            Tag::ExtXDiscontinuitySequence(value) => value.fmt(f),
            Tag::ExtXEndList(value) => value.fmt(f),
            Tag::ExtXPlaylistType(value) => value.fmt(f),
            Tag::ExtXIFramesOnly(value) => value.fmt(f),
            Tag::ExtXMedia(value) => value.fmt(f),
            Tag::ExtXStreamInf(value) => value.fmt(f),
            Tag::ExtXIFrameStreamInf(value) => value.fmt(f),
            Tag::ExtXSessionData(value) => value.fmt(f),
            Tag::ExtXSessionKey(value) => value.fmt(f),
            Tag::ExtXIndependentSegments(value) => value.fmt(f),
            Tag::ExtXStart(value) => value.fmt(f),
            Tag::Unknown(value) => value.fmt(f),
        }
    }
}

impl FromStr for Tag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(tags::ExtM3u::PREFIX) {
            s.parse().map(Tag::ExtM3u)
        } else if s.starts_with(tags::ExtXVersion::PREFIX) {
            s.parse().map(Tag::ExtXVersion)
        } else if s.starts_with(tags::ExtInf::PREFIX) {
            s.parse().map(Tag::ExtInf)
        } else if s.starts_with(tags::ExtXByteRange::PREFIX) {
            s.parse().map(Tag::ExtXByteRange)
        } else if s.starts_with(tags::ExtXDiscontinuity::PREFIX) {
            s.parse().map(Tag::ExtXDiscontinuity)
        } else if s.starts_with(tags::ExtXKey::PREFIX) {
            s.parse().map(Tag::ExtXKey)
        } else if s.starts_with(tags::ExtXMap::PREFIX) {
            s.parse().map(Tag::ExtXMap)
        } else if s.starts_with(tags::ExtXProgramDateTime::PREFIX) {
            s.parse().map(Tag::ExtXProgramDateTime)
        } else if s.starts_with(tags::ExtXTargetDuration::PREFIX) {
            s.parse().map(Tag::ExtXTargetDuration)
        } else if s.starts_with(tags::ExtXDateRange::PREFIX) {
            s.parse().map(Tag::ExtXDateRange)
        } else if s.starts_with(tags::ExtXMediaSequence::PREFIX) {
            s.parse().map(Tag::ExtXMediaSequence)
        } else if s.starts_with(tags::ExtXDiscontinuitySequence::PREFIX) {
            s.parse().map(Tag::ExtXDiscontinuitySequence)
        } else if s.starts_with(tags::ExtXEndList::PREFIX) {
            s.parse().map(Tag::ExtXEndList)
        } else if s.starts_with(tags::ExtXPlaylistType::PREFIX) {
            s.parse().map(Tag::ExtXPlaylistType)
        } else if s.starts_with(tags::ExtXIFramesOnly::PREFIX) {
            s.parse().map(Tag::ExtXIFramesOnly)
        } else if s.starts_with(tags::ExtXMedia::PREFIX) {
            s.parse().map(Tag::ExtXMedia)
        } else if s.starts_with(tags::ExtXStreamInf::PREFIX) {
            s.parse().map(Tag::ExtXStreamInf)
        } else if s.starts_with(tags::ExtXIFrameStreamInf::PREFIX) {
            s.parse().map(Tag::ExtXIFrameStreamInf)
        } else if s.starts_with(tags::ExtXSessionData::PREFIX) {
            s.parse().map(Tag::ExtXSessionData)
        } else if s.starts_with(tags::ExtXSessionKey::PREFIX) {
            s.parse().map(Tag::ExtXSessionKey)
        } else if s.starts_with(tags::ExtXIndependentSegments::PREFIX) {
            s.parse().map(Tag::ExtXIndependentSegments)
        } else if s.starts_with(tags::ExtXStart::PREFIX) {
            s.parse().map(Tag::ExtXStart)
        } else {
            Ok(Tag::Unknown(s.to_string()))
        }
    }
}
