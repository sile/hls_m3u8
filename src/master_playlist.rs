use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use derive_builder::Builder;
use shorthand::ShortHand;

use crate::line::{Line, Lines, Tag};
use crate::tags::{
    ExtM3u, ExtXIFrameStreamInf, ExtXIndependentSegments, ExtXMedia, ExtXSessionData,
    ExtXSessionKey, ExtXStart, ExtXStreamInf, ExtXVersion,
};
use crate::types::{ClosedCaptions, MediaType, ProtocolVersion};
use crate::{Error, RequiredVersion};

/// Master playlist.
#[derive(ShortHand, Debug, Clone, Builder, PartialEq)]
#[builder(build_fn(validate = "Self::validate"))]
#[builder(setter(into, strip_option))]
#[shorthand(enable(must_use, get_mut, collection_magic))]
pub struct MasterPlaylist {
    /// The [`ExtXIndependentSegments`] tag of the playlist.
    ///
    /// # Note
    ///
    /// This tag is optional.
    #[builder(default)]
    independent_segments_tag: Option<ExtXIndependentSegments>,
    /// The [`ExtXStart`] tag of the playlist.
    ///
    /// # Note
    ///
    /// This tag is optional.
    #[builder(default)]
    start_tag: Option<ExtXStart>,
    /// The [`ExtXMedia`] tags of the playlist.
    ///
    /// # Note
    ///
    /// This tag is optional.
    #[builder(default)]
    media_tags: Vec<ExtXMedia>,
    /// The [`ExtXStreamInf`] tags of the playlist.
    ///
    /// # Note
    ///
    /// This tag is optional.
    #[builder(default)]
    stream_inf_tags: Vec<ExtXStreamInf>,
    /// The [`ExtXIFrameStreamInf`] tags of the playlist.
    ///
    /// # Note
    ///
    /// This tag is optional.
    #[builder(default)]
    i_frame_stream_inf_tags: Vec<ExtXIFrameStreamInf>,
    /// The [`ExtXSessionData`] tags of the playlist.
    ///
    /// # Note
    ///
    /// This tag is optional.
    #[builder(default)]
    session_data_tags: Vec<ExtXSessionData>,
    /// The [`ExtXSessionKey`] tags of the playlist.
    ///
    /// # Note
    ///
    /// This tag is optional.
    #[builder(default)]
    session_key_tags: Vec<ExtXSessionKey>,
    /// A list of tags that are unknown.
    ///
    /// # Note
    ///
    /// This tag is optional.
    #[builder(default)]
    unknown_tags: Vec<String>,
}

impl MasterPlaylist {
    /// Returns a builder for a [`MasterPlaylist`].
    ///
    /// # Example
    ///
    /// ```
    /// use hls_m3u8::tags::ExtXStart;
    /// use hls_m3u8::MasterPlaylist;
    ///
    /// MasterPlaylist::builder()
    ///     .start_tag(ExtXStart::new(20.123456))
    ///     .build()?;
    /// # Ok::<(), Box<dyn ::std::error::Error>>(())
    /// ```
    pub fn builder() -> MasterPlaylistBuilder { MasterPlaylistBuilder::default() }
}

impl RequiredVersion for MasterPlaylist {
    fn required_version(&self) -> ProtocolVersion {
        required_version![
            self.independent_segments_tag,
            self.start_tag,
            self.media_tags,
            self.stream_inf_tags,
            self.i_frame_stream_inf_tags,
            self.session_data_tags,
            self.session_key_tags
        ]
    }
}

impl MasterPlaylistBuilder {
    fn validate(&self) -> Result<(), String> {
        self.validate_stream_inf_tags().map_err(|e| e.to_string())?;
        self.validate_i_frame_stream_inf_tags()
            .map_err(|e| e.to_string())?;
        self.validate_session_data_tags()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn validate_stream_inf_tags(&self) -> crate::Result<()> {
        if let Some(value) = &self.stream_inf_tags {
            let mut has_none_closed_captions = false;

            for t in value {
                if let Some(group_id) = t.audio() {
                    if !self.check_media_group(MediaType::Audio, group_id) {
                        return Err(Error::unmatched_group(group_id));
                    }
                }
                if let Some(group_id) = t.video() {
                    if !self.check_media_group(MediaType::Video, group_id) {
                        return Err(Error::unmatched_group(group_id));
                    }
                }
                if let Some(group_id) = t.subtitles() {
                    if !self.check_media_group(MediaType::Subtitles, group_id) {
                        return Err(Error::unmatched_group(group_id));
                    }
                }
                match &t.closed_captions() {
                    Some(ClosedCaptions::GroupId(group_id)) => {
                        if !self.check_media_group(MediaType::ClosedCaptions, group_id) {
                            return Err(Error::unmatched_group(group_id));
                        }
                    }
                    Some(ClosedCaptions::None) => {
                        has_none_closed_captions = true;
                    }
                    _ => {}
                }
            }
            if has_none_closed_captions
                && !value
                    .iter()
                    .all(|t| t.closed_captions() == Some(&ClosedCaptions::None))
            {
                return Err(Error::invalid_input());
            }
        }
        Ok(())
    }

    fn validate_i_frame_stream_inf_tags(&self) -> crate::Result<()> {
        if let Some(value) = &self.i_frame_stream_inf_tags {
            for t in value {
                if let Some(group_id) = t.video() {
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
        if let Some(value) = &self.session_data_tags {
            for t in value {
                if !set.insert((t.data_id(), t.language())) {
                    return Err(Error::custom(format!("Conflict: {}", t)));
                }
            }
        }
        Ok(())
    }

    fn check_media_group<T: ToString>(&self, media_type: MediaType, group_id: T) -> bool {
        if let Some(value) = &self.media_tags {
            value
                .iter()
                .any(|t| t.media_type() == media_type && t.group_id() == &group_id.to_string())
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
            self.independent_segments_tag.flatten(),
            self.start_tag.flatten(),
            self.media_tags,
            self.stream_inf_tags,
            self.i_frame_stream_inf_tags,
            self.session_data_tags,
            self.session_key_tags
        ]
    }
}

impl fmt::Display for MasterPlaylist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", ExtM3u)?;
        if self.required_version() != ProtocolVersion::V1 {
            writeln!(f, "{}", ExtXVersion::new(self.required_version()))?;
        }

        for t in &self.media_tags {
            writeln!(f, "{}", t)?;
        }

        for t in &self.stream_inf_tags {
            writeln!(f, "{}", t)?;
        }

        for t in &self.i_frame_stream_inf_tags {
            writeln!(f, "{}", t)?;
        }

        for t in &self.session_data_tags {
            writeln!(f, "{}", t)?;
        }

        for t in &self.session_key_tags {
            writeln!(f, "{}", t)?;
        }

        if let Some(value) = &self.independent_segments_tag {
            writeln!(f, "{}", value)?;
        }

        if let Some(value) = &self.start_tag {
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
        let mut builder = Self::builder();

        let mut media_tags = vec![];
        let mut stream_inf_tags = vec![];
        let mut i_frame_stream_inf_tags = vec![];
        let mut session_data_tags = vec![];
        let mut session_key_tags = vec![];
        let mut unknown_tags = vec![];

        for (i, line) in input.parse::<Lines>()?.into_iter().enumerate() {
            match line {
                Line::Tag(tag) => {
                    if i == 0 {
                        if tag != Tag::ExtM3u(ExtM3u) {
                            return Err(Error::invalid_input());
                        }
                        continue;
                    }
                    match tag {
                        Tag::ExtM3u(_) => {
                            return Err(Error::invalid_input());
                        }
                        Tag::ExtXVersion(_) => {
                            // This tag can be ignored, because the
                            // MasterPlaylist will automatically set the
                            // ExtXVersion tag to correct version!
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
                            media_tags.push(t);
                        }
                        Tag::ExtXStreamInf(t) => {
                            stream_inf_tags.push(t);
                        }
                        Tag::ExtXIFrameStreamInf(t) => {
                            i_frame_stream_inf_tags.push(t);
                        }
                        Tag::ExtXSessionData(t) => {
                            session_data_tags.push(t);
                        }
                        Tag::ExtXSessionKey(t) => {
                            session_key_tags.push(t);
                        }
                        Tag::ExtXIndependentSegments(t) => {
                            builder.independent_segments_tag(t);
                        }
                        Tag::ExtXStart(t) => {
                            builder.start_tag(t);
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
            }
        }

        builder.media_tags(media_tags);
        builder.stream_inf_tags(stream_inf_tags);
        builder.i_frame_stream_inf_tags(i_frame_stream_inf_tags);
        builder.session_data_tags(session_data_tags);
        builder.session_key_tags(session_key_tags);
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
