//! [4.3. Playlist Tags]
//!
//! [4.3. Playlist Tags]: https://tools.ietf.org/html/rfc8216#section-4.3

macro_rules! may_invalid {
    ($expr:expr) => {
        $expr.map_err(|e| track!(Error::from(ErrorKind::InvalidInput.cause(e))))
    };
}

macro_rules! impl_from {
    ($to:ident, $from:ident) => {
        impl From<$from> for $to {
            fn from(f: $from) -> Self {
                $to::$from(f)
            }
        }
    };
}

// new mods:
mod byte_range;
mod date_range;
mod discontinuity;
mod discontinuity_sequence;
mod end_list;
mod iframe_stream_inf;
mod iframes_only;
mod independent_segments;
mod inf;
mod key;
mod m3u;
mod map;
mod media;
mod media_sequence;
mod playlist_type;
mod program_date_time;
mod session_data;
mod session_key;
mod start;
mod stream_inf;
mod target_duration;
mod version;

pub use byte_range::*;
pub use date_range::*;
pub use discontinuity::*;
pub use discontinuity_sequence::*;
pub use end_list::*;
pub use iframe_stream_inf::*;
pub use iframes_only::*;
pub use independent_segments::*;
pub use inf::*;
pub use key::*;
pub use m3u::*;
pub use map::*;
pub use media::*;
pub use media_sequence::*;
pub use playlist_type::*;
pub use program_date_time::*;
pub use session_data::*;
pub use session_key::*;
pub use start::*;
pub use stream_inf::*;
pub use target_duration::*;
pub use version::*;

/// [4.3.4. Master Playlist Tags]
///
/// See also [4.3.5. Media or Master Playlist Tags]
///
/// [4.3.4. Master Playlist Tags]: https://tools.ietf.org/html/rfc8216#section-4.3.4
/// [4.3.5. Media or Master Playlist Tags]: https://tools.ietf.org/html/rfc8216#section-4.3.5
#[allow(missing_docs)]
#[allow(clippy::large_enum_variant)]
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
/// [4.3.3. Media Playlist Tags]: https://tools.ietf.org/html/rfc8216#section-4.3.3
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

/// [4.3.2. Media Segment Tags]
///
/// [4.3.2. Media Segment Tags]: https://tools.ietf.org/html/rfc8216#section-4.3.2
#[allow(missing_docs)]
#[allow(clippy::large_enum_variant)]
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

impl_from!(MediaSegmentTag, ExtInf);
impl_from!(MediaSegmentTag, ExtXByteRange);
impl_from!(MediaSegmentTag, ExtXDateRange);
impl_from!(MediaSegmentTag, ExtXDiscontinuity);
impl_from!(MediaSegmentTag, ExtXKey);
impl_from!(MediaSegmentTag, ExtXMap);
impl_from!(MediaSegmentTag, ExtXProgramDateTime);
