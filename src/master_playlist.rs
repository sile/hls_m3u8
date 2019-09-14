use std::collections::HashSet;
use std::fmt;
use std::iter;
use std::str::FromStr;

use derive_builder::Builder;

use crate::line::{Line, Lines, Tag};
use crate::tags::{
    ExtM3u, ExtXIFrameStreamInf, ExtXIndependentSegments, ExtXMedia, ExtXSessionData,
    ExtXSessionKey, ExtXStart, ExtXStreamInf, ExtXVersion,
};
use crate::types::{ClosedCaptions, MediaType, ProtocolVersion};
use crate::Error;

/// Master playlist.
#[derive(Debug, Clone, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
#[builder(setter(into, strip_option))]
pub struct MasterPlaylist {
    #[builder(default, setter(name = "version"))]
    /// Sets the protocol compatibility version of the resulting playlist.
    ///
    /// If the resulting playlist has tags which requires a compatibility version greater than
    /// `version`,
    /// `build()` method will fail with an `ErrorKind::InvalidInput` error.
    ///
    /// The default is the maximum version among the tags in the playlist.
    version_tag: ExtXVersion,
    #[builder(default)]
    /// Sets the [ExtXIndependentSegments] tag.
    independent_segments_tag: Option<ExtXIndependentSegments>,
    #[builder(default)]
    /// Sets the [ExtXStart] tag.
    start_tag: Option<ExtXStart>,
    /// Sets the [ExtXMedia] tag.
    media_tags: Vec<ExtXMedia>,
    /// Sets all [ExtXStreamInf]s.
    stream_inf_tags: Vec<ExtXStreamInf>,
    /// Sets all [ExtXIFrameStreamInf]s.
    i_frame_stream_inf_tags: Vec<ExtXIFrameStreamInf>,
    /// Sets all [ExtXSessionData]s.
    session_data_tags: Vec<ExtXSessionData>,
    /// Sets all [ExtXSessionKey]s.
    session_key_tags: Vec<ExtXSessionKey>,
}

impl MasterPlaylist {
    /// Returns a Builder for a MasterPlaylist.
    pub fn builder() -> MasterPlaylistBuilder {
        MasterPlaylistBuilder::default()
    }

    /// Returns the `EXT-X-VERSION` tag contained in the playlist.
    pub const fn version_tag(&self) -> ExtXVersion {
        self.version_tag
    }

    /// Returns the `EXT-X-INDEPENDENT-SEGMENTS` tag contained in the playlist.
    pub const fn independent_segments_tag(&self) -> Option<ExtXIndependentSegments> {
        self.independent_segments_tag
    }

    /// Returns the `EXT-X-START` tag contained in the playlist.
    pub const fn start_tag(&self) -> Option<ExtXStart> {
        self.start_tag
    }

    /// Returns the `EXT-X-MEDIA` tags contained in the playlist.
    pub fn media_tags(&self) -> &[ExtXMedia] {
        &self.media_tags
    }

    /// Returns the `EXT-X-STREAM-INF` tags contained in the playlist.
    pub fn stream_inf_tags(&self) -> &[ExtXStreamInf] {
        &self.stream_inf_tags
    }

    /// Returns the `EXT-X-I-FRAME-STREAM-INF` tags contained in the playlist.
    pub fn i_frame_stream_inf_tags(&self) -> &[ExtXIFrameStreamInf] {
        &self.i_frame_stream_inf_tags
    }

    /// Returns the `EXT-X-SESSION-DATA` tags contained in the playlist.
    pub fn session_data_tags(&self) -> &[ExtXSessionData] {
        &self.session_data_tags
    }

    /// Returns the `EXT-X-SESSION-KEY` tags contained in the playlist.
    pub fn session_key_tags(&self) -> &[ExtXSessionKey] {
        &self.session_key_tags
    }
}

impl MasterPlaylistBuilder {
    fn validate(&self) -> Result<(), String> {
        let required_version = self.required_version();
        let specified_version = self
            .version_tag
            .unwrap_or(required_version.into())
            .version();

        if required_version > specified_version {
            return Err(Error::required_version(required_version, specified_version).to_string());
        }

        self.validate_stream_inf_tags().map_err(|e| e.to_string())?;
        self.validate_i_frame_stream_inf_tags()
            .map_err(|e| e.to_string())?;
        self.validate_session_data_tags()
            .map_err(|e| e.to_string())?;
        self.validate_session_key_tags()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn required_version(&self) -> ProtocolVersion {
        iter::empty()
            .chain(
                self.independent_segments_tag
                    .iter()
                    .map(|t| t.iter().map(|t| t.requires_version()))
                    .flatten(),
            )
            .chain(
                self.start_tag
                    .iter()
                    .map(|t| t.iter().map(|t| t.requires_version()))
                    .flatten(),
            )
            .chain(
                self.media_tags
                    .iter()
                    .map(|t| t.iter().map(|t| t.requires_version()))
                    .flatten(),
            )
            .chain(
                self.stream_inf_tags
                    .iter()
                    .map(|t| t.iter().map(|t| t.requires_version()))
                    .flatten(),
            )
            .chain(
                self.i_frame_stream_inf_tags
                    .iter()
                    .map(|t| t.iter().map(|t| t.requires_version()))
                    .flatten(),
            )
            .chain(
                self.session_data_tags
                    .iter()
                    .map(|t| t.iter().map(|t| t.requires_version()))
                    .flatten(),
            )
            .chain(
                self.session_key_tags
                    .iter()
                    .map(|t| t.iter().map(|t| t.requires_version()))
                    .flatten(),
            )
            .max()
            .unwrap_or(ProtocolVersion::V7)
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
                    Some(&ClosedCaptions::GroupId(ref group_id)) => {
                        if !self.check_media_group(MediaType::ClosedCaptions, group_id) {
                            return Err(Error::unmatched_group(group_id));
                        }
                    }
                    Some(&ClosedCaptions::None) => {
                        has_none_closed_captions = true;
                    }
                    None => {}
                }
            }
            if has_none_closed_captions {
                if !value
                    .iter()
                    .all(|t| t.closed_captions() == Some(&ClosedCaptions::None))
                {
                    return Err(Error::invalid_input());
                }
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

    fn validate_session_key_tags(&self) -> crate::Result<()> {
        let mut set = HashSet::new();
        if let Some(value) = &self.session_key_tags {
            for t in value {
                if !set.insert(t.key()) {
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

impl fmt::Display for MasterPlaylist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", ExtM3u)?;
        if self.version_tag.version() != ProtocolVersion::V1 {
            writeln!(f, "{}", self.version_tag)?;
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
                        Tag::ExtXVersion(t) => {
                            builder.version(t.version());
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
        r#"#EXTM3U
#EXT-X-STREAM-INF:BANDWIDTH=150000,RESOLUTION=416x234,CODECS="avc1.42e00a,mp4a.40.2"
http://example.com/low/index.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=240000,RESOLUTION=416x234,CODECS="avc1.42e00a,mp4a.40.2"
http://example.com/lo_mid/index.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=440000,RESOLUTION=416x234,CODECS="avc1.42e00a,mp4a.40.2"
http://example.com/hi_mid/index.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=640000,RESOLUTION=640x360,CODECS="avc1.42e00a,mp4a.40.2"
http://example.com/high/index.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=64000,CODECS="mp4a.40.5"
http://example.com/audio/index.m3u8
"#
        .parse::<MasterPlaylist>()
        .unwrap();
    }

    #[test]
    fn test_display() {
        let input = r#"#EXTM3U
#EXT-X-STREAM-INF:BANDWIDTH=150000,RESOLUTION=416x234,CODECS="avc1.42e00a,mp4a.40.2"
http://example.com/low/index.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=240000,RESOLUTION=416x234,CODECS="avc1.42e00a,mp4a.40.2"
http://example.com/lo_mid/index.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=440000,RESOLUTION=416x234,CODECS="avc1.42e00a,mp4a.40.2"
http://example.com/hi_mid/index.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=640000,RESOLUTION=640x360,CODECS="avc1.42e00a,mp4a.40.2"
http://example.com/high/index.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=64000,CODECS="mp4a.40.5"
http://example.com/audio/index.m3u8
"#;
        let playlist = input.parse::<MasterPlaylist>().unwrap();
        assert_eq!(playlist.to_string(), input);
    }
}
