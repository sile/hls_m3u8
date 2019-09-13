use std::fmt;
use std::str::FromStr;

use crate::tags;
use crate::types::SingleLineString;
use crate::Error;

#[derive(Debug)]
pub struct Lines<'a> {
    input: &'a str,
}
impl<'a> Lines<'a> {
    pub const fn new(input: &'a str) -> Self {
        Lines { input }
    }

    fn read_line(&mut self) -> crate::Result<Line<'a>> {
        let mut end = self.input.len();
        let mut next_start = self.input.len();
        let mut adjust = 0;
        let mut next_line_of_ext_x_stream_inf = false;

        for (i, c) in self.input.char_indices() {
            match c {
                '\n' => {
                    if !next_line_of_ext_x_stream_inf
                        && self.input.starts_with(tags::ExtXStreamInf::PREFIX)
                    {
                        next_line_of_ext_x_stream_inf = true;
                        adjust = 0;
                        continue;
                    }
                    next_start = i + 1;
                    end = i - adjust;
                    break;
                }
                '\r' => {
                    adjust = 1;
                }
                _ => {
                    if c.is_control() {
                        return Err(Error::invalid_input());
                    }
                    adjust = 0;
                }
            }
        }
        let raw_line = &self.input[..end];

        let line = if raw_line.is_empty() {
            Line::Blank
        } else if raw_line.starts_with("#EXT") {
            Line::Tag((raw_line.parse())?)
        } else if raw_line.starts_with('#') {
            Line::Comment(raw_line)
        } else {
            let uri = SingleLineString::new(raw_line)?;
            Line::Uri(uri)
        };

        self.input = &self.input[next_start..];
        Ok(line)
    }
}
impl<'a> Iterator for Lines<'a> {
    type Item = crate::Result<Line<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        match self.read_line() {
            Err(e) => Some(Err(e)),
            Ok(line) => Some(Ok(line)),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq)]
pub enum Line<'a> {
    Blank,
    Comment(&'a str),
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
