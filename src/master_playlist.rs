use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use derive_builder::Builder;
use shorthand::ShortHand;

use crate::line::{Line, Lines, Tag};
use crate::tags::{
    ExtM3u, ExtXIndependentSegments, ExtXMedia, ExtXSessionData, ExtXSessionKey, ExtXStart,
    ExtXVersion, VariantStream,
};
use crate::types::{ClosedCaptions, MediaType, ProtocolVersion};
use crate::utils::tag;
use crate::{Error, RequiredVersion};

/// The master playlist describes all of the available variants for your
/// content. Each variant is a version of the stream at a particular bitrate
/// and is contained in a separate playlist.
#[derive(ShortHand, Debug, Clone, Builder, PartialEq)]
#[builder(build_fn(validate = "Self::validate"))]
#[builder(setter(into, strip_option))]
#[shorthand(enable(must_use, get_mut, collection_magic))]
pub struct MasterPlaylist {
    /// The [`ExtXIndependentSegments`] tag signals that all media samples in a
    /// [`MediaSegment`] can be decoded without information from other segments.
    ///
    /// # Note
    ///
    /// This tag is optional.
    ///
    /// If this tag is specified it will apply to every [`MediaSegment`] in
    /// every [`MediaPlaylist`] in the [`MasterPlaylist`].
    ///
    /// [`MediaSegment`]: crate::MediaSegment
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    #[builder(default)]
    independent_segments: Option<ExtXIndependentSegments>,
    /// The [`ExtXStart`] tag indicates a preferred point at which to start
    /// playing a Playlist.
    ///
    /// # Note
    ///
    /// This tag is optional.
    #[builder(default)]
    start: Option<ExtXStart>,
    /// The [`ExtXMedia`] tag is used to relate [`MediaPlaylist`]s,
    /// that contain alternative renditions of the same content.
    ///
    /// For example, three [`ExtXMedia`] tags can be used to identify audio-only
    /// [`MediaPlaylist`]s, that contain English, French, and Spanish
    /// renditions of the same presentation. Or, two [`ExtXMedia`] tags can
    /// be used to identify video-only [`MediaPlaylist`]s that show two
    /// different camera angles.
    ///
    /// # Note
    ///
    /// This tag is optional.
    ///
    /// [`MediaPlaylist`]: crate::MediaPlaylist
    #[builder(default)]
    media: Vec<ExtXMedia>,
    /// A list of all streams of this [`MasterPlaylist`].
    ///
    /// # Note
    ///
    /// This tag is optional.
    #[builder(default)]
    variants: Vec<VariantStream>,
    /// The [`ExtXSessionData`] tags of the playlist.
    ///
    /// # Note
    ///
    /// This tag is optional.
    #[builder(default)]
    session_data: Vec<ExtXSessionData>,
    /// The [`ExtXSessionKey`] tags of the playlist.
    ///
    /// # Note
    ///
    /// This tag is optional.
    #[builder(default)]
    session_keys: Vec<ExtXSessionKey>,
    /// A list of tags that are unknown.
    ///
    /// # Note
    ///
    /// This tag is optional.
    #[builder(default)]
    unknown_tags: Vec<String>,
}

impl MasterPlaylist {
    // TODO: finish builder example!
    /// Returns a builder for a [`MasterPlaylist`].
    ///
    /// # Example
    ///
    /// ```
    /// use hls_m3u8::tags::ExtXStart;
    /// use hls_m3u8::MasterPlaylist;
    ///
    /// MasterPlaylist::builder()
    ///     .start(ExtXStart::new(20.123456))
    ///     .build()?;
    /// # Ok::<(), Box<dyn ::std::error::Error>>(())
    /// ```
    pub fn builder() -> MasterPlaylistBuilder { MasterPlaylistBuilder::default() }
}

impl RequiredVersion for MasterPlaylist {
    fn required_version(&self) -> ProtocolVersion {
        required_version![
            self.independent_segments,
            self.start,
            self.media,
            self.variants,
            self.session_data,
            self.session_keys
        ]
    }
}

impl MasterPlaylistBuilder {
    fn validate(&self) -> Result<(), String> {
        self.validate_variants().map_err(|e| e.to_string())?;
        self.validate_session_data_tags()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn validate_variants(&self) -> crate::Result<()> {
        if let Some(variants) = &self.variants {
            self.validate_stream_inf(variants)?;
            self.validate_i_frame_stream_inf(variants)?;
        }

        Ok(())
    }

    fn validate_stream_inf(&self, value: &[VariantStream]) -> crate::Result<()> {
        let mut has_none_closed_captions = false;

        for t in value {
            if let VariantStream::ExtXStreamInf {
                audio,
                subtitles,
                closed_captions,
                stream_data,
                ..
            } = &t
            {
                if let Some(group_id) = &audio {
                    if !self.check_media_group(MediaType::Audio, group_id) {
                        return Err(Error::unmatched_group(group_id));
                    }
                }
                if let Some(group_id) = &stream_data.video() {
                    if !self.check_media_group(MediaType::Video, group_id) {
                        return Err(Error::unmatched_group(group_id));
                    }
                }
                if let Some(group_id) = &subtitles {
                    if !self.check_media_group(MediaType::Subtitles, group_id) {
                        return Err(Error::unmatched_group(group_id));
                    }
                }

                if let Some(closed_captions) = &closed_captions {
                    match &closed_captions {
                        ClosedCaptions::GroupId(group_id) => {
                            if !self.check_media_group(MediaType::ClosedCaptions, group_id) {
                                return Err(Error::unmatched_group(group_id));
                            }
                        }
                        ClosedCaptions::None => {
                            has_none_closed_captions = true;
                        }
                    }
                }
            }
        }

        if has_none_closed_captions
            && !value.iter().all(|t| {
                if let VariantStream::ExtXStreamInf {
                    closed_captions, ..
                } = &t
                {
                    closed_captions == &Some(ClosedCaptions::None)
                } else {
                    false
                }
            })
        {
            return Err(Error::invalid_input());
        }

        Ok(())
    }

    fn validate_i_frame_stream_inf(&self, value: &[VariantStream]) -> crate::Result<()> {
        for t in value {
            if let VariantStream::ExtXIFrame { stream_data, .. } = &t {
                if let Some(group_id) = stream_data.video() {
                    if !self.check_media_group(MediaType::Video, group_id) {
                        return Err(Error::unmatched_group(group_id));
                    }
                }
            }
        }

        Ok(())
    }

    fn validate_session_data_tags(&self) -> crate::Result<()> {
        let mut set = HashSet::new();

        if let Some(value) = &self.session_data {
            for t in value {
                if !set.insert((t.data_id(), t.language())) {
                    return Err(Error::custom(format!("Conflict: {}", t)));
                }
            }
        }

        Ok(())
    }

    fn check_media_group<T: AsRef<str>>(&self, media_type: MediaType, group_id: T) -> bool {
        if let Some(value) = &self.media {
            value
                .iter()
                .any(|t| t.media_type() == media_type && t.group_id().as_str() == group_id.as_ref())
        } else {
            false
        }
    }
}

impl RequiredVersion for MasterPlaylistBuilder {
    fn required_version(&self) -> ProtocolVersion {
        // TODO: the .flatten() can be removed as soon as `recursive traits` are
        //       supported. (RequiredVersion is implemented for Option<T>, but
        //       not for Option<Option<T>>)
        // https://github.com/rust-lang/chalk/issues/12
        required_version![
            self.independent_segments.flatten(),
            self.start.flatten(),
            self.media,
            self.variants,
            self.session_data,
            self.session_keys
        ]
    }
}

impl fmt::Display for MasterPlaylist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", ExtM3u)?;

        if self.required_version() != ProtocolVersion::V1 {
            writeln!(f, "{}", ExtXVersion::new(self.required_version()))?;
        }

        for t in &self.media {
            writeln!(f, "{}", t)?;
        }

        for t in &self.variants {
            writeln!(f, "{}", t)?;
        }

        for t in &self.session_data {
            writeln!(f, "{}", t)?;
        }

        for t in &self.session_keys {
            writeln!(f, "{}", t)?;
        }

        if let Some(value) = &self.independent_segments {
            writeln!(f, "{}", value)?;
        }

        if let Some(value) = &self.start {
            writeln!(f, "{}", value)?;
        }

        for t in &self.unknown_tags {
            writeln!(f, "{}", t)?;
        }

        Ok(())
    }
}

impl FromStr for MasterPlaylist {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, ExtM3u::PREFIX)?;
        let mut builder = Self::builder();

        let mut media = vec![];
        let mut variants = vec![];
        let mut session_data = vec![];
        let mut session_keys = vec![];
        let mut unknown_tags = vec![];

        for line in Lines::from(input) {
            match line? {
                Line::Tag(tag) => {
                    match tag {
                        Tag::ExtXVersion(_) => {
                            // This tag can be ignored, because the
                            // MasterPlaylist will automatically set the
                            // ExtXVersion tag to the minimum required version
                            // TODO: this might be verified?
                        }
                        Tag::ExtInf(_)
                        | Tag::ExtXByteRange(_)
                        | Tag::ExtXDiscontinuity(_)
                        | Tag::ExtXKey(_)
                        | Tag::ExtXMap(_)
                        | Tag::ExtXProgramDateTime(_)
                        | Tag::ExtXDateRange(_)
                        | Tag::ExtXTargetDuration(_)
                        | Tag::ExtXMediaSequence(_)
                        | Tag::ExtXDiscontinuitySequence(_)
                        | Tag::ExtXEndList(_)
                        | Tag::ExtXPlaylistType(_)
                        | Tag::ExtXIFramesOnly(_) => {
                            return Err(Error::custom(format!(
                                "This tag isn't allowed in a master playlist: {}",
                                tag
                            )));
                        }
                        Tag::ExtXMedia(t) => {
                            media.push(t);
                        }
                        Tag::VariantStream(t) => {
                            variants.push(t);
                        }
                        Tag::ExtXSessionData(t) => {
                            session_data.push(t);
                        }
                        Tag::ExtXSessionKey(t) => {
                            session_keys.push(t);
                        }
                        Tag::ExtXIndependentSegments(t) => {
                            builder.independent_segments(t);
                        }
                        Tag::ExtXStart(t) => {
                            builder.start(t);
                        }
                        _ => {
                            // [6.3.1. General Client Responsibilities]
                            // > ignore any unrecognized tags.
                            unknown_tags.push(tag.to_string());
                        }
                    }
                }
                Line::Uri(uri) => {
                    return Err(Error::custom(format!("Unexpected URI: {:?}", uri)));
                }
                _ => {}
            }
        }

        builder.media(media);
        builder.variants(variants);
        builder.session_data(session_data);
        builder.session_keys(session_keys);
        builder.unknown_tags(unknown_tags);

        builder.build().map_err(Error::builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parser() {
        "#EXTM3U\n\
         #EXT-X-STREAM-INF:BANDWIDTH=150000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n\
         http://example.com/low/index.m3u8\n\
         #EXT-X-STREAM-INF:BANDWIDTH=240000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n\
         http://example.com/lo_mid/index.m3u8\n\
         #EXT-X-STREAM-INF:BANDWIDTH=440000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n\
         http://example.com/hi_mid/index.m3u8\n\
         #EXT-X-STREAM-INF:BANDWIDTH=640000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=640x360\n\
         http://example.com/high/index.m3u8\n\
         #EXT-X-STREAM-INF:BANDWIDTH=64000,CODECS=\"mp4a.40.5\"\n\
         http://example.com/audio/index.m3u8\n"
            .parse::<MasterPlaylist>()
            .unwrap();
    }

    #[test]
    fn test_display() {
        let input = "#EXTM3U\n\
        #EXT-X-STREAM-INF:BANDWIDTH=150000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n\
        http://example.com/low/index.m3u8\n\
        #EXT-X-STREAM-INF:BANDWIDTH=240000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n\
        http://example.com/lo_mid/index.m3u8\n\
        #EXT-X-STREAM-INF:BANDWIDTH=440000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=416x234\n\
        http://example.com/hi_mid/index.m3u8\n\
        #EXT-X-STREAM-INF:BANDWIDTH=640000,CODECS=\"avc1.42e00a,mp4a.40.2\",RESOLUTION=640x360\n\
        http://example.com/high/index.m3u8\n\
        #EXT-X-STREAM-INF:BANDWIDTH=64000,CODECS=\"mp4a.40.5\"\n\
        http://example.com/audio/index.m3u8\n";

        let playlist = input.parse::<MasterPlaylist>().unwrap();
        assert_eq!(playlist.to_string(), input);
    }
}
