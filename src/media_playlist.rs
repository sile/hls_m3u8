use std::fmt;
use std::iter;
use std::str::FromStr;
use std::time::Duration;

use derive_builder::Builder;

use crate::line::{Line, Lines, Tag};
use crate::media_segment::{MediaSegment, MediaSegmentBuilder};
use crate::tags::{
    ExtM3u, ExtXDiscontinuitySequence, ExtXEndList, ExtXIFramesOnly, ExtXIndependentSegments,
    ExtXMediaSequence, ExtXPlaylistType, ExtXStart, ExtXTargetDuration, ExtXVersion,
};
use crate::types::ProtocolVersion;
use crate::Error;

/// Media playlist.
#[derive(Debug, Clone, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
#[builder(setter(into, strip_option))]
pub struct MediaPlaylist {
    /// Sets the protocol compatibility version of the resulting playlist.
    ///
    /// If the resulting playlist has tags which requires a compatibility
    /// version greater than `version`,
    /// `build()` method will fail with an `ErrorKind::InvalidInput` error.
    ///
    /// The default is the maximum version among the tags in the playlist.
    #[builder(setter(name = "version"))]
    version_tag: ExtXVersion,
    /// Sets the [ExtXTargetDuration] tag.
    target_duration_tag: ExtXTargetDuration,
    #[builder(default)]
    /// Sets the [ExtXMediaSequence] tag.
    media_sequence_tag: Option<ExtXMediaSequence>,
    #[builder(default)]
    /// Sets the [ExtXDiscontinuitySequence] tag.
    discontinuity_sequence_tag: Option<ExtXDiscontinuitySequence>,
    #[builder(default)]
    /// Sets the [ExtXPlaylistType] tag.
    playlist_type_tag: Option<ExtXPlaylistType>,
    #[builder(default)]
    /// Sets the [ExtXIFramesOnly] tag.
    i_frames_only_tag: Option<ExtXIFramesOnly>,
    #[builder(default)]
    /// Sets the [ExtXIndependentSegments] tag.
    independent_segments_tag: Option<ExtXIndependentSegments>,
    #[builder(default)]
    /// Sets the [ExtXStart] tag.
    start_tag: Option<ExtXStart>,
    #[builder(default)]
    /// Sets the [ExtXEndList] tag.
    end_list_tag: Option<ExtXEndList>,
    /// Sets all [MediaSegment]s.
    segments: Vec<MediaSegment>,
    /// Sets the allowable excess duration of each media segment in the associated playlist.
    ///
    /// # Error
    /// If there is a media segment of which duration exceeds
    /// `#EXT-X-TARGETDURATION + allowable_excess_duration`,
    /// the invocation of `MediaPlaylistBuilder::build()` method will fail.
    ///
    /// The default value is `Duration::from_secs(0)`.
    #[builder(default = "Duration::from_secs(0)")]
    allowable_excess_duration: Duration,
}

impl MediaPlaylistBuilder {
    fn validate(&self) -> Result<(), String> {
        let required_version = self.required_version();
        let specified_version = self
            .version_tag
            .unwrap_or(required_version.into())
            .version();

        if required_version > specified_version {
            return Err(Error::custom(format!(
                "required_version: {}, specified_version: {}",
                required_version, specified_version
            ))
            .to_string());
        }

        if let Some(target_duration) = &self.target_duration_tag {
            self.validate_media_segments(target_duration.duration())
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    fn validate_media_segments(&self, target_duration: Duration) -> crate::Result<()> {
        let mut last_range_uri = None;
        if let Some(segments) = &self.segments {
            for s in segments {
                // CHECK: `#EXT-X-TARGETDURATION`
                let segment_duration = s.inf_tag().duration();
                let rounded_segment_duration = if segment_duration.subsec_nanos() < 500_000_000 {
                    Duration::from_secs(segment_duration.as_secs())
                } else {
                    Duration::from_secs(segment_duration.as_secs() + 1)
                };

                let max_segment_duration = {
                    if let Some(value) = &self.allowable_excess_duration {
                        target_duration + *value
                    } else {
                        target_duration
                    }
                };

                if !(rounded_segment_duration <= max_segment_duration) {
                    return Err(Error::custom(format!(
                        "Too large segment duration: actual={:?}, max={:?}, target_duration={:?}, uri={:?}",
                        segment_duration,
                        max_segment_duration,
                        target_duration,
                        s.uri()
                    )));
                }

                // CHECK: `#EXT-X-BYTE-RANGE`
                if let Some(tag) = s.byte_range_tag() {
                    if tag.to_range().start().is_none() {
                        let last_uri = last_range_uri.ok_or(Error::invalid_input())?;
                        if last_uri != s.uri() {
                            return Err(Error::invalid_input());
                        }
                    } else {
                        last_range_uri = Some(s.uri());
                    }
                } else {
                    last_range_uri = None;
                }
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
            )
            .chain(self.media_sequence_tag.iter().map(|t| {
                if let Some(p) = t {
                    p.requires_version()
                } else {
                    ProtocolVersion::V1
                }
            }))
            .chain(self.discontinuity_sequence_tag.iter().map(|t| {
                if let Some(p) = t {
                    p.requires_version()
                } else {
                    ProtocolVersion::V1
                }
            }))
            .chain(self.playlist_type_tag.iter().map(|t| {
                if let Some(p) = t {
                    p.requires_version()
                } else {
                    ProtocolVersion::V1
                }
            }))
            .chain(self.i_frames_only_tag.iter().map(|t| {
                if let Some(p) = t {
                    p.requires_version()
                } else {
                    ProtocolVersion::V1
                }
            }))
            .chain(self.independent_segments_tag.iter().map(|t| {
                if let Some(p) = t {
                    p.requires_version()
                } else {
                    ProtocolVersion::V1
                }
            }))
            .chain(self.start_tag.iter().map(|t| {
                if let Some(p) = t {
                    p.requires_version()
                } else {
                    ProtocolVersion::V1
                }
            }))
            .chain(self.end_list_tag.iter().map(|t| {
                if let Some(p) = t {
                    p.requires_version()
                } else {
                    ProtocolVersion::V1
                }
            }))
            .chain(self.segments.iter().map(|t| {
                t.iter()
                    .map(|s| s.requires_version())
                    .max()
                    .unwrap_or(ProtocolVersion::V1)
            }))
            .max()
            .unwrap_or(ProtocolVersion::V1)
    }

    /// Adds a media segment to the resulting playlist.
    pub fn push_segment<VALUE: Into<MediaSegment>>(&mut self, value: VALUE) -> &mut Self {
        if let Some(segments) = &mut self.segments {
            segments.push(value.into());
        } else {
            self.segments = Some(vec![value.into()]);
        }
        self
    }

    /// Parse the rest of the [MediaPlaylist] from an m3u8 file.
    pub fn parse(&mut self, input: &str) -> crate::Result<MediaPlaylist> {
        parse_media_playlist(input, self)
    }
}

impl MediaPlaylist {
    /// Creates a [MediaPlaylistBuilder].
    pub fn builder() -> MediaPlaylistBuilder {
        MediaPlaylistBuilder::default()
    }
    /// Returns the `EXT-X-VERSION` tag contained in the playlist.
    pub const fn version_tag(&self) -> ExtXVersion {
        self.version_tag
    }

    /// Returns the `EXT-X-TARGETDURATION` tag contained in the playlist.
    pub const fn target_duration_tag(&self) -> ExtXTargetDuration {
        self.target_duration_tag
    }

    /// Returns the `EXT-X-MEDIA-SEQUENCE` tag contained in the playlist.
    pub const fn media_sequence_tag(&self) -> Option<ExtXMediaSequence> {
        self.media_sequence_tag
    }

    /// Returns the `EXT-X-DISCONTINUITY-SEQUENCE` tag contained in the playlist.
    pub const fn discontinuity_sequence_tag(&self) -> Option<ExtXDiscontinuitySequence> {
        self.discontinuity_sequence_tag
    }

    /// Returns the `EXT-X-PLAYLIST-TYPE` tag contained in the playlist.
    pub const fn playlist_type_tag(&self) -> Option<ExtXPlaylistType> {
        self.playlist_type_tag
    }

    /// Returns the `EXT-X-I-FRAMES-ONLY` tag contained in the playlist.
    pub const fn i_frames_only_tag(&self) -> Option<ExtXIFramesOnly> {
        self.i_frames_only_tag
    }

    /// Returns the `EXT-X-INDEPENDENT-SEGMENTS` tag contained in the playlist.
    pub const fn independent_segments_tag(&self) -> Option<ExtXIndependentSegments> {
        self.independent_segments_tag
    }

    /// Returns the `EXT-X-START` tag contained in the playlist.
    pub const fn start_tag(&self) -> Option<ExtXStart> {
        self.start_tag
    }

    /// Returns the `EXT-X-ENDLIST` tag contained in the playlist.
    pub const fn end_list_tag(&self) -> Option<ExtXEndList> {
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
            write!(f, "{}", segment)?;
        }
        if let Some(ref t) = self.end_list_tag {
            writeln!(f, "{}", t)?;
        }
        Ok(())
    }
}

fn parse_media_playlist(
    input: &str,
    builder: &mut MediaPlaylistBuilder,
) -> crate::Result<MediaPlaylist> {
    let mut segment = MediaSegmentBuilder::new();
    let mut segments = vec![];

    let mut has_partial_segment = false;
    let mut has_discontinuity_tag = false;

    for (i, line) in input.parse::<Lines>()?.into_iter().enumerate() {
        match line {
            Line::Tag(tag) => {
                if i == 0 {
                    if tag != Tag::ExtM3u(ExtM3u) {
                        return Err(Error::custom("m3u8 doesn't start with #EXTM3U"));
                    }
                    continue;
                }
                match tag {
                    Tag::ExtM3u(_) => return Err(Error::invalid_input()),
                    Tag::ExtXVersion(t) => {
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
                    Tag::ExtXDateRange(t) => {
                        has_partial_segment = true;
                        segment.tag(t);
                    }
                    Tag::ExtXTargetDuration(t) => {
                        builder.target_duration_tag(t);
                    }
                    Tag::ExtXMediaSequence(t) => {
                        builder.media_sequence_tag(t);
                    }
                    Tag::ExtXDiscontinuitySequence(t) => {
                        if segments.is_empty() {
                            return Err(Error::invalid_input());
                        }
                        if has_discontinuity_tag {
                            return Err(Error::invalid_input());
                        }
                        builder.discontinuity_sequence_tag(t);
                    }
                    Tag::ExtXEndList(t) => {
                        builder.end_list_tag(t);
                    }
                    Tag::ExtXPlaylistType(t) => {
                        builder.playlist_type_tag(t);
                    }
                    Tag::ExtXIFramesOnly(t) => {
                        builder.i_frames_only_tag(t);
                    }
                    Tag::ExtXMedia(_)
                    | Tag::ExtXStreamInf(_)
                    | Tag::ExtXIFrameStreamInf(_)
                    | Tag::ExtXSessionData(_)
                    | Tag::ExtXSessionKey(_) => {
                        return Err(Error::custom(tag));
                    }
                    Tag::ExtXIndependentSegments(t) => {
                        builder.independent_segments_tag(t);
                    }
                    Tag::ExtXStart(t) => {
                        builder.start_tag(t);
                    }
                    Tag::Unknown(_) => {
                        // [6.3.1. General Client Responsibilities]
                        // > ignore any unrecognized tags.
                    }
                }
            }
            Line::Uri(uri) => {
                segment.uri(uri);
                segments.push(segment.finish()?);
                segment = MediaSegmentBuilder::new();
                has_partial_segment = false;
            }
        }
    }
    if has_partial_segment {
        return Err(Error::invalid_input());
    }

    builder.segments(segments);
    builder.build().map_err(Error::builder_error)
}

impl FromStr for MediaPlaylist {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_media_playlist(input, &mut Self::builder())
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
        assert!(MediaPlaylist::builder()
            .allowable_excess_duration(Duration::from_secs(1))
            .parse(m3u8)
            .is_err());

        // Ok (allowable segment duration = 10)
        MediaPlaylist::builder()
            .allowable_excess_duration(Duration::from_secs(2))
            .parse(m3u8)
            .unwrap();
    }

    #[test]
    fn test_parser() {
        let m3u8 = "";
        assert!(m3u8.parse::<MediaPlaylist>().is_err());
    }
}
