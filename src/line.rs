use std::fmt;
use std::str::FromStr;

use {Error, ErrorKind, Result};
use tag;
use types::SingleLineString;

#[derive(Debug)]
pub struct Lines<'a> {
    input: &'a str,
}
impl<'a> Lines<'a> {
    pub fn new(input: &'a str) -> Self {
        Lines { input }
    }

    fn read_line(&mut self) -> Result<Line<'a>> {
        let mut end = self.input.len();
        let mut next_start = self.input.len();
        let mut adjust = 0;
        let mut next_line_of_ext_x_stream_inf = false;
        for (i, c) in self.input.char_indices() {
            match c {
                '\n' => {
                    if !next_line_of_ext_x_stream_inf
                        && self.input.starts_with(tag::ExtXStreamInf::PREFIX)
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
                    track_assert!(!c.is_control(), ErrorKind::InvalidInput);
                    adjust = 0;
                }
            }
        }
        let raw_line = &self.input[..end];
        let line = if raw_line.is_empty() {
            Line::Blank
        } else if raw_line.starts_with("#EXT") {
            Line::Tag(track!(raw_line.parse())?)
        } else if raw_line.starts_with("#") {
            Line::Comment(raw_line)
        } else {
            let uri = track!(SingleLineString::new(raw_line))?;
            Line::Uri(uri)
        };
        self.input = &self.input[next_start..];
        Ok(line)
    }
}
impl<'a> Iterator for Lines<'a> {
    type Item = Result<Line<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }
        match track!(self.read_line()) {
            Err(e) => Some(Err(e)),
            Ok(line) => Some(Ok(line)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Line<'a> {
    Blank,
    Comment(&'a str),
    Tag(Tag),
    Uri(SingleLineString),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    ExtM3u(tag::ExtM3u),
    ExtXVersion(tag::ExtXVersion),
    ExtInf(tag::ExtInf),
    ExtXByteRange(tag::ExtXByteRange),
    ExtXDiscontinuity(tag::ExtXDiscontinuity),
    ExtXKey(tag::ExtXKey),
    ExtXMap(tag::ExtXMap),
    ExtXProgramDateTime(tag::ExtXProgramDateTime),
    ExtXDateRange(tag::ExtXDateRange),
    ExtXTargetDuration(tag::ExtXTargetDuration),
    ExtXMediaSequence(tag::ExtXMediaSequence),
    ExtXDiscontinuitySequence(tag::ExtXDiscontinuitySequence),
    ExtXEndList(tag::ExtXEndList),
    ExtXPlaylistType(tag::ExtXPlaylistType),
    ExtXIFramesOnly(tag::ExtXIFramesOnly),
    ExtXMedia(tag::ExtXMedia),
    ExtXStreamInf(tag::ExtXStreamInf),
    ExtXIFrameStreamInf(tag::ExtXIFrameStreamInf),
    ExtXSessionData(tag::ExtXSessionData),
    ExtXSessionKey(tag::ExtXSessionKey),
    ExtXIndependentSegments(tag::ExtXIndependentSegments),
    ExtXStart(tag::ExtXStart),
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
    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with(tag::ExtM3u::PREFIX) {
            track!(s.parse().map(Tag::ExtM3u))
        } else if s.starts_with(tag::ExtXVersion::PREFIX) {
            track!(s.parse().map(Tag::ExtXVersion))
        } else if s.starts_with(tag::ExtInf::PREFIX) {
            track!(s.parse().map(Tag::ExtInf))
        } else if s.starts_with(tag::ExtXByteRange::PREFIX) {
            track!(s.parse().map(Tag::ExtXByteRange))
        } else if s.starts_with(tag::ExtXDiscontinuity::PREFIX) {
            track!(s.parse().map(Tag::ExtXDiscontinuity))
        } else if s.starts_with(tag::ExtXKey::PREFIX) {
            track!(s.parse().map(Tag::ExtXKey))
        } else if s.starts_with(tag::ExtXMap::PREFIX) {
            track!(s.parse().map(Tag::ExtXMap))
        } else if s.starts_with(tag::ExtXProgramDateTime::PREFIX) {
            track!(s.parse().map(Tag::ExtXProgramDateTime))
        } else if s.starts_with(tag::ExtXTargetDuration::PREFIX) {
            track!(s.parse().map(Tag::ExtXTargetDuration))
        } else if s.starts_with(tag::ExtXDateRange::PREFIX) {
            track!(s.parse().map(Tag::ExtXDateRange))
        } else if s.starts_with(tag::ExtXMediaSequence::PREFIX) {
            track!(s.parse().map(Tag::ExtXMediaSequence))
        } else if s.starts_with(tag::ExtXDiscontinuitySequence::PREFIX) {
            track!(s.parse().map(Tag::ExtXDiscontinuitySequence))
        } else if s.starts_with(tag::ExtXEndList::PREFIX) {
            track!(s.parse().map(Tag::ExtXEndList))
        } else if s.starts_with(tag::ExtXPlaylistType::PREFIX) {
            track!(s.parse().map(Tag::ExtXPlaylistType))
        } else if s.starts_with(tag::ExtXIFramesOnly::PREFIX) {
            track!(s.parse().map(Tag::ExtXIFramesOnly))
        } else if s.starts_with(tag::ExtXMedia::PREFIX) {
            track!(s.parse().map(Tag::ExtXMedia))
        } else if s.starts_with(tag::ExtXStreamInf::PREFIX) {
            track!(s.parse().map(Tag::ExtXStreamInf))
        } else if s.starts_with(tag::ExtXIFrameStreamInf::PREFIX) {
            track!(s.parse().map(Tag::ExtXIFrameStreamInf))
        } else if s.starts_with(tag::ExtXSessionData::PREFIX) {
            track!(s.parse().map(Tag::ExtXSessionData))
        } else if s.starts_with(tag::ExtXSessionKey::PREFIX) {
            track!(s.parse().map(Tag::ExtXSessionKey))
        } else if s.starts_with(tag::ExtXIndependentSegments::PREFIX) {
            track!(s.parse().map(Tag::ExtXIndependentSegments))
        } else if s.starts_with(tag::ExtXStart::PREFIX) {
            track!(s.parse().map(Tag::ExtXStart))
        } else {
            track!(SingleLineString::new(s)).map(Tag::Unknown)
        }
    }
}
