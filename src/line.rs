use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use crate::tags;
use crate::types::SingleLineString;
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

        for line in input.lines() {
            // ignore empty lines
            if line.trim().len() == 0 {
                continue;
            }

            let pline = {
                if line.starts_with("#EXT") {
                    Line::Tag(line.parse()?)
                } else if line.starts_with("#") {
                    continue; // ignore comments
                } else {
                    Line::Uri(SingleLineString::new(line)?)
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
    Uri(SingleLineString),
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
    Unknown(SingleLineString),
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tag::ExtM3u(ref t) => t.fmt(f),
            Tag::ExtXVersion(ref t) => t.fmt(f),
            Tag::ExtInf(ref t) => t.fmt(f),
            Tag::ExtXByteRange(ref t) => t.fmt(f),
            Tag::ExtXDiscontinuity(ref t) => t.fmt(f),
            Tag::ExtXKey(ref t) => t.fmt(f),
            Tag::ExtXMap(ref t) => t.fmt(f),
            Tag::ExtXProgramDateTime(ref t) => t.fmt(f),
            Tag::ExtXDateRange(ref t) => t.fmt(f),
            Tag::ExtXTargetDuration(ref t) => t.fmt(f),
            Tag::ExtXMediaSequence(ref t) => t.fmt(f),
            Tag::ExtXDiscontinuitySequence(ref t) => t.fmt(f),
            Tag::ExtXEndList(ref t) => t.fmt(f),
            Tag::ExtXPlaylistType(ref t) => t.fmt(f),
            Tag::ExtXIFramesOnly(ref t) => t.fmt(f),
            Tag::ExtXMedia(ref t) => t.fmt(f),
            Tag::ExtXStreamInf(ref t) => t.fmt(f),
            Tag::ExtXIFrameStreamInf(ref t) => t.fmt(f),
            Tag::ExtXSessionData(ref t) => t.fmt(f),
            Tag::ExtXSessionKey(ref t) => t.fmt(f),
            Tag::ExtXIndependentSegments(ref t) => t.fmt(f),
            Tag::ExtXStart(ref t) => t.fmt(f),
            Tag::Unknown(ref t) => t.fmt(f),
        }
    }
}

impl FromStr for Tag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(tags::ExtM3u::PREFIX) {
            (s.parse().map(Tag::ExtM3u))
        } else if s.starts_with(tags::ExtXVersion::PREFIX) {
            (s.parse().map(Tag::ExtXVersion))
        } else if s.starts_with(tags::ExtInf::PREFIX) {
            (s.parse().map(Tag::ExtInf))
        } else if s.starts_with(tags::ExtXByteRange::PREFIX) {
            (s.parse().map(Tag::ExtXByteRange))
        } else if s.starts_with(tags::ExtXDiscontinuity::PREFIX) {
            (s.parse().map(Tag::ExtXDiscontinuity))
        } else if s.starts_with(tags::ExtXKey::PREFIX) {
            (s.parse().map(Tag::ExtXKey))
        } else if s.starts_with(tags::ExtXMap::PREFIX) {
            (s.parse().map(Tag::ExtXMap))
        } else if s.starts_with(tags::ExtXProgramDateTime::PREFIX) {
            (s.parse().map(Tag::ExtXProgramDateTime))
        } else if s.starts_with(tags::ExtXTargetDuration::PREFIX) {
            (s.parse().map(Tag::ExtXTargetDuration))
        } else if s.starts_with(tags::ExtXDateRange::PREFIX) {
            (s.parse().map(Tag::ExtXDateRange))
        } else if s.starts_with(tags::ExtXMediaSequence::PREFIX) {
            (s.parse().map(Tag::ExtXMediaSequence))
        } else if s.starts_with(tags::ExtXDiscontinuitySequence::PREFIX) {
            (s.parse().map(Tag::ExtXDiscontinuitySequence))
        } else if s.starts_with(tags::ExtXEndList::PREFIX) {
            (s.parse().map(Tag::ExtXEndList))
        } else if s.starts_with(tags::ExtXPlaylistType::PREFIX) {
            (s.parse().map(Tag::ExtXPlaylistType))
        } else if s.starts_with(tags::ExtXIFramesOnly::PREFIX) {
            (s.parse().map(Tag::ExtXIFramesOnly))
        } else if s.starts_with(tags::ExtXMedia::PREFIX) {
            (s.parse().map(Tag::ExtXMedia))
        } else if s.starts_with(tags::ExtXStreamInf::PREFIX) {
            (s.parse().map(Tag::ExtXStreamInf))
        } else if s.starts_with(tags::ExtXIFrameStreamInf::PREFIX) {
            (s.parse().map(Tag::ExtXIFrameStreamInf))
        } else if s.starts_with(tags::ExtXSessionData::PREFIX) {
            (s.parse().map(Tag::ExtXSessionData))
        } else if s.starts_with(tags::ExtXSessionKey::PREFIX) {
            (s.parse().map(Tag::ExtXSessionKey))
        } else if s.starts_with(tags::ExtXIndependentSegments::PREFIX) {
            (s.parse().map(Tag::ExtXIndependentSegments))
        } else if s.starts_with(tags::ExtXStart::PREFIX) {
            (s.parse().map(Tag::ExtXStart))
        } else {
            SingleLineString::new(s).map(Tag::Unknown)
        }
    }
}
