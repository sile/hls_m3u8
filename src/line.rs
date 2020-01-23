use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use crate::tags;
use crate::Error;

#[derive(Debug, Default)]
pub struct Lines(Vec<Line>);

impl Lines {
    pub fn new() -> Self { Self::default() }
}

impl FromStr for Lines {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut result = Self::new();

        let mut stream_inf = false;
        let mut stream_inf_line = None;

        for l in input.lines() {
            let raw_line = l.trim();

            if raw_line.is_empty() {
                continue;
            }

            let line = {
                if raw_line.starts_with(tags::ExtXStreamInf::PREFIX) {
                    stream_inf = true;
                    stream_inf_line = Some(raw_line);

                    continue;
                } else if raw_line.starts_with("#EXT") {
                    Line::Tag(raw_line.parse()?)
                } else if raw_line.starts_with('#') {
                    continue; // ignore comments
                } else {
                    // stream inf line needs special treatment
                    if stream_inf {
                        stream_inf = false;
                        if let Some(first_line) = stream_inf_line {
                            let res = Line::Tag(format!("{}\n{}", first_line, raw_line).parse()?);
                            stream_inf_line = None;
                            res
                        } else {
                            continue;
                        }
                    } else {
                        Line::Uri(raw_line.to_string())
                    }
                }
            };

            result.push(line);
        }

        Ok(result)
    }
}

impl IntoIterator for Lines {
    type IntoIter = ::std::vec::IntoIter<Line>;
    type Item = Line;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl Deref for Lines {
    type Target = Vec<Line>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Lines {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Line {
    Tag(Tag),
    Uri(String),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
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
            Self::ExtM3u(value) => value.fmt(f),
            Self::ExtXVersion(value) => value.fmt(f),
            Self::ExtInf(value) => value.fmt(f),
            Self::ExtXByteRange(value) => value.fmt(f),
            Self::ExtXDiscontinuity(value) => value.fmt(f),
            Self::ExtXKey(value) => value.fmt(f),
            Self::ExtXMap(value) => value.fmt(f),
            Self::ExtXProgramDateTime(value) => value.fmt(f),
            Self::ExtXDateRange(value) => value.fmt(f),
            Self::ExtXTargetDuration(value) => value.fmt(f),
            Self::ExtXMediaSequence(value) => value.fmt(f),
            Self::ExtXDiscontinuitySequence(value) => value.fmt(f),
            Self::ExtXEndList(value) => value.fmt(f),
            Self::ExtXPlaylistType(value) => value.fmt(f),
            Self::ExtXIFramesOnly(value) => value.fmt(f),
            Self::ExtXMedia(value) => value.fmt(f),
            Self::ExtXStreamInf(value) => value.fmt(f),
            Self::ExtXIFrameStreamInf(value) => value.fmt(f),
            Self::ExtXSessionData(value) => value.fmt(f),
            Self::ExtXSessionKey(value) => value.fmt(f),
            Self::ExtXIndependentSegments(value) => value.fmt(f),
            Self::ExtXStart(value) => value.fmt(f),
            Self::Unknown(value) => value.fmt(f),
        }
    }
}

impl FromStr for Tag {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.starts_with(tags::ExtM3u::PREFIX) {
            input.parse().map(Self::ExtM3u)
        } else if input.starts_with(tags::ExtXVersion::PREFIX) {
            input.parse().map(Self::ExtXVersion)
        } else if input.starts_with(tags::ExtInf::PREFIX) {
            input.parse().map(Self::ExtInf)
        } else if input.starts_with(tags::ExtXByteRange::PREFIX) {
            input.parse().map(Self::ExtXByteRange)
        } else if input.starts_with(tags::ExtXDiscontinuity::PREFIX) {
            input.parse().map(Self::ExtXDiscontinuity)
        } else if input.starts_with(tags::ExtXKey::PREFIX) {
            input.parse().map(Self::ExtXKey)
        } else if input.starts_with(tags::ExtXMap::PREFIX) {
            input.parse().map(Self::ExtXMap)
        } else if input.starts_with(tags::ExtXProgramDateTime::PREFIX) {
            input.parse().map(Self::ExtXProgramDateTime)
        } else if input.starts_with(tags::ExtXTargetDuration::PREFIX) {
            input.parse().map(Self::ExtXTargetDuration)
        } else if input.starts_with(tags::ExtXDateRange::PREFIX) {
            input.parse().map(Self::ExtXDateRange)
        } else if input.starts_with(tags::ExtXMediaSequence::PREFIX) {
            input.parse().map(Self::ExtXMediaSequence)
        } else if input.starts_with(tags::ExtXDiscontinuitySequence::PREFIX) {
            input.parse().map(Self::ExtXDiscontinuitySequence)
        } else if input.starts_with(tags::ExtXEndList::PREFIX) {
            input.parse().map(Self::ExtXEndList)
        } else if input.starts_with(tags::ExtXPlaylistType::PREFIX) {
            input.parse().map(Self::ExtXPlaylistType)
        } else if input.starts_with(tags::ExtXIFramesOnly::PREFIX) {
            input.parse().map(Self::ExtXIFramesOnly)
        } else if input.starts_with(tags::ExtXMedia::PREFIX) {
            input.parse().map(Self::ExtXMedia).map_err(Error::custom)
        } else if input.starts_with(tags::ExtXStreamInf::PREFIX) {
            input.parse().map(Self::ExtXStreamInf)
        } else if input.starts_with(tags::ExtXIFrameStreamInf::PREFIX) {
            input.parse().map(Self::ExtXIFrameStreamInf)
        } else if input.starts_with(tags::ExtXSessionData::PREFIX) {
            input.parse().map(Self::ExtXSessionData)
        } else if input.starts_with(tags::ExtXSessionKey::PREFIX) {
            input.parse().map(Self::ExtXSessionKey)
        } else if input.starts_with(tags::ExtXIndependentSegments::PREFIX) {
            input.parse().map(Self::ExtXIndependentSegments)
        } else if input.starts_with(tags::ExtXStart::PREFIX) {
            input.parse().map(Self::ExtXStart)
        } else {
            Ok(Self::Unknown(input.to_string()))
        }
    }
}
