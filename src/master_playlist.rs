use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use derive_builder::Builder;

use crate::line::{Line, Lines, Tag};
use crate::tags::{
    ExtM3u, ExtXIFrameStreamInf, ExtXIndependentSegments, ExtXMedia, ExtXSessionData,
    ExtXSessionKey, ExtXStart, ExtXStreamInf, ExtXVersion,
};
use crate::types::{ClosedCaptions, MediaType, ProtocolVersion};
use crate::{Error, RequiredVersion};

#[derive(Debug, Clone, Builder, PartialEq)]
#[builder(build_fn(validate = "Self::validate"))]
#[builder(setter(into, strip_option))]
/// Master playlist.
pub struct MasterPlaylist {
    //#[builder(default, setter(name = "version"))]
    #[builder(default, setter(skip))]
    version_tag: ExtXVersion,
    #[builder(default)]
    /// Sets the [`ExtXIndependentSegments`] tag.
    independent_segments_tag: Option<ExtXIndependentSegments>,
    #[builder(default)]
    /// Sets the [`ExtXStart`] tag.
    start_tag: Option<ExtXStart>,
    #[builder(default)]
    /// Sets the [`ExtXMedia`] tag.
    media_tags: Vec<ExtXMedia>,
    #[builder(default)]
    /// Sets all [`ExtXStreamInf`] tags.
    stream_inf_tags: Vec<ExtXStreamInf>,
    #[builder(default)]
    /// Sets all [`ExtXIFrameStreamInf`] tags.
    i_frame_stream_inf_tags: Vec<ExtXIFrameStreamInf>,
    #[builder(default)]
    /// Sets all [`ExtXSessionData`] tags.
    session_data_tags: Vec<ExtXSessionData>,
    #[builder(default)]
    /// Sets all [`ExtXSessionKey`] tags.
    session_key_tags: Vec<ExtXSessionKey>,
}

impl MasterPlaylist {
    /// Returns a Builder for a [`MasterPlaylist`].
    pub fn builder() -> MasterPlaylistBuilder { MasterPlaylistBuilder::default() }

    /// Returns the [`ExtXIndependentSegments`] tag contained in the playlist.
    pub const fn independent_segments(&self) -> Option<ExtXIndependentSegments> {
        self.independent_segments_tag
    }

    /// Sets the [`ExtXIndependentSegments`] tag contained in the playlist.
    pub fn set_independent_segments<T>(&mut self, value: Option<T>) -> &mut Self
    where
        T: Into<ExtXIndependentSegments>,
    {
        self.independent_segments_tag = value.map(|v| v.into());
        self
    }

    /// Returns the [`ExtXStart`] tag contained in the playlist.
    pub const fn start(&self) -> Option<ExtXStart> { self.start_tag }

    /// Sets the [`ExtXStart`] tag contained in the playlist.
    pub fn set_start<T>(&mut self, value: Option<T>) -> &mut Self
    where
        T: Into<ExtXStart>,
    {
        self.start_tag = value.map(|v| v.into());
        self
    }

    /// Returns the [`ExtXMedia`] tags contained in the playlist.
    pub const fn media_tags(&self) -> &Vec<ExtXMedia> { &self.media_tags }

    /// Appends an [`ExtXMedia`].
    pub fn push_media_tag(&mut self, value: ExtXMedia) -> &mut Self {
        self.media_tags.push(value);
        self
    }

    /// Sets the [`ExtXMedia`] tags contained in the playlist.
    pub fn set_media_tags<T>(&mut self, value: Vec<T>) -> &mut Self
    where
        T: Into<ExtXMedia>,
    {
        self.media_tags = value.into_iter().map(|v| v.into()).collect();
        self
    }

    /// Returns the [`ExtXStreamInf`] tags contained in the playlist.
    pub const fn stream_inf_tags(&self) -> &Vec<ExtXStreamInf> { &self.stream_inf_tags }

    /// Appends an [`ExtXStreamInf`].
    pub fn push_stream_inf(&mut self, value: ExtXStreamInf) -> &mut Self {
        self.stream_inf_tags.push(value);
        self
    }

    /// Sets the [`ExtXStreamInf`] tags contained in the playlist.
    pub fn set_stream_inf_tags<T>(&mut self, value: Vec<T>) -> &mut Self
    where
        T: Into<ExtXStreamInf>,
    {
        self.stream_inf_tags = value.into_iter().map(|v| v.into()).collect();
        self
    }

    /// Returns the [`ExtXIFrameStreamInf`] tags contained in the playlist.
    pub const fn i_frame_stream_inf_tags(&self) -> &Vec<ExtXIFrameStreamInf> {
        &self.i_frame_stream_inf_tags
    }

    /// Appends an [`ExtXIFrameStreamInf`].
    pub fn push_i_frame_stream_inf(&mut self, value: ExtXIFrameStreamInf) -> &mut Self {
        self.i_frame_stream_inf_tags.push(value);
        self
    }

    /// Sets the [`ExtXIFrameStreamInf`] tags contained in the playlist.
    pub fn set_i_frame_stream_inf_tags<T>(&mut self, value: Vec<T>) -> &mut Self
    where
        T: Into<ExtXIFrameStreamInf>,
    {
        self.i_frame_stream_inf_tags = value.into_iter().map(|v| v.into()).collect();
        self
    }

    /// Returns the [`ExtXSessionData`] tags contained in the playlist.
    pub const fn session_data_tags(&self) -> &Vec<ExtXSessionData> { &self.session_data_tags }

    /// Appends an [`ExtXSessionData`].
    pub fn push_session_data(&mut self, value: ExtXSessionData) -> &mut Self {
        self.session_data_tags.push(value);
        self
    }

    /// Sets the [`ExtXSessionData`] tags contained in the playlist.
    pub fn set_session_data_tags<T>(&mut self, value: Vec<T>) -> &mut Self
    where
        T: Into<ExtXSessionData>,
    {
        self.session_data_tags = value.into_iter().map(|v| v.into()).collect();
        self
    }

    /// Returns the [`ExtXSessionKey`] tags contained in the playlist.
    pub const fn session_key_tags(&self) -> &Vec<ExtXSessionKey> { &self.session_key_tags }

    /// Appends an [`ExtXSessionKey`].
    pub fn push_session_key(&mut self, value: ExtXSessionKey) -> &mut Self {
        self.session_key_tags.push(value);
        self
    }

    /// Sets the [`ExtXSessionKey`] tags contained in the playlist.
    pub fn set_session_key_tags<T>(&mut self, value: Vec<T>) -> &mut Self
    where
        T: Into<ExtXSessionKey>,
    {
        self.session_key_tags = value.into_iter().map(|v| v.into()).collect();
        self
    }
}

macro_rules! required_version {
    ( $( $tag:expr ),* ) => {
        ::core::iter::empty()
            $(
                .chain(::core::iter::once($tag.required_version()))
            )*
            .max()
            .unwrap_or_default()
    }
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
                match t.closed_captions() {
                    &Some(ClosedCaptions::GroupId(ref group_id)) => {
                        if !self.check_media_group(MediaType::ClosedCaptions, group_id) {
                            return Err(Error::unmatched_group(group_id));
                        }
                    }
                    &Some(ClosedCaptions::None) => {
                        has_none_closed_captions = true;
                    }
                    None => {}
                }
            }
            if has_none_closed_captions
                && !value
                    .iter()
                    .all(|t| t.closed_captions() == &Some(ClosedCaptions::None))
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
        Ok(())
    }
}

impl FromStr for MasterPlaylist {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut builder = MasterPlaylist::builder();

        let mut media_tags = vec![];
        let mut stream_inf_tags = vec![];
        let mut i_frame_stream_inf_tags = vec![];
        let mut session_data_tags = vec![];
        let mut session_key_tags = vec![];

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

                            // builder.version(t.version());
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
                            // TODO: collect custom tags
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

        builder.build().map_err(Error::builder_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
