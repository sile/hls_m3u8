use std::fmt;
use std::str::FromStr;

use {Error, ErrorKind, Result};
use line::{Line, Lines};
use media_segment::{MediaSegment, MediaSegmentBuilder};
use tag::{ExtM3u, ExtXDiscontinuitySequence, ExtXEndList, ExtXIFramesOnly,
          ExtXIndependentSegments, ExtXMediaSequence, ExtXPlaylistType, ExtXStart,
          ExtXTargetDuration, ExtXVersion, Tag};
use version::ProtocolVersion;

// TODO: There MUST NOT be more than one Media Playlist tag of each type in any Media Playlist.
// TODO: A Media Playlist tag MUST NOT appear in a Master Playlist.
#[derive(Debug, Clone)]
pub struct MediaPlaylist {
    pub version: ExtXVersion,

    // TODO:  The EXTINF duration of each Media Segment in the Playlist
    // file, when rounded to the nearest integer, MUST be less than or equal
    // to the target duration
    pub target_duration: ExtXTargetDuration,

    // TODO: The EXT-X-MEDIA-SEQUENCE tag MUST appear before the first Media
    // Segment in the Playlist.
    pub media_sequence: Option<ExtXMediaSequence>,

    // TODO: The EXT-X-DISCONTINUITY-SEQUENCE tag MUST appear before the first
    // Media Segment in the Playlist.
    //
    // TODO: The EXT-X-DISCONTINUITY-SEQUENCE tag MUST appear before any EXT-
    // X-DISCONTINUITY tag.
    pub discontinuity_sequence: Option<ExtXDiscontinuitySequence>,

    pub playlist_type: Option<ExtXPlaylistType>,
    pub i_frames_only: Option<ExtXIFramesOnly>,
    pub independent_segments: Option<ExtXIndependentSegments>,
    pub start: Option<ExtXStart>,

    pub segments: Vec<MediaSegment>,

    pub end_list: Option<ExtXEndList>,
}
impl fmt::Display for MediaPlaylist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", ExtM3u)?;
        if self.version.value() != ProtocolVersion::V1 {
            writeln!(f, "{}", self.version)?;
        }
        writeln!(f, "{}", self.target_duration)?;
        if let Some(ref t) = self.media_sequence {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.discontinuity_sequence {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.playlist_type {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.i_frames_only {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.independent_segments {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.start {
            writeln!(f, "{}", t)?;
        }
        for segment in &self.segments {
            writeln!(f, "{}", segment)?;
        }
        if let Some(ref t) = self.end_list {
            writeln!(f, "{}", t)?;
        }
        Ok(())
    }
}
impl FromStr for MediaPlaylist {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut version = None;
        let mut target_duration = None;
        let mut media_sequence = None;
        let mut discontinuity_sequence = None;
        let mut playlist_type = None;
        let mut i_frames_only = None;
        let mut independent_segments = None;
        let mut start = None;
        let mut end_list = None;

        let mut segment = MediaSegmentBuilder::new();
        let mut segments = Vec::new();
        for (i, line) in Lines::new(s).enumerate() {
            match track!(line)? {
                Line::Blank | Line::Comment(_) => {}
                Line::Tag(tag) => {
                    if i == 0 {
                        track_assert_eq!(tag, Tag::ExtM3u(ExtM3u), ErrorKind::InvalidInput);
                        continue;
                    }
                    match tag {
                        Tag::ExtM3u(_) => unreachable!(),
                        Tag::ExtXVersion(t) => {
                            track_assert_eq!(version, None, ErrorKind::InvalidInput);
                            version = Some(t);
                        }
                        Tag::ExtInf(t) => {
                            segment.tag(t);
                        }
                        Tag::ExtXByteRange(t) => {
                            segment.tag(t);
                        }
                        Tag::ExtXDiscontinuity(t) => {
                            segment.tag(t);
                        }
                        Tag::ExtXKey(t) => {
                            segment.tag(t);
                        }
                        Tag::ExtXMap(t) => {
                            segment.tag(t);
                        }
                        Tag::ExtXProgramDateTime(t) => {
                            segment.tag(t);
                        }
                        Tag::ExtXDateRange(t) => {
                            segment.tag(t);
                        }
                        Tag::ExtXTargetDuration(t) => {
                            track_assert_eq!(target_duration, None, ErrorKind::InvalidInput);
                            target_duration = Some(t);
                        }
                        Tag::ExtXMediaSequence(t) => {
                            track_assert_eq!(media_sequence, None, ErrorKind::InvalidInput);
                            media_sequence = Some(t);
                        }
                        Tag::ExtXDiscontinuitySequence(t) => {
                            track_assert_eq!(discontinuity_sequence, None, ErrorKind::InvalidInput);
                            discontinuity_sequence = Some(t);
                        }
                        Tag::ExtXEndList(t) => {
                            track_assert_eq!(end_list, None, ErrorKind::InvalidInput);
                            end_list = Some(t);
                        }
                        Tag::ExtXPlaylistType(t) => {
                            track_assert_eq!(playlist_type, None, ErrorKind::InvalidInput);
                            playlist_type = Some(t);
                        }
                        Tag::ExtXIFramesOnly(t) => {
                            track_assert_eq!(i_frames_only, None, ErrorKind::InvalidInput);
                            i_frames_only = Some(t);
                        }
                        Tag::ExtXMedia(_)
                        | Tag::ExtXStreamInf(_)
                        | Tag::ExtXIFrameStreamInf(_)
                        | Tag::ExtXSessionData(_)
                        | Tag::ExtXSessionKey(_) => {
                            track_panic!(ErrorKind::InvalidInput, "{}", tag)
                        }
                        Tag::ExtXIndependentSegments(t) => {
                            track_assert_eq!(independent_segments, None, ErrorKind::InvalidInput);
                            independent_segments = Some(t);
                        }
                        Tag::ExtXStart(t) => {
                            track_assert_eq!(start, None, ErrorKind::InvalidInput);
                            start = Some(t);
                        }
                    }
                }
                Line::Uri(uri) => {
                    segment.uri(uri.to_owned());
                    segments.push(track!(segment.finish())?);
                    segment = MediaSegmentBuilder::new();
                }
            }
        }

        let target_duration = track_assert_some!(target_duration, ErrorKind::InvalidInput);
        // TODO: check compatibility
        Ok(MediaPlaylist {
            version: version.unwrap_or_else(|| ExtXVersion::new(ProtocolVersion::V1)),
            target_duration,
            media_sequence,
            discontinuity_sequence,
            playlist_type,
            i_frames_only,
            independent_segments,
            start,
            segments,
            end_list,
        })
    }
}
