use std::collections::HashSet;
use std::fmt;
use std::iter;
use std::str::FromStr;

use line::{Line, Lines, Tag};
use tags::{
    ExtM3u, ExtXIFrameStreamInf, ExtXIndependentSegments, ExtXMedia, ExtXSessionData,
    ExtXSessionKey, ExtXStart, ExtXStreamInf, ExtXVersion, MasterPlaylistTag,
};
use types::{ClosedCaptions, MediaType, ProtocolVersion, QuotedString};
use {Error, ErrorKind, Result};

/// Master playlist builder.
#[derive(Debug, Clone)]
pub struct MasterPlaylistBuilder {
    version: Option<ProtocolVersion>,
    independent_segments_tag: Option<ExtXIndependentSegments>,
    start_tag: Option<ExtXStart>,
    media_tags: Vec<ExtXMedia>,
    stream_inf_tags: Vec<ExtXStreamInf>,
    i_frame_stream_inf_tags: Vec<ExtXIFrameStreamInf>,
    session_data_tags: Vec<ExtXSessionData>,
    session_key_tags: Vec<ExtXSessionKey>,
}
impl MasterPlaylistBuilder {
    /// Makes a new `MasterPlaylistBuilder` instance.
    pub fn new() -> Self {
        MasterPlaylistBuilder {
            version: None,
            independent_segments_tag: None,
            start_tag: None,
            media_tags: Vec::new(),
            stream_inf_tags: Vec::new(),
            i_frame_stream_inf_tags: Vec::new(),
            session_data_tags: Vec::new(),
            session_key_tags: Vec::new(),
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

    /// Adds the given tag to the resulting playlist.
    ///
    /// If it is forbidden to have multiple instance of the tag, the existing one will be overwritten.
    pub fn tag<T: Into<MasterPlaylistTag>>(&mut self, tag: T) -> &mut Self {
        match tag.into() {
            MasterPlaylistTag::ExtXIndependentSegments(t) => {
                self.independent_segments_tag = Some(t);
            }
            MasterPlaylistTag::ExtXStart(t) => self.start_tag = Some(t),
            MasterPlaylistTag::ExtXMedia(t) => self.media_tags.push(t),
            MasterPlaylistTag::ExtXStreamInf(t) => self.stream_inf_tags.push(t),
            MasterPlaylistTag::ExtXIFrameStreamInf(t) => self.i_frame_stream_inf_tags.push(t),
            MasterPlaylistTag::ExtXSessionData(t) => self.session_data_tags.push(t),
            MasterPlaylistTag::ExtXSessionKey(t) => self.session_key_tags.push(t),
        }
        self
    }

    /// Builds a `MasterPlaylist` instance.
    pub fn finish(self) -> Result<MasterPlaylist> {
        let required_version = self.required_version();
        let specified_version = self.version.unwrap_or(required_version);
        track_assert!(
            required_version <= specified_version,
            ErrorKind::InvalidInput,
            "required_version:{}, specified_version:{}",
            required_version,
            specified_version,
        );

        track!(self.validate_stream_inf_tags())?;
        track!(self.validate_i_frame_stream_inf_tags())?;
        track!(self.validate_session_data_tags())?;
        track!(self.validate_session_key_tags())?;

        Ok(MasterPlaylist {
            version_tag: ExtXVersion::new(specified_version),
            independent_segments_tag: self.independent_segments_tag,
            start_tag: self.start_tag,
            media_tags: self.media_tags,
            stream_inf_tags: self.stream_inf_tags,
            i_frame_stream_inf_tags: self.i_frame_stream_inf_tags,
            session_data_tags: self.session_data_tags,
            session_key_tags: self.session_key_tags,
        })
    }

    fn required_version(&self) -> ProtocolVersion {
        iter::empty()
            .chain(
                self.independent_segments_tag
                    .iter()
                    .map(|t| t.requires_version()),
            ).chain(self.start_tag.iter().map(|t| t.requires_version()))
            .chain(self.media_tags.iter().map(|t| t.requires_version()))
            .chain(self.stream_inf_tags.iter().map(|t| t.requires_version()))
            .chain(
                self.i_frame_stream_inf_tags
                    .iter()
                    .map(|t| t.requires_version()),
            ).chain(self.session_data_tags.iter().map(|t| t.requires_version()))
            .chain(self.session_key_tags.iter().map(|t| t.requires_version()))
            .max()
            .expect("Never fails")
    }

    fn validate_stream_inf_tags(&self) -> Result<()> {
        let mut has_none_closed_captions = false;
        for t in &self.stream_inf_tags {
            if let Some(group_id) = t.audio() {
                track_assert!(
                    self.check_media_group(MediaType::Audio, group_id),
                    ErrorKind::InvalidInput,
                    "Unmatched audio group: {:?}",
                    group_id
                );
            }
            if let Some(group_id) = t.video() {
                track_assert!(
                    self.check_media_group(MediaType::Video, group_id),
                    ErrorKind::InvalidInput,
                    "Unmatched video group: {:?}",
                    group_id
                );
            }
            if let Some(group_id) = t.subtitles() {
                track_assert!(
                    self.check_media_group(MediaType::Subtitles, group_id),
                    ErrorKind::InvalidInput,
                    "Unmatched subtitles group: {:?}",
                    group_id
                );
            }
            match t.closed_captions() {
                Some(&ClosedCaptions::GroupId(ref group_id)) => {
                    track_assert!(
                        self.check_media_group(MediaType::ClosedCaptions, group_id),
                        ErrorKind::InvalidInput,
                        "Unmatched closed-captions group: {:?}",
                        group_id
                    );
                }
                Some(&ClosedCaptions::None) => {
                    has_none_closed_captions = true;
                }
                None => {}
            }
        }
        if has_none_closed_captions {
            track_assert!(
                self.stream_inf_tags
                    .iter()
                    .all(|t| t.closed_captions() == Some(&ClosedCaptions::None)),
                ErrorKind::InvalidInput
            );
        }
        Ok(())
    }

    fn validate_i_frame_stream_inf_tags(&self) -> Result<()> {
        for t in &self.i_frame_stream_inf_tags {
            if let Some(group_id) = t.video() {
                track_assert!(
                    self.check_media_group(MediaType::Video, group_id),
                    ErrorKind::InvalidInput,
                    "Unmatched video group: {:?}",
                    group_id
                );
            }
        }
        Ok(())
    }

    fn validate_session_data_tags(&self) -> Result<()> {
        let mut set = HashSet::new();
        for t in &self.session_data_tags {
            track_assert!(
                set.insert((t.data_id(), t.language())),
                ErrorKind::InvalidInput,
                "Conflict: {}",
                t
            );
        }
        Ok(())
    }

    fn validate_session_key_tags(&self) -> Result<()> {
        let mut set = HashSet::new();
        for t in &self.session_key_tags {
            track_assert!(
                set.insert(t.key()),
                ErrorKind::InvalidInput,
                "Conflict: {}",
                t
            );
        }
        Ok(())
    }

    fn check_media_group(&self, media_type: MediaType, group_id: &QuotedString) -> bool {
        self.media_tags
            .iter()
            .any(|t| t.media_type() == media_type && t.group_id() == group_id)
    }
}
impl Default for MasterPlaylistBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Master playlist.
#[derive(Debug, Clone)]
pub struct MasterPlaylist {
    version_tag: ExtXVersion,
    independent_segments_tag: Option<ExtXIndependentSegments>,
    start_tag: Option<ExtXStart>,
    media_tags: Vec<ExtXMedia>,
    stream_inf_tags: Vec<ExtXStreamInf>,
    i_frame_stream_inf_tags: Vec<ExtXIFrameStreamInf>,
    session_data_tags: Vec<ExtXSessionData>,
    session_key_tags: Vec<ExtXSessionKey>,
}
impl MasterPlaylist {
    /// Returns the `EXT-X-VERSION` tag contained in the playlist.
    pub fn version_tag(&self) -> ExtXVersion {
        self.version_tag
    }

    /// Returns the `EXT-X-INDEPENDENT-SEGMENTS` tag contained in the playlist.
    pub fn independent_segments_tag(&self) -> Option<ExtXIndependentSegments> {
        self.independent_segments_tag
    }

    /// Returns the `EXT-X-START` tag contained in the playlist.
    pub fn start_tag(&self) -> Option<ExtXStart> {
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
    pub fn i_fream_stream_inf_tags(&self) -> &[ExtXIFrameStreamInf] {
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
        if let Some(ref t) = self.independent_segments_tag {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.start_tag {
            writeln!(f, "{}", t)?;
        }
        Ok(())
    }
}
impl FromStr for MasterPlaylist {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut builder = MasterPlaylistBuilder::new();
        for (i, line) in Lines::new(s).enumerate() {
            match track!(line)? {
                Line::Blank | Line::Comment(_) => {}
                Line::Tag(tag) => {
                    if i == 0 {
                        track_assert_eq!(tag, Tag::ExtM3u(ExtM3u), ErrorKind::InvalidInput);
                        continue;
                    }
                    match tag {
                        Tag::ExtM3u(_) => {
                            track_panic!(ErrorKind::InvalidInput);
                        }
                        Tag::ExtXVersion(t) => {
                            track_assert_eq!(builder.version, None, ErrorKind::InvalidInput);
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
                            track_panic!(ErrorKind::InvalidInput, "{}", tag)
                        }
                        Tag::ExtXMedia(t) => {
                            builder.tag(t);
                        }
                        Tag::ExtXStreamInf(t) => {
                            builder.tag(t);
                        }
                        Tag::ExtXIFrameStreamInf(t) => {
                            builder.tag(t);
                        }
                        Tag::ExtXSessionData(t) => {
                            builder.tag(t);
                        }
                        Tag::ExtXSessionKey(t) => {
                            builder.tag(t);
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
                    track_panic!(ErrorKind::InvalidInput, "Unexpected URI: {:?}", uri);
                }
            }
        }
        track!(builder.finish())
    }
}
