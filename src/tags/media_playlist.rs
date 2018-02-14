use std::fmt;
use std::str::FromStr;
use std::time::Duration;
use trackable::error::ErrorKindExt;

use {Error, ErrorKind, Result};
use types::{PlaylistType, ProtocolVersion};

/// [4.3.3.1. EXT-X-TARGETDURATION]
///
/// [4.3.3.1. EXT-X-TARGETDURATION]: https://tools.ietf.org/html/rfc8216#section-4.3.3.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXTargetDuration {
    duration: Duration,
}
impl ExtXTargetDuration {
    pub(crate) const PREFIX: &'static str = "#EXT-X-TARGETDURATION:";

    /// Makes a new `ExtXTargetduration` tag.
    ///
    /// Note that the nanoseconds part of the `duration` will be discarded.
    pub fn new(duration: Duration) -> Self {
        let duration = Duration::from_secs(duration.as_secs());
        ExtXTargetDuration { duration }
    }

    /// Returns the maximum media segment duration in the associated playlist.
    pub fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}
impl fmt::Display for ExtXTargetDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.duration.as_secs())
    }
}
impl FromStr for ExtXTargetDuration {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let duration = may_invalid!(s.split_at(Self::PREFIX.len()).1.parse())?;
        Ok(ExtXTargetDuration {
            duration: Duration::from_secs(duration),
        })
    }
}

/// [4.3.3.2. EXT-X-MEDIA-SEQUENCE]
///
/// [4.3.3.2. EXT-X-MEDIA-SEQUENCE]: https://tools.ietf.org/html/rfc8216#section-4.3.3.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXMediaSequence {
    seq_num: u64,
}
impl ExtXMediaSequence {
    pub(crate) const PREFIX: &'static str = "#EXT-X-MEDIA-SEQUENCE:";

    /// Makes a new `ExtXMediaSequence` tag.
    pub fn new(seq_num: u64) -> Self {
        ExtXMediaSequence { seq_num }
    }

    /// Returns the sequence number of the first media segment that appears in the associated playlist.
    pub fn seq_num(&self) -> u64 {
        self.seq_num
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}
impl fmt::Display for ExtXMediaSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.seq_num)
    }
}
impl FromStr for ExtXMediaSequence {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let seq_num = may_invalid!(s.split_at(Self::PREFIX.len()).1.parse())?;
        Ok(ExtXMediaSequence { seq_num })
    }
}

/// [4.3.3.3. EXT-X-DISCONTINUITY-SEQUENCE]
///
/// [4.3.3.3. EXT-X-DISCONTINUITY-SEQUENCE]: https://tools.ietf.org/html/rfc8216#section-4.3.3.3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXDiscontinuitySequence {
    seq_num: u64,
}
impl ExtXDiscontinuitySequence {
    pub(crate) const PREFIX: &'static str = "#EXT-X-DISCONTINUITY-SEQUENCE:";

    /// Makes a new `ExtXDiscontinuitySequence` tag.
    pub fn new(seq_num: u64) -> Self {
        ExtXDiscontinuitySequence { seq_num }
    }

    /// Returns the discontinuity sequence number of
    /// the first media segment that appears in the associated playlist.
    pub fn seq_num(&self) -> u64 {
        self.seq_num
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}
impl fmt::Display for ExtXDiscontinuitySequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.seq_num)
    }
}
impl FromStr for ExtXDiscontinuitySequence {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let seq_num = may_invalid!(s.split_at(Self::PREFIX.len()).1.parse())?;
        Ok(ExtXDiscontinuitySequence { seq_num })
    }
}

/// [4.3.3.4. EXT-X-ENDLIST]
///
/// [4.3.3.4. EXT-X-ENDLIST]: https://tools.ietf.org/html/rfc8216#section-4.3.3.4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXEndList;
impl ExtXEndList {
    pub(crate) const PREFIX: &'static str = "#EXT-X-ENDLIST";

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}
impl fmt::Display for ExtXEndList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Self::PREFIX.fmt(f)
    }
}
impl FromStr for ExtXEndList {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert_eq!(s, Self::PREFIX, ErrorKind::InvalidInput);
        Ok(ExtXEndList)
    }
}

/// [4.3.3.5. EXT-X-PLAYLIST-TYPE]
///
/// [4.3.3.5. EXT-X-PLAYLIST-TYPE]: https://tools.ietf.org/html/rfc8216#section-4.3.3.5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXPlaylistType {
    playlist_type: PlaylistType,
}
impl ExtXPlaylistType {
    pub(crate) const PREFIX: &'static str = "#EXT-X-PLAYLIST-TYPE:";

    /// Makes a new `ExtXPlaylistType` tag.
    pub fn new(playlist_type: PlaylistType) -> Self {
        ExtXPlaylistType { playlist_type }
    }

    /// Returns the type of the associated media playlist.
    pub fn playlist_type(&self) -> PlaylistType {
        self.playlist_type
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}
impl fmt::Display for ExtXPlaylistType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.playlist_type)
    }
}
impl FromStr for ExtXPlaylistType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let playlist_type = may_invalid!(s.split_at(Self::PREFIX.len()).1.parse())?;
        Ok(ExtXPlaylistType { playlist_type })
    }
}

/// [4.3.3.6. EXT-X-I-FRAMES-ONLY]
///
/// [4.3.3.6. EXT-X-I-FRAMES-ONLY]: https://tools.ietf.org/html/rfc8216#section-4.3.3.6
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXIFramesOnly;
impl ExtXIFramesOnly {
    pub(crate) const PREFIX: &'static str = "#EXT-X-I-FRAMES-ONLY";

    /// Returns the protocol compatibility version that this tag requires.
    pub fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V4
    }
}
impl fmt::Display for ExtXIFramesOnly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Self::PREFIX.fmt(f)
    }
}
impl FromStr for ExtXIFramesOnly {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert_eq!(s, Self::PREFIX, ErrorKind::InvalidInput);
        Ok(ExtXIFramesOnly)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_targetduration() {
        let tag = ExtXTargetDuration::new(Duration::from_secs(5));
        let text = "#EXT-X-TARGETDURATION:5";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }

    #[test]
    fn ext_x_media_sequence() {
        let tag = ExtXMediaSequence::new(123);
        let text = "#EXT-X-MEDIA-SEQUENCE:123";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }

    #[test]
    fn ext_x_discontinuity_sequence() {
        let tag = ExtXDiscontinuitySequence::new(123);
        let text = "#EXT-X-DISCONTINUITY-SEQUENCE:123";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }

    #[test]
    fn ext_x_endlist() {
        let tag = ExtXEndList;
        let text = "#EXT-X-ENDLIST";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }

    #[test]
    fn ext_x_playlist_type() {
        let tag = ExtXPlaylistType::new(PlaylistType::Vod);
        let text = "#EXT-X-PLAYLIST-TYPE:VOD";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }

    #[test]
    fn ext_i_frames_only() {
        let tag = ExtXIFramesOnly;
        let text = "#EXT-X-I-FRAMES-ONLY";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V4);
    }
}
