use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use derive_builder::Builder;
use shorthand::ShortHand;

use crate::line::{Line, Lines, Tag};
use crate::media_segment::MediaSegment;
use crate::tags::{
    ExtM3u, ExtXDiscontinuitySequence, ExtXEndList, ExtXIFramesOnly, ExtXIndependentSegments,
    ExtXMediaSequence, ExtXPlaylistType, ExtXStart, ExtXTargetDuration, ExtXVersion,
};
use crate::types::ProtocolVersion;
use crate::utils::tag;
use crate::{Encrypted, Error, RequiredVersion};

/// Media playlist.
#[derive(ShortHand, Debug, Clone, Builder, PartialEq, PartialOrd)]
#[builder(build_fn(validate = "Self::validate"))]
#[builder(setter(into, strip_option))]
#[shorthand(enable(must_use, collection_magic, get_mut))]
pub struct MediaPlaylist {
    /// The [`ExtXTargetDuration`] tag of the playlist.
    ///
    /// # Note
    ///
    /// This field is required.
    #[shorthand(enable(copy))]
    target_duration_tag: ExtXTargetDuration,
    /// Sets the [`ExtXMediaSequence`] tag.
    ///
    /// # Note
    ///
    /// This field is optional.
    #[builder(default)]
    media_sequence_tag: Option<ExtXMediaSequence>,
    /// Sets the [`ExtXDiscontinuitySequence`] tag.
    ///
    /// # Note
    ///
    /// This field is optional.
    #[builder(default)]
    discontinuity_sequence_tag: Option<ExtXDiscontinuitySequence>,
    /// Sets the [`ExtXPlaylistType`] tag.
    ///
    /// # Note
    ///
    /// This field is optional.
    #[builder(default)]
    playlist_type_tag: Option<ExtXPlaylistType>,
    /// Sets the [`ExtXIFramesOnly`] tag.
    ///
    /// # Note
    ///
    /// This field is optional.
    #[builder(default)]
    i_frames_only_tag: Option<ExtXIFramesOnly>,
    /// Sets the [`ExtXIndependentSegments`] tag.
    ///
    /// # Note
    ///
    /// This field is optional.
    #[builder(default)]
    independent_segments_tag: Option<ExtXIndependentSegments>,
    /// Sets the [`ExtXStart`] tag.
    ///
    /// # Note
    ///
    /// This field is optional.
    #[builder(default)]
    start_tag: Option<ExtXStart>,
    /// Sets the [`ExtXEndList`] tag.
    ///
    /// # Note
    ///
    /// This field is optional.
    #[builder(default)]
    end_list_tag: Option<ExtXEndList>,
    /// A list of all [`MediaSegment`]s.
    ///
    /// # Note
    ///
    /// This field is required.
    segments: Vec<MediaSegment>,
    /// The allowable excess duration of each media segment in the
    /// associated playlist.
    ///
    /// # Error
    ///
    /// If there is a media segment of which duration exceeds
    /// `#EXT-X-TARGETDURATION + allowable_excess_duration`,
    /// the invocation of `MediaPlaylistBuilder::build()` method will fail.
    ///
    ///
    /// # Note
    ///
    /// This field is optional and the default value is
    /// `Duration::from_secs(0)`.
    #[builder(default = "Duration::from_secs(0)")]
    allowable_excess_duration: Duration,
    /// A list of unknown tags.
    ///
    /// # Note
    ///
    /// This field is optional.
    #[builder(default)]
    unknown_tags: Vec<String>,
}

impl MediaPlaylistBuilder {
    fn validate(&self) -> Result<(), String> {
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
                let rounded_segment_duration = {
                    if segment_duration.subsec_nanos() < 500_000_000 {
                        Duration::from_secs(segment_duration.as_secs())
                    } else {
                        Duration::from_secs(segment_duration.as_secs() + 1)
                    }
                };

                let max_segment_duration = {
                    if let Some(value) = &self.allowable_excess_duration {
                        target_duration + *value
                    } else {
                        target_duration
                    }
                };

                if rounded_segment_duration > max_segment_duration {
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
                        let last_uri = last_range_uri.ok_or_else(Error::invalid_input)?;
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

    /// Adds a media segment to the resulting playlist.
    pub fn push_segment<VALUE: Into<MediaSegment>>(&mut self, value: VALUE) -> &mut Self {
        if let Some(segments) = &mut self.segments {
            segments.push(value.into());
        } else {
            self.segments = Some(vec![value.into()]);
        }
        self
    }

    /// Parse the rest of the [`MediaPlaylist`] from an m3u8 file.
    pub fn parse(&mut self, input: &str) -> crate::Result<MediaPlaylist> {
        parse_media_playlist(input, self)
    }
}

impl RequiredVersion for MediaPlaylistBuilder {
    fn required_version(&self) -> ProtocolVersion {
        required_version![
            self.target_duration_tag,
            self.media_sequence_tag,
            self.discontinuity_sequence_tag,
            self.playlist_type_tag,
            self.i_frames_only_tag,
            self.independent_segments_tag,
            self.start_tag,
            self.end_list_tag,
            self.segments
        ]
    }
}

impl MediaPlaylist {
    /// Returns a builder for [`MediaPlaylist`].
    pub fn builder() -> MediaPlaylistBuilder { MediaPlaylistBuilder::default() }
}

impl RequiredVersion for MediaPlaylist {
    fn required_version(&self) -> ProtocolVersion {
        required_version![
            self.target_duration_tag,
            self.media_sequence_tag,
            self.discontinuity_sequence_tag,
            self.playlist_type_tag,
            self.i_frames_only_tag,
            self.independent_segments_tag,
            self.start_tag,
            self.end_list_tag,
            self.segments
        ]
    }
}

impl fmt::Display for MediaPlaylist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", ExtM3u)?;

        if self.required_version() != ProtocolVersion::V1 {
            writeln!(f, "{}", ExtXVersion::new(self.required_version()))?;
        }

        writeln!(f, "{}", self.target_duration_tag)?;

        if let Some(value) = &self.media_sequence_tag {
            writeln!(f, "{}", value)?;
        }

        if let Some(value) = &self.discontinuity_sequence_tag {
            writeln!(f, "{}", value)?;
        }

        if let Some(value) = &self.playlist_type_tag {
            writeln!(f, "{}", value)?;
        }

        if let Some(value) = &self.i_frames_only_tag {
            writeln!(f, "{}", value)?;
        }

        if let Some(value) = &self.independent_segments_tag {
            writeln!(f, "{}", value)?;
        }

        if let Some(value) = &self.start_tag {
            writeln!(f, "{}", value)?;
        }

        for segment in &self.segments {
            write!(f, "{}", segment)?;
        }

        if let Some(value) = &self.end_list_tag {
            writeln!(f, "{}", value)?;
        }

        for value in &self.unknown_tags {
            writeln!(f, "{}", value)?;
        }

        Ok(())
    }
}

fn parse_media_playlist(
    input: &str,
    builder: &mut MediaPlaylistBuilder,
) -> crate::Result<MediaPlaylist> {
    let input = tag(input, "#EXTM3U")?;

    let mut segment = MediaSegment::builder();
    let mut segments = vec![];

    let mut has_partial_segment = false;
    let mut has_discontinuity_tag = false;
    let mut unknown_tags = vec![];

    let mut available_key_tags: Vec<crate::tags::ExtXKey> = vec![];

    for line in Lines::from(input) {
        match line? {
            Line::Tag(tag) => {
                match tag {
                    Tag::ExtInf(t) => {
                        has_partial_segment = true;
                        segment.inf_tag(t);
                    }
                    Tag::ExtXByteRange(t) => {
                        has_partial_segment = true;
                        segment.byte_range_tag(t);
                    }
                    Tag::ExtXDiscontinuity(t) => {
                        has_discontinuity_tag = true;
                        has_partial_segment = true;
                        segment.discontinuity_tag(t);
                    }
                    Tag::ExtXKey(t) => {
                        has_partial_segment = true;
                        if available_key_tags.is_empty() {
                            // An ExtXKey applies to every MediaSegment and to every Media
                            // Initialization Section declared by an EXT-X-MAP tag, that appears
                            // between it and the next EXT-X-KEY tag in the Playlist file with the
                            // same KEYFORMAT attribute (or the end of the Playlist file).
                            available_key_tags = available_key_tags
                                .into_iter()
                                .map(|k| {
                                    if t.key_format() == k.key_format() {
                                        t.clone()
                                    } else {
                                        k
                                    }
                                })
                                .collect();
                        } else {
                            available_key_tags.push(t);
                        }
                    }
                    Tag::ExtXMap(mut t) => {
                        has_partial_segment = true;

                        t.set_keys(available_key_tags.clone());
                        segment.map_tag(t);
                    }
                    Tag::ExtXProgramDateTime(t) => {
                        has_partial_segment = true;
                        segment.program_date_time_tag(t);
                    }
                    Tag::ExtXDateRange(t) => {
                        has_partial_segment = true;
                        segment.date_range_tag(t);
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
                        return Err(Error::unexpected_tag(tag));
                    }
                    Tag::ExtXIndependentSegments(t) => {
                        builder.independent_segments_tag(t);
                    }
                    Tag::ExtXStart(t) => {
                        builder.start_tag(t);
                    }
                    Tag::ExtXVersion(_) => {}
                    Tag::Unknown(_) => {
                        // [6.3.1. General Client Responsibilities]
                        // > ignore any unrecognized tags.
                        unknown_tags.push(tag.to_string());
                    }
                }
            }
            Line::Uri(uri) => {
                segment.uri(uri);
                segment.keys(available_key_tags.clone());
                segments.push(segment.build().map_err(Error::builder)?);
                segment = MediaSegment::builder();
                has_partial_segment = false;
            }
        }
    }

    if has_partial_segment {
        return Err(Error::invalid_input());
    }

    builder.unknown_tags(unknown_tags);
    builder.segments(segments);
    builder.build().map_err(Error::builder)
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
    use pretty_assertions::assert_eq;

    #[test]
    fn too_large_segment_duration_test() {
        let playlist = concat!(
            "#EXTM3U\n",
            "#EXT-X-TARGETDURATION:8\n",
            "#EXT-X-VERSION:3\n",
            "#EXTINF:9.009,\n",
            "http://media.example.com/first.ts\n",
            "#EXTINF:9.509,\n",
            "http://media.example.com/second.ts\n",
            "#EXTINF:3.003,\n",
            "http://media.example.com/third.ts\n",
            "#EXT-X-ENDLIST\n"
        );

        // Error (allowable segment duration = target duration = 8)
        assert!(playlist.parse::<MediaPlaylist>().is_err());

        // Error (allowable segment duration = 9)
        assert!(MediaPlaylist::builder()
            .allowable_excess_duration(Duration::from_secs(1))
            .parse(playlist)
            .is_err());

        // Ok (allowable segment duration = 10)
        MediaPlaylist::builder()
            .allowable_excess_duration(Duration::from_secs(2))
            .parse(playlist)
            .unwrap();
    }

    #[test]
    fn test_empty_playlist() {
        let playlist = "";
        assert!(playlist.parse::<MediaPlaylist>().is_err());
    }
}
