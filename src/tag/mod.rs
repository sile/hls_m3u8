//! [4.3. Playlist Tags]
//!
//! [4.3. Playlist Tags]: https://tools.ietf.org/html/rfc8216#section-4.3
use std::fmt;
use std::str::FromStr;

use {Error, ErrorKind, Result};

macro_rules! may_invalid {
    ($expr:expr) => {
        $expr.map_err(|e| track!(Error::from(ErrorKind::InvalidInput.cause(e))))
    }
}

macro_rules! impl_from {
    ($to:ident, $from:ident) => {
        impl From<$from> for $to {
            fn from(f: $from) -> Self {
                $to::$from(f)
            }
        }
    }
}

pub use self::basic::{ExtM3u, ExtXVersion};
pub use self::master_playlist::{ExtXIFrameStreamInf, ExtXMedia, ExtXSessionData, ExtXSessionKey,
                                ExtXStreamInf};
pub use self::media_or_master_playlist::{ExtXIndependentSegments, ExtXStart};
pub use self::media_playlist::{ExtXDiscontinuitySequence, ExtXEndList, ExtXIFramesOnly,
                               ExtXMediaSequence, ExtXPlaylistType, ExtXTargetDuration};
pub use self::media_segment::{ExtInf, ExtXByteRange, ExtXDateRange, ExtXDiscontinuity, ExtXKey,
                              ExtXMap, ExtXProgramDateTime};

mod basic;
mod master_playlist;
mod media_or_master_playlist;
mod media_playlist;
mod media_segment;

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaSegmentTag {
    ExtInf(ExtInf),
    ExtXByteRange(ExtXByteRange),
    ExtXDateRange(ExtXDateRange),
    ExtXDiscontinuity(ExtXDiscontinuity),
    ExtXKey(ExtXKey),
    ExtXMap(ExtXMap),
    ExtXProgramDateTime(ExtXProgramDateTime),
}
// TODO: delete
#[allow(missing_docs)]
impl MediaSegmentTag {
    pub fn as_inf(&self) -> Option<&ExtInf> {
        if let MediaSegmentTag::ExtInf(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
    pub fn as_byte_range(&self) -> Option<&ExtXByteRange> {
        if let MediaSegmentTag::ExtXByteRange(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
    pub fn as_date_range(&self) -> Option<&ExtXDateRange> {
        if let MediaSegmentTag::ExtXDateRange(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
    pub fn as_discontinuity(&self) -> Option<&ExtXDiscontinuity> {
        if let MediaSegmentTag::ExtXDiscontinuity(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
    pub fn as_key(&self) -> Option<&ExtXKey> {
        if let MediaSegmentTag::ExtXKey(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
    pub fn as_map(&self) -> Option<&ExtXMap> {
        if let MediaSegmentTag::ExtXMap(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
    pub fn as_program_date_time(&self) -> Option<&ExtXProgramDateTime> {
        if let MediaSegmentTag::ExtXProgramDateTime(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
}
impl fmt::Display for MediaSegmentTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MediaSegmentTag::ExtInf(ref t) => t.fmt(f),
            MediaSegmentTag::ExtXByteRange(ref t) => t.fmt(f),
            MediaSegmentTag::ExtXDateRange(ref t) => t.fmt(f),
            MediaSegmentTag::ExtXDiscontinuity(ref t) => t.fmt(f),
            MediaSegmentTag::ExtXKey(ref t) => t.fmt(f),
            MediaSegmentTag::ExtXMap(ref t) => t.fmt(f),
            MediaSegmentTag::ExtXProgramDateTime(ref t) => t.fmt(f),
        }
    }
}
impl_from!(MediaSegmentTag, ExtInf);
impl_from!(MediaSegmentTag, ExtXByteRange);
impl_from!(MediaSegmentTag, ExtXDateRange);
impl_from!(MediaSegmentTag, ExtXDiscontinuity);
impl_from!(MediaSegmentTag, ExtXKey);
impl_from!(MediaSegmentTag, ExtXMap);
impl_from!(MediaSegmentTag, ExtXProgramDateTime);

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    ExtM3u(ExtM3u),
    ExtXVersion(ExtXVersion),
    ExtInf(ExtInf),
    ExtXByteRange(ExtXByteRange),
    ExtXDiscontinuity(ExtXDiscontinuity),
    ExtXKey(ExtXKey),
    ExtXMap(ExtXMap),
    ExtXProgramDateTime(ExtXProgramDateTime),
    ExtXDateRange(ExtXDateRange),
    ExtXTargetDuration(ExtXTargetDuration),
    ExtXMediaSequence(ExtXMediaSequence),
    ExtXDiscontinuitySequence(ExtXDiscontinuitySequence),
    ExtXEndList(ExtXEndList),
    ExtXPlaylistType(ExtXPlaylistType),
    ExtXIFramesOnly(ExtXIFramesOnly),
    ExtXMedia(ExtXMedia),
    ExtXStreamInf(ExtXStreamInf),
    ExtXIFrameStreamInf(ExtXIFrameStreamInf),
    ExtXSessionData(ExtXSessionData),
    ExtXSessionKey(ExtXSessionKey),
    ExtXIndependentSegments(ExtXIndependentSegments),
    ExtXStart(ExtXStart),
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
        }
    }
}
impl FromStr for Tag {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with(ExtM3u::PREFIX) {
            track!(s.parse().map(Tag::ExtM3u))
        } else if s.starts_with(ExtXVersion::PREFIX) {
            track!(s.parse().map(Tag::ExtXVersion))
        } else if s.starts_with(ExtInf::PREFIX) {
            track!(s.parse().map(Tag::ExtInf))
        } else if s.starts_with(ExtXByteRange::PREFIX) {
            track!(s.parse().map(Tag::ExtXByteRange))
        } else if s.starts_with(ExtXDiscontinuity::PREFIX) {
            track!(s.parse().map(Tag::ExtXDiscontinuity))
        } else if s.starts_with(ExtXKey::PREFIX) {
            track!(s.parse().map(Tag::ExtXKey))
        } else if s.starts_with(ExtXMap::PREFIX) {
            track!(s.parse().map(Tag::ExtXMap))
        } else if s.starts_with(ExtXProgramDateTime::PREFIX) {
            track!(s.parse().map(Tag::ExtXProgramDateTime))
        } else if s.starts_with(ExtXTargetDuration::PREFIX) {
            track!(s.parse().map(Tag::ExtXTargetDuration))
        } else if s.starts_with(ExtXDateRange::PREFIX) {
            track!(s.parse().map(Tag::ExtXDateRange))
        } else if s.starts_with(ExtXMediaSequence::PREFIX) {
            track!(s.parse().map(Tag::ExtXMediaSequence))
        } else if s.starts_with(ExtXDiscontinuitySequence::PREFIX) {
            track!(s.parse().map(Tag::ExtXDiscontinuitySequence))
        } else if s.starts_with(ExtXEndList::PREFIX) {
            track!(s.parse().map(Tag::ExtXEndList))
        } else if s.starts_with(ExtXPlaylistType::PREFIX) {
            track!(s.parse().map(Tag::ExtXPlaylistType))
        } else if s.starts_with(ExtXIFramesOnly::PREFIX) {
            track!(s.parse().map(Tag::ExtXIFramesOnly))
        } else if s.starts_with(ExtXMedia::PREFIX) {
            track!(s.parse().map(Tag::ExtXMedia))
        } else if s.starts_with(ExtXStreamInf::PREFIX) {
            track!(s.parse().map(Tag::ExtXStreamInf))
        } else if s.starts_with(ExtXIFrameStreamInf::PREFIX) {
            track!(s.parse().map(Tag::ExtXIFrameStreamInf))
        } else if s.starts_with(ExtXSessionData::PREFIX) {
            track!(s.parse().map(Tag::ExtXSessionData))
        } else if s.starts_with(ExtXSessionKey::PREFIX) {
            track!(s.parse().map(Tag::ExtXSessionKey))
        } else if s.starts_with(ExtXIndependentSegments::PREFIX) {
            track!(s.parse().map(Tag::ExtXIndependentSegments))
        } else if s.starts_with(ExtXStart::PREFIX) {
            track!(s.parse().map(Tag::ExtXStart))
        } else {
            // TODO: ignore any unrecognized tags. (section-6.3.1)
            track_panic!(ErrorKind::InvalidInput, "Unknown tag: {:?}", s)
        }
    }
}

fn parse_yes_or_no(s: &str) -> Result<bool> {
    match s {
        "YES" => Ok(true),
        "NO" => Ok(false),
        _ => track_panic!(ErrorKind::InvalidInput, "Unexpected value: {:?}", s),
    }
}
