use std::fmt;
use std::str::FromStr;
use std::time::Duration;
use trackable::error::ErrorKindExt;

use {Error, ErrorKind, Result};
use attribute::{AttributePairs, QuotedString};
use string::M3u8String;
use version::ProtocolVersion;

macro_rules! may_invalid {
    ($expr:expr) => {
        $expr.map_err(|e| track!(Error::from(ErrorKind::InvalidInput.cause(e))))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    ExtM3u(ExtM3u),
    ExtXVersion(ExtXVersion),
    ExtInf(ExtInf),
    ExtXByteRange(ExtXByteRange),
    ExtXDiscontinuity(ExtXDiscontinuity),
    ExtXKey(ExtXKey),
    ExtXTargetDuration(ExtXTargetDuration),
    ExtXMediaSequence(ExtXMediaSequence),
    ExtXDiscontinuitySequence(ExtXDiscontinuitySequence),
    ExtXEndList(ExtXEndList),
    ExtXPlaylistType(ExtXPlaylistType),
    ExtXIFramesOnly(ExtXIFramesOnly),
    ExtXIndependentSegments(ExtXIndependentSegments),
}
impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tag::ExtM3u(ref t) => t.fmt(f),
            Tag::ExtXVersion(ref t) => t.fmt(f),
            Tag::ExtInf(ref t) => t.fmt(f),
            Tag::ExtXByteRange(ref t) => t.fmt(f),
            Tag::ExtXDiscontinuity(ref t) => t.fmt(f),
            Tag::ExtXKey(ref t) => t.fmt(f),
            Tag::ExtXTargetDuration(ref t) => t.fmt(f),
            Tag::ExtXMediaSequence(ref t) => t.fmt(f),
            Tag::ExtXDiscontinuitySequence(ref t) => t.fmt(f),
            Tag::ExtXEndList(ref t) => t.fmt(f),
            Tag::ExtXPlaylistType(ref t) => t.fmt(f),
            Tag::ExtXIFramesOnly(ref t) => t.fmt(f),
            Tag::ExtXIndependentSegments(ref t) => t.fmt(f),
        }
    }
}
impl FromStr for Tag {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with(ExtM3u::PREFIX) {
            track!(s.parse().map(Tag::ExtM3u))
        } else if s.starts_with(ExtXVersion::PREFIX) {
            track!(s.parse().map(Tag::ExtXVersion))
        } else if s.starts_with(ExtInf::PREFIX) {
            track!(s.parse().map(Tag::ExtInf))
        } else if s.starts_with(ExtXByteRange::PREFIX) {
            track!(s.parse().map(Tag::ExtXByteRange))
        } else if s.starts_with(ExtXDiscontinuity::PREFIX) {
            track!(s.parse().map(Tag::ExtXDiscontinuity))
        } else if s.starts_with(ExtXKey::PREFIX) {
            track!(s.parse().map(Tag::ExtXKey))
        } else if s.starts_with(ExtXTargetDuration::PREFIX) {
            track!(s.parse().map(Tag::ExtXTargetDuration))
        } else if s.starts_with(ExtXMediaSequence::PREFIX) {
            track!(s.parse().map(Tag::ExtXMediaSequence))
        } else if s.starts_with(ExtXDiscontinuitySequence::PREFIX) {
            track!(s.parse().map(Tag::ExtXDiscontinuitySequence))
        } else if s.starts_with(ExtXEndList::PREFIX) {
            track!(s.parse().map(Tag::ExtXEndList))
        } else if s.starts_with(ExtXPlaylistType::PREFIX) {
            track!(s.parse().map(Tag::ExtXPlaylistType))
        } else if s.starts_with(ExtXIFramesOnly::PREFIX) {
            track!(s.parse().map(Tag::ExtXIFramesOnly))
        } else if s.starts_with(ExtXIndependentSegments::PREFIX) {
            track!(s.parse().map(Tag::ExtXIndependentSegments))
        } else {
            // TODO: ignore any unrecognized tags. (section-6.3.1)
            track_panic!(ErrorKind::InvalidInput, "Unknown tag: {:?}", s)
        }
    }
}

// TODO: MediaSegmentTag

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtM3u;
impl ExtM3u {
    const PREFIX: &'static str = "#EXTM3U";
}
impl fmt::Display for ExtM3u {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Self::PREFIX.fmt(f)
    }
}
impl FromStr for ExtM3u {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert_eq!(s, Self::PREFIX, ErrorKind::InvalidInput);
        Ok(ExtM3u)
    }
}

// TODO:  A Playlist file MUST NOT contain more than one EXT-X-VERSION tag
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXVersion {
    pub version: ProtocolVersion,
}
impl ExtXVersion {
    const PREFIX: &'static str = "#EXT-X-VERSION:";
}
impl fmt::Display for ExtXVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.version)
    }
}
impl FromStr for ExtXVersion {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let suffix = s.split_at(Self::PREFIX.len()).1;
        let version = track!(suffix.parse())?;
        Ok(ExtXVersion { version })
    }
}

// TODO: This tag is REQUIRED for each Media Segment
// TODO: if the compatibility version number is less than 3, durations MUST be integers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtInf {
    pub duration: Duration,
    pub title: Option<M3u8String>,
}
impl ExtInf {
    const PREFIX: &'static str = "#EXTINF:";

    // TODO: pub fn required_version(&self) -> ProtocolVersion;
}
impl fmt::Display for ExtInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;

        let duration = (self.duration.as_secs() as f64)
            + (self.duration.subsec_nanos() as f64 / 1_000_000_000.0);
        write!(f, "{}", duration)?;

        if let Some(ref title) = self.title {
            write!(f, ",{}", title)?;
        }
        Ok(())
    }
}
impl FromStr for ExtInf {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let mut tokens = s.split_at(Self::PREFIX.len()).1.splitn(2, ',');

        let duration: f64 = may_invalid!(tokens.next().expect("Never fails").parse())?;
        let duration = Duration::new(duration as u64, (duration.fract() * 1_000_000_000.0) as u32);

        let title = if let Some(title) = tokens.next() {
            Some(track!(M3u8String::new(title))?)
        } else {
            None
        };
        Ok(ExtInf { duration, title })
    }
}

// TODO: If o is not present, a previous Media Segment MUST appear in the Playlist file
// TDOO: Use of the EXT-X-BYTERANGE tag REQUIRES a compatibility version number of 4 or greater.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXByteRange {
    pub length: usize,
    pub offset: Option<usize>,
}
impl ExtXByteRange {
    const PREFIX: &'static str = "#EXT-X-BYTERANGE:";
}
impl fmt::Display for ExtXByteRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.length)?;
        if let Some(offset) = self.offset {
            write!(f, "@{}", offset)?;
        }
        Ok(())
    }
}
impl FromStr for ExtXByteRange {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let mut tokens = s.split_at(Self::PREFIX.len()).1.splitn(2, '@');

        let length = may_invalid!(tokens.next().expect("Never fails").parse())?;
        let offset = if let Some(offset) = tokens.next() {
            Some(may_invalid!(offset.parse())?)
        } else {
            None
        };
        Ok(ExtXByteRange { length, offset })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXDiscontinuity;
impl ExtXDiscontinuity {
    const PREFIX: &'static str = "#EXT-X-DISCONTINUITY";
}
impl fmt::Display for ExtXDiscontinuity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Self::PREFIX.fmt(f)
    }
}
impl FromStr for ExtXDiscontinuity {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert_eq!(s, Self::PREFIX, ErrorKind::InvalidInput);
        Ok(ExtXDiscontinuity)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXKey {
    pub method: EncryptionMethod,
    pub uri: Option<QuotedString>,
}
impl ExtXKey {
    const PREFIX: &'static str = "#EXT-X-KEY:";
}
impl fmt::Display for ExtXKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        unimplemented!()
    }
}
impl FromStr for ExtXKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut method = None;
        let mut uri = None;
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "METHOD" => {
                    method = Some(track!(value.parse())?);
                }
                "URI" => {
                    uri = Some(track!(value.parse())?);
                }
                "IV" => unimplemented!(),
                "KEYFORMAT" => unimplemented!(),
                "KEYFORMATVERSIONS" => unimplemented!(),
                _ => {
                    // [6.3.1] ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }
        let method = track_assert_some!(method, ErrorKind::InvalidInput);
        if let EncryptionMethod::None = method {
            track_assert_eq!(uri, None, ErrorKind::InvalidInput);
        } else {
            track_assert!(uri.is_some(), ErrorKind::InvalidInput);
        };
        Ok(ExtXKey { method, uri })
    }
}

// TODO: move
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncryptionMethod {
    None,
    Aes128,
    SampleAes,
}
impl fmt::Display for EncryptionMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncryptionMethod::None => "NONE".fmt(f),
            EncryptionMethod::Aes128 => "AES-128".fmt(f),
            EncryptionMethod::SampleAes => "SAMPLE-AES".fmt(f),
        }
    }
}
impl FromStr for EncryptionMethod {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "NONE" => Ok(EncryptionMethod::None),
            "AES-128" => Ok(EncryptionMethod::Aes128),
            "SAMPLE-AES" => Ok(EncryptionMethod::SampleAes),
            _ => track_panic!(
                ErrorKind::InvalidInput,
                "Unknown encryption method: {:?}",
                s
            ),
        }
    }
}

// TODO: https://tools.ietf.org/html/rfc8216#section-4.3.2.5

// TODO:
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct ExtXProgramDateTime { date_time }
// impl ExtXProgramDateTime {
//     const PREFIX: &'static str = "#EXT-X-PROGRAM-DATE-TIME:";
// }
// impl fmt::Display for ExtXProgramDateTime {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}{}", Self::PREFIX, self.length)?;
//         if let Some(offset) = self.offset {
//             write!(f, "@{}", offset)?;
//         }
//         Ok(())
//     }
// }
// impl FromStr for ExtXProgramDateTime {
//     type Err = Error;
//     fn from_str(s: &str) -> Result<Self> {
//         track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
//         let mut tokens = s.split_at(Self::PREFIX.len()).1.splitn(2, '@');

//         let length = may_invalid!(tokens.next().expect("Never fails").parse())?;
//         let offset = if let Some(offset) = tokens.next() {
//             Some(may_invalid!(offset.parse())?)
//         } else {
//             None
//         };
//         Ok(ExtXByteRange { length, offset })
//     }
// }

// TODO: https://tools.ietf.org/html/rfc8216#section-4.3.2.7

// TODO: he EXT-X-TARGETDURATION tag is REQUIRED.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXTargetDuration {
    pub duration: Duration,
}
impl ExtXTargetDuration {
    const PREFIX: &'static str = "#EXT-X-TARGETDURATION:";
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

// TODO: The EXT-X-MEDIA-SEQUENCE tag MUST appear before the first Media Segment in the Playlist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXMediaSequence {
    pub seq_num: u64,
}
impl ExtXMediaSequence {
    const PREFIX: &'static str = "#EXT-X-MEDIA-SEQUENCE:";
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

// TODO: The EXT-X-DISCONTINUITY-SEQUENCE tag MUST appear before the first Media Segment in the Playlist.
// TODO: The EXT-X-DISCONTINUITY-SEQUENCE tag MUST appear before any EXT-X-DISCONTINUITY tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXDiscontinuitySequence {
    pub seq_num: u64,
}
impl ExtXDiscontinuitySequence {
    const PREFIX: &'static str = "#EXT-X-DISCONTINUITY-SEQUENCE:";
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXEndList;
impl ExtXEndList {
    const PREFIX: &'static str = "#EXT-X-ENDLIST";
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXPlaylistType {
    pub playlist_type: PlaylistType,
}
impl ExtXPlaylistType {
    const PREFIX: &'static str = "#EXT-X-PLAYLIST-TYPE:";
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaylistType {
    Event,
    Vod,
}
impl fmt::Display for PlaylistType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlaylistType::Event => write!(f, "EVENT"),
            PlaylistType::Vod => write!(f, "VOD"),
        }
    }
}
impl FromStr for PlaylistType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "EVENT" => Ok(PlaylistType::Event),
            "VOD" => Ok(PlaylistType::Vod),
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown playlist type: {:?}", s),
        }
    }
}

// TODO: Media resources containing I-frame segments MUST begin with ...
// TODO: Use of the EXT-X-I-FRAMES-ONLY REQUIRES a compatibility version number of 4 or greater.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXIFramesOnly;
impl ExtXIFramesOnly {
    const PREFIX: &'static str = "#EXT-X-I-FRAMES-ONLY";
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

// TODO: https://tools.ietf.org/html/rfc8216#section-4.3.4.1

// TODO: https://tools.ietf.org/html/rfc8216#section-4.3.4.2

// TODO: https://tools.ietf.org/html/rfc8216#section-4.3.4.3

// TODO: https://tools.ietf.org/html/rfc8216#section-4.3.4.4

// TODO: https://tools.ietf.org/html/rfc8216#section-4.3.4.5

// 4.3.5.  Media or Master Playlist Tags
// TODO: A tag that appears in both MUST have the same value; otherwise, clients SHOULD ignore the value in the Media Playlist(s).
// TODO: These tags MUST NOT appear more than once in a Playlist.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXIndependentSegments;
impl ExtXIndependentSegments {
    const PREFIX: &'static str = "#EXT-X-INDEPENDENT-SEGMENTS";
}
impl fmt::Display for ExtXIndependentSegments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Self::PREFIX.fmt(f)
    }
}
impl FromStr for ExtXIndependentSegments {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert_eq!(s, Self::PREFIX, ErrorKind::InvalidInput);
        Ok(ExtXIndependentSegments)
    }
}

// TODO: https://tools.ietf.org/html/rfc8216#section-4.3.5.2
