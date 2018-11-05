use std::fmt;
use std::iter;
use std::str::FromStr;
use std::time::Duration;

use line::{Line, Lines, Tag};
use media_segment::{MediaSegment, MediaSegmentBuilder};
use tags::{
    ExtM3u, ExtXDiscontinuitySequence, ExtXEndList, ExtXIFramesOnly, ExtXIndependentSegments,
    ExtXMediaSequence, ExtXPlaylistType, ExtXStart, ExtXTargetDuration, ExtXVersion,
    MediaPlaylistTag,
};
use types::ProtocolVersion;
use {Error, ErrorKind, Result};

/// Media playlist builder.
#[derive(Debug, Clone)]
pub struct MediaPlaylistBuilder {
    version: Option<ProtocolVersion>,
    target_duration_tag: Option<ExtXTargetDuration>,
    media_sequence_tag: Option<ExtXMediaSequence>,
    discontinuity_sequence_tag: Option<ExtXDiscontinuitySequence>,
    playlist_type_tag: Option<ExtXPlaylistType>,
    i_frames_only_tag: Option<ExtXIFramesOnly>,
    independent_segments_tag: Option<ExtXIndependentSegments>,
    start_tag: Option<ExtXStart>,
    end_list_tag: Option<ExtXEndList>,
    segments: Vec<MediaSegment>,
    options: MediaPlaylistOptions,
}
impl MediaPlaylistBuilder {
    /// Makes a new `MediaPlaylistBuilder` instance.
    pub fn new() -> Self {
        MediaPlaylistBuilder {
            version: None,
            target_duration_tag: None,
            media_sequence_tag: None,
            discontinuity_sequence_tag: None,
            playlist_type_tag: None,
            i_frames_only_tag: None,
            independent_segments_tag: None,
            start_tag: None,
            end_list_tag: None,
            segments: Vec::new(),
            options: MediaPlaylistOptions::new(),
        }
    }

    /// Sets the protocol compatibility version of the resulting playlist.
    ///
    /// If the resulting playlist has tags which requires a compatibility version greater than `version`,
    /// `finish()` method will fail with an `ErrorKind::InvalidInput` error.
    ///
    /// The default is the maximum version among the tags in the playlist.
    pub fn version(&mut self, version: ProtocolVersion) -> &mut Self {
        self.version = Some(version);
        self
    }
    /// Sets the given tag to the resulting playlist.
    pub fn tag<T: Into<MediaPlaylistTag>>(&mut self, tag: T) -> &mut Self {
        match tag.into() {
            MediaPlaylistTag::ExtXTargetDuration(t) => self.target_duration_tag = Some(t),
            MediaPlaylistTag::ExtXMediaSequence(t) => self.media_sequence_tag = Some(t),
            MediaPlaylistTag::ExtXDiscontinuitySequence(t) => {
                self.discontinuity_sequence_tag = Some(t)
            }
            MediaPlaylistTag::ExtXPlaylistType(t) => self.playlist_type_tag = Some(t),
            MediaPlaylistTag::ExtXIFramesOnly(t) => self.i_frames_only_tag = Some(t),
            MediaPlaylistTag::ExtXIndependentSegments(t) => self.independent_segments_tag = Some(t),
            MediaPlaylistTag::ExtXStart(t) => self.start_tag = Some(t),
            MediaPlaylistTag::ExtXEndList(t) => self.end_list_tag = Some(t),
        }
        self
    }

    /// Adds a media segment to the resulting playlist.
    pub fn segment(&mut self, segment: MediaSegment) -> &mut Self {
        self.segments.push(segment);
        self
    }

    /// Sets the options that will be associated to the resulting playlist.
    ///
    /// The default value is `MediaPlaylistOptions::default()`.
    pub fn options(&mut self, options: MediaPlaylistOptions) -> &mut Self {
        self.options = options;
        self
    }

    /// Builds a `MediaPlaylist` instance.
    pub fn finish(self) -> Result<MediaPlaylist> {
        let required_version = self.required_version();
        let specified_version = self.version.unwrap_or(required_version);
        track_assert!(
            required_version <= specified_version,
            ErrorKind::InvalidInput,
            "required_version:{}, specified_version:{}",
            required_version,
            specified_version,
        );

        let target_duration_tag =
            track_assert_some!(self.target_duration_tag, ErrorKind::InvalidInput);
        track!(self.validate_media_segments(target_duration_tag.duration()))?;

        Ok(MediaPlaylist {
            version_tag: ExtXVersion::new(specified_version),
            target_duration_tag,
            media_sequence_tag: self.media_sequence_tag,
            discontinuity_sequence_tag: self.discontinuity_sequence_tag,
            playlist_type_tag: self.playlist_type_tag,
            i_frames_only_tag: self.i_frames_only_tag,
            independent_segments_tag: self.independent_segments_tag,
            start_tag: self.start_tag,
            end_list_tag: self.end_list_tag,
            segments: self.segments,
        })
    }

    fn validate_media_segments(&self, target_duration: Duration) -> Result<()> {
        let mut last_range_uri = None;
        for s in &self.segments {
            // CHECK: `#EXT-X-TARGETDURATION`
            let mut segment_duration = s.inf_tag().duration();
            let rounded_segment_duration = if segment_duration.subsec_nanos() < 500_000_000 {
                Duration::from_secs(segment_duration.as_secs())
            } else {
                Duration::from_secs(segment_duration.as_secs() + 1)
            };
            let max_segment_duration = target_duration + self.options.allowable_excess_duration;
            track_assert!(
                rounded_segment_duration <= max_segment_duration,
                ErrorKind::InvalidInput,
                "Too large segment duration: actual={:?}, max={:?}, target_duration={:?}, uri={:?}",
                segment_duration,
                max_segment_duration,
                target_duration,
                s.uri()
            );

            // CHECK: `#EXT-X-BYTE-RANGE`
            if let Some(tag) = s.byte_range_tag() {
                if tag.range().start.is_none() {
                    let last_uri = track_assert_some!(last_range_uri, ErrorKind::InvalidInput);
                    track_assert_eq!(last_uri, s.uri(), ErrorKind::InvalidInput);
                } else {
                    last_range_uri = Some(s.uri());
                }
            } else {
                last_range_uri = None;
            }
        }
        Ok(())
    }

    fn required_version(&self) -> ProtocolVersion {
        iter::empty()
            .chain(
                self.target_duration_tag
                    .iter()
                    .map(|t| t.requires_version()),
            ).chain(self.media_sequence_tag.iter().map(|t| t.requires_version()))
            .chain(
                self.discontinuity_sequence_tag
                    .iter()
                    .map(|t| t.requires_version()),
            ).chain(self.playlist_type_tag.iter().map(|t| t.requires_version()))
            .chain(self.i_frames_only_tag.iter().map(|t| t.requires_version()))
            .chain(
                self.independent_segments_tag
                    .iter()
                    .map(|t| t.requires_version()),
            ).chain(self.start_tag.iter().map(|t| t.requires_version()))
            .chain(self.end_list_tag.iter().map(|t| t.requires_version()))
            .chain(self.segments.iter().map(|s| s.requires_version()))
            .max()
            .unwrap_or(ProtocolVersion::V1)
    }
}
impl Default for MediaPlaylistBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Media playlist.
#[derive(Debug, Clone)]
pub struct MediaPlaylist {
    version_tag: ExtXVersion,
    target_duration_tag: ExtXTargetDuration,
    media_sequence_tag: Option<ExtXMediaSequence>,
    discontinuity_sequence_tag: Option<ExtXDiscontinuitySequence>,
    playlist_type_tag: Option<ExtXPlaylistType>,
    i_frames_only_tag: Option<ExtXIFramesOnly>,
    independent_segments_tag: Option<ExtXIndependentSegments>,
    start_tag: Option<ExtXStart>,
    end_list_tag: Option<ExtXEndList>,
    segments: Vec<MediaSegment>,
}
impl MediaPlaylist {
    /// Returns the `EXT-X-VERSION` tag contained in the playlist.
    pub fn version_tag(&self) -> ExtXVersion {
        self.version_tag
    }

    /// Returns the `EXT-X-TARGETDURATION` tag contained in the playlist.
    pub fn target_duration_tag(&self) -> ExtXTargetDuration {
        self.target_duration_tag
    }

    /// Returns the `EXT-X-MEDIA-SEQUENCE` tag contained in the playlist.
    pub fn media_sequence_tag(&self) -> Option<ExtXMediaSequence> {
        self.media_sequence_tag
    }

    /// Returns the `EXT-X-DISCONTINUITY-SEQUENCE` tag contained in the playlist.
    pub fn discontinuity_sequence_tag(&self) -> Option<ExtXDiscontinuitySequence> {
        self.discontinuity_sequence_tag
    }

    /// Returns the `EXT-X-PLAYLIST-TYPE` tag contained in the playlist.
    pub fn playlist_type_tag(&self) -> Option<ExtXPlaylistType> {
        self.playlist_type_tag
    }

    /// Returns the `EXT-X-I-FRAMES-ONLY` tag contained in the playlist.
    pub fn i_frames_only_tag(&self) -> Option<ExtXIFramesOnly> {
        self.i_frames_only_tag
    }

    /// Returns the `EXT-X-INDEPENDENT-SEGMENTS` tag contained in the playlist.
    pub fn independent_segments_tag(&self) -> Option<ExtXIndependentSegments> {
        self.independent_segments_tag
    }

    /// Returns the `EXT-X-START` tag contained in the playlist.
    pub fn start_tag(&self) -> Option<ExtXStart> {
        self.start_tag
    }

    /// Returns the `EXT-X-ENDLIST` tag contained in the playlist.
    pub fn end_list_tag(&self) -> Option<ExtXEndList> {
        self.end_list_tag
    }

    /// Returns the media segments contained in the playlist.
    pub fn segments(&self) -> &[MediaSegment] {
        &self.segments
    }
}
impl fmt::Display for MediaPlaylist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", ExtM3u)?;
        if self.version_tag.version() != ProtocolVersion::V1 {
            writeln!(f, "{}", self.version_tag)?;
        }
        writeln!(f, "{}", self.target_duration_tag)?;
        if let Some(ref t) = self.media_sequence_tag {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.discontinuity_sequence_tag {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.playlist_type_tag {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.i_frames_only_tag {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.independent_segments_tag {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.start_tag {
            writeln!(f, "{}", t)?;
        }
        for segment in &self.segments {
            writeln!(f, "{}", segment)?;
        }
        if let Some(ref t) = self.end_list_tag {
            writeln!(f, "{}", t)?;
        }
        Ok(())
    }
}
impl FromStr for MediaPlaylist {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track!(MediaPlaylistOptions::new().parse(s))
    }
}

/// Media playlist options.
#[derive(Debug, Clone)]
pub struct MediaPlaylistOptions {
    allowable_excess_duration: Duration,
}
impl MediaPlaylistOptions {
    /// Makes a new `MediaPlaylistOptions` with the default settings.
    pub fn new() -> Self {
        MediaPlaylistOptions {
            allowable_excess_duration: Duration::from_secs(0),
        }
    }

    /// Sets the allowable excess duration of each media segment in the associated playlist.
    ///
    /// If there is a media segment of which duration exceeds
    /// `#EXT-X-TARGETDURATION + allowable_excess_duration`,
    /// the invocation of `MediaPlaylistBuilder::finish()` method will fail.
    ///
    /// The default value is `Duration::from_secs(0)`.
    pub fn allowable_excess_segment_duration(
        &mut self,
        allowable_excess_duration: Duration,
    ) -> &mut Self {
        self.allowable_excess_duration = allowable_excess_duration;
        self
    }

    /// Parses the given M3U8 text with the specified settings.
    pub fn parse(&self, m3u8: &str) -> Result<MediaPlaylist> {
        let mut builder = MediaPlaylistBuilder::new();
        builder.options(self.clone());

        let mut segment = MediaSegmentBuilder::new();
        let mut has_partial_segment = false;
        let mut has_discontinuity_tag = false;
        for (i, line) in Lines::new(m3u8).enumerate() {
            match track!(line)? {
                Line::Blank | Line::Comment(_) => {}
                Line::Tag(tag) => {
                    if i == 0 {
                        track_assert_eq!(tag, Tag::ExtM3u(ExtM3u), ErrorKind::InvalidInput);
                        continue;
                    }
                    match tag {
                        Tag::ExtM3u(_) => track_panic!(ErrorKind::InvalidInput),
                        Tag::ExtXVersion(t) => {
                            track_assert_eq!(builder.version, None, ErrorKind::InvalidInput);
                            builder.version(t.version());
                        }
                        Tag::ExtInf(t) => {
                            has_partial_segment = true;
                            segment.tag(t);
                        }
                        Tag::ExtXByteRange(t) => {
                            has_partial_segment = true;
                            segment.tag(t);
                        }
                        Tag::ExtXDiscontinuity(t) => {
                            has_discontinuity_tag = true;
                            has_partial_segment = true;
                            segment.tag(t);
                        }
                        Tag::ExtXKey(t) => {
                            has_partial_segment = true;
                            segment.tag(t);
                        }
                        Tag::ExtXMap(t) => {
                            has_partial_segment = true;
                            segment.tag(t);
                        }
                        Tag::ExtXProgramDateTime(t) => {
                            has_partial_segment = true;
                            segment.tag(t);
                        }
                        Tag::ExtXCueOut(t) => {
                            has_partial_segment = true;
                            segment.tag(t);
                        }
                        Tag::ExtXCueIn(t) => {
                            has_partial_segment = true;
                            segment.tag(t);
                        }
                        Tag::ExtXDateRange(t) => {
                            has_partial_segment = true;
                            segment.tag(t);
                        }
                        Tag::ExtXTargetDuration(t) => {
                            track_assert_eq!(
                                builder.target_duration_tag,
                                None,
                                ErrorKind::InvalidInput
                            );
                            builder.tag(t);
                        }
                        Tag::ExtXMediaSequence(t) => {
                            track_assert_eq!(
                                builder.media_sequence_tag,
                                None,
                                ErrorKind::InvalidInput
                            );
                            track_assert!(builder.segments.is_empty(), ErrorKind::InvalidInput);
                            builder.tag(t);
                        }
                        Tag::ExtXDiscontinuitySequence(t) => {
                            track_assert!(builder.segments.is_empty(), ErrorKind::InvalidInput);
                            track_assert!(!has_discontinuity_tag, ErrorKind::InvalidInput);
                            builder.tag(t);
                        }
                        Tag::ExtXEndList(t) => {
                            track_assert_eq!(builder.end_list_tag, None, ErrorKind::InvalidInput);
                            builder.tag(t);
                        }
                        Tag::ExtXPlaylistType(t) => {
                            track_assert_eq!(
                                builder.playlist_type_tag,
                                None,
                                ErrorKind::InvalidInput
                            );
                            builder.tag(t);
                        }
                        Tag::ExtXIFramesOnly(t) => {
                            track_assert_eq!(
                                builder.i_frames_only_tag,
                                None,
                                ErrorKind::InvalidInput
                            );
                            builder.tag(t);
                        }
                        Tag::ExtXMedia(_)
                        | Tag::ExtXStreamInf(_)
                        | Tag::ExtXIFrameStreamInf(_)
                        | Tag::ExtXSessionData(_)
                        | Tag::ExtXSessionKey(_) => {
                            track_panic!(ErrorKind::InvalidInput, "{}", tag)
                        }
                        Tag::ExtXIndependentSegments(t) => {
                            track_assert_eq!(
                                builder.independent_segments_tag,
                                None,
                                ErrorKind::InvalidInput
                            );
                            builder.tag(t);
                        }
                        Tag::ExtXStart(t) => {
                            track_assert_eq!(builder.start_tag, None, ErrorKind::InvalidInput);
                            builder.tag(t);
                        }
                        Tag::Unknown(_) => {
                            // [6.3.1. General Client Responsibilities]
                            // > ignore any unrecognized tags.
                        }
                    }
                }
                Line::Uri(uri) => {
                    segment.uri(uri);
                    builder.segment(track!(segment.finish())?);
                    segment = MediaSegmentBuilder::new();
                    has_partial_segment = false;
                }
            }
        }
        track_assert!(!has_partial_segment, ErrorKind::InvalidInput);
        track!(builder.finish())
    }
}
impl Default for MediaPlaylistOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_large_segment_duration_test() {
        let m3u8 = "#EXTM3U\n\
                    #EXT-X-TARGETDURATION:8\n\
                    #EXT-X-VERSION:3\n\
                    #EXTINF:9.009,\n\
                    http://media.example.com/first.ts\n\
                    #EXTINF:9.509,\n\
                    http://media.example.com/second.ts\n\
                    #EXTINF:3.003,\n\
                    http://media.example.com/third.ts\n\
                    #EXT-X-ENDLIST";

        // Error (allowable segment duration = target duration = 8)
        assert!(m3u8.parse::<MediaPlaylist>().is_err());

        // Error (allowable segment duration = 9)
        assert!(
            MediaPlaylistOptions::new()
                .allowable_excess_segment_duration(Duration::from_secs(1))
                .parse(m3u8)
                .is_err()
        );

        // Ok (allowable segment duration = 10)
        assert!(
            MediaPlaylistOptions::new()
                .allowable_excess_segment_duration(Duration::from_secs(2))
                .parse(m3u8)
                .is_ok()
        );
    }

    #[test]
    fn empty_m3u8_parse_test() {
        let m3u8 = "";
        assert!(m3u8.parse::<MediaPlaylist>().is_err());
    }
}
