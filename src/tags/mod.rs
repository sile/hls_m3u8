//! [4.3. Playlist Tags]
//!
//! [4.3. Playlist Tags]: https://tools.ietf.org/html/rfc8216#section-4.3

macro_rules! impl_from {
    ($to:ident, $from:ident) => {
        impl From<$from> for $to {
            fn from(f: $from) -> Self {
                $to::$from(f)
            }
        }
    };
}

mod basic;
mod master_playlist;
mod media_playlist;
mod media_segment;
mod shared;

pub use basic::*;
pub use master_playlist::*;
pub use media_playlist::*;
pub use media_segment::*;
pub use shared::*;

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
