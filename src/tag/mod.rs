//! [4.3. Playlist Tags]
//!
//! [4.3. Playlist Tags]: https://tools.ietf.org/html/rfc8216#section-4.3
use std::fmt;
use trackable::error::ErrorKindExt;

use {ErrorKind, Result};

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

/// [4.3.4. Master Playlist Tags]
///
/// See also [4.3.5. Media or Master Playlist Tags]
///
/// [4.3.4. Master Playlist Tags] https://tools.ietf.org/html/rfc8216#section-4.3.4
/// [4.3.5. Media or Master Playlist Tags]: https://tools.ietf.org/html/rfc8216#section-4.3.5
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MasterPlaylistTag {
    ExtXMedia(ExtXMedia),
    ExtXStreamInf(ExtXStreamInf),
    ExtXIFrameStreamInf(ExtXIFrameStreamInf),
    ExtXSessionData(ExtXSessionData),
    ExtXSessionKey(ExtXSessionKey),
    ExtXIndependentSegments(ExtXIndependentSegments),
    ExtXStart(ExtXStart),
}
impl_from!(MasterPlaylistTag, ExtXMedia);
impl_from!(MasterPlaylistTag, ExtXStreamInf);
impl_from!(MasterPlaylistTag, ExtXIFrameStreamInf);
impl_from!(MasterPlaylistTag, ExtXSessionData);
impl_from!(MasterPlaylistTag, ExtXSessionKey);
impl_from!(MasterPlaylistTag, ExtXIndependentSegments);
impl_from!(MasterPlaylistTag, ExtXStart);

/// [4.3.3. Media Playlist Tags]
///
/// See also [4.3.5. Media or Master Playlist Tags]
///
/// [4.3.3. Media Playlist Tags] https://tools.ietf.org/html/rfc8216#section-4.3.3
/// [4.3.5. Media or Master Playlist Tags]: https://tools.ietf.org/html/rfc8216#section-4.3.5
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaPlaylistTag {
    ExtXTargetDuration(ExtXTargetDuration),
    ExtXMediaSequence(ExtXMediaSequence),
    ExtXDiscontinuitySequence(ExtXDiscontinuitySequence),
    ExtXEndList(ExtXEndList),
    ExtXPlaylistType(ExtXPlaylistType),
    ExtXIFramesOnly(ExtXIFramesOnly),
    ExtXIndependentSegments(ExtXIndependentSegments),
    ExtXStart(ExtXStart),
}
impl_from!(MediaPlaylistTag, ExtXTargetDuration);
impl_from!(MediaPlaylistTag, ExtXMediaSequence);
impl_from!(MediaPlaylistTag, ExtXDiscontinuitySequence);
impl_from!(MediaPlaylistTag, ExtXEndList);
impl_from!(MediaPlaylistTag, ExtXPlaylistType);
impl_from!(MediaPlaylistTag, ExtXIFramesOnly);
impl_from!(MediaPlaylistTag, ExtXIndependentSegments);
impl_from!(MediaPlaylistTag, ExtXStart);

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

fn parse_yes_or_no(s: &str) -> Result<bool> {
    match s {
        "YES" => Ok(true),
        "NO" => Ok(false),
        _ => track_panic!(ErrorKind::InvalidInput, "Unexpected value: {:?}", s),
    }
}

fn parse_u64(s: &str) -> Result<u64> {
    let n = track!(s.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
    Ok(n)
}
