use std::fmt;
use std::str::FromStr;

use {Error, ErrorKind, Result};
use line::{Line, Lines};
use tag::{ExtM3u, ExtXIFrameStreamInf, ExtXIndependentSegments, ExtXMedia, ExtXSessionData,
          ExtXSessionKey, ExtXStart, ExtXStreamInf, ExtXVersion, Tag};
use types::ProtocolVersion;

#[derive(Debug, Clone)]
pub struct ExtXStreamInfWithUri {
    pub inf: ExtXStreamInf,
    pub uri: String,
}

#[derive(Debug, Clone)]
pub struct MasterPlaylist {
    pub version: ExtXVersion,
    pub media_tags: Vec<ExtXMedia>,
    pub stream_infs: Vec<ExtXStreamInfWithUri>,
    pub i_frame_stream_infs: Vec<ExtXIFrameStreamInf>,

    // TODO: A Playlist MUST NOT contain more than one EXT-X-
    // SESSION-DATA tag with the same DATA-ID attribute and the same
    // LANGUAGE attribute.
    pub session_data_tags: Vec<ExtXSessionData>,

    // TODO: A Master Playlist MUST NOT contain more than one EXT-X-SESSION-KEY
    // tag with the same METHOD, URI, IV, KEYFORMAT, and KEYFORMATVERSIONS
    // attribute values.
    pub session_keys: Vec<ExtXSessionKey>,

    pub independent_segments: Option<ExtXIndependentSegments>,
    pub start: Option<ExtXStart>,
}
impl fmt::Display for MasterPlaylist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", ExtM3u)?;
        if self.version.version() != ProtocolVersion::V1 {
            writeln!(f, "{}", self.version)?;
        }
        for t in &self.media_tags {
            writeln!(f, "{}", t)?;
        }
        for t in &self.stream_infs {
            writeln!(f, "{}", t.inf)?;
            writeln!(f, "{}", t.uri)?;
        }
        for t in &self.i_frame_stream_infs {
            writeln!(f, "{}", t)?;
        }
        for t in &self.session_data_tags {
            writeln!(f, "{}", t)?;
        }
        for t in &self.session_keys {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.independent_segments {
            writeln!(f, "{}", t)?;
        }
        if let Some(ref t) = self.start {
            writeln!(f, "{}", t)?;
        }
        Ok(())
    }
}
impl FromStr for MasterPlaylist {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut version = None;
        let mut media_tags = Vec::new();
        let mut stream_infs = Vec::new();
        let mut i_frame_stream_infs = Vec::new();
        let mut session_data_tags = Vec::new();
        let mut session_keys = Vec::new();
        let mut independent_segments = None;
        let mut start = None;

        let mut last_stream_inf = None;
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
                            media_tags.push(t);
                        }
                        Tag::ExtXStreamInf(t) => {
                            // TODO: It MUST match the value of the GROUP-ID attribute of an EXT-X-MEDIA tag
                            track_assert_eq!(last_stream_inf, None, ErrorKind::InvalidInput);
                            last_stream_inf = Some((i, t));
                        }
                        Tag::ExtXIFrameStreamInf(t) => {
                            // TODO: It MUST match the value of the GROUP-ID attribute of an EXT-X-MEDIA tag
                            i_frame_stream_infs.push(t);
                        }
                        Tag::ExtXSessionData(t) => {
                            session_data_tags.push(t);
                        }
                        Tag::ExtXSessionKey(t) => {
                            session_keys.push(t);
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
                    let (line, inf) = track_assert_some!(last_stream_inf, ErrorKind::InvalidInput);
                    track_assert_eq!(line + 1, i, ErrorKind::InvalidInput);
                    stream_infs.push(ExtXStreamInfWithUri {
                        inf,
                        uri: uri.to_owned(),
                    });
                    last_stream_inf = None;
                }
            }
        }

        // TODO: check compatibility
        Ok(MasterPlaylist {
            version: version.unwrap_or_else(|| ExtXVersion::new(ProtocolVersion::V1)),
            media_tags,
            stream_infs,
            i_frame_stream_infs,
            session_data_tags,
            session_keys,
            independent_segments,
            start,
        })
    }
}
