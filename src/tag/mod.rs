//! [4.3. Playlist Tags]
//!
//! [4.3. Playlist Tags]: https://tools.ietf.org/html/rfc8216#section-4.3
use std::fmt;
use std::str::FromStr;
use std::time::Duration;
use trackable::error::ErrorKindExt;

use {Error, ErrorKind, Result};
use attribute::{AttributePairs, DecimalFloatingPoint, DecimalInteger, DecimalResolution,
                HexadecimalSequence, QuotedString, SignedDecimalFloatingPoint};

macro_rules! may_invalid {
    ($expr:expr) => {
        $expr.map_err(|e| track!(Error::from(ErrorKind::InvalidInput.cause(e))))
    }
}

macro_rules! impl_from {
    ($to:ident, $from:ident) => {
        impl From<$from> for $to {
            fn from(f: $from) -> Self {
                $to::$from(f)
            }
        }
    }
}

pub use self::basic::{ExtM3u, ExtXVersion};
pub use self::media_segment::{ExtInf, ExtXByteRange, ExtXDateRange, ExtXDiscontinuity, ExtXKey,
                              ExtXMap, ExtXProgramDateTime};

mod basic;
mod media_segment;

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaSegmentTag {
    ExtInf(ExtInf),
    ExtXByteRange(ExtXByteRange),
    ExtXDateRange(ExtXDateRange),
    ExtXDiscontinuity(ExtXDiscontinuity),
    ExtXKey(ExtXKey),
    ExtXMap(ExtXMap),
    ExtXProgramDateTime(ExtXProgramDateTime),
}
// TODO: delete
#[allow(missing_docs)]
impl MediaSegmentTag {
    pub fn as_inf(&self) -> Option<&ExtInf> {
        if let MediaSegmentTag::ExtInf(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
    pub fn as_byte_range(&self) -> Option<&ExtXByteRange> {
        if let MediaSegmentTag::ExtXByteRange(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
    pub fn as_date_range(&self) -> Option<&ExtXDateRange> {
        if let MediaSegmentTag::ExtXDateRange(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
    pub fn as_discontinuity(&self) -> Option<&ExtXDiscontinuity> {
        if let MediaSegmentTag::ExtXDiscontinuity(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
    pub fn as_key(&self) -> Option<&ExtXKey> {
        if let MediaSegmentTag::ExtXKey(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
    pub fn as_map(&self) -> Option<&ExtXMap> {
        if let MediaSegmentTag::ExtXMap(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
    pub fn as_program_date_time(&self) -> Option<&ExtXProgramDateTime> {
        if let MediaSegmentTag::ExtXProgramDateTime(ref t) = *self {
            Some(t)
        } else {
            None
        }
    }
}
impl fmt::Display for MediaSegmentTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MediaSegmentTag::ExtInf(ref t) => t.fmt(f),
            MediaSegmentTag::ExtXByteRange(ref t) => t.fmt(f),
            MediaSegmentTag::ExtXDateRange(ref t) => t.fmt(f),
            MediaSegmentTag::ExtXDiscontinuity(ref t) => t.fmt(f),
            MediaSegmentTag::ExtXKey(ref t) => t.fmt(f),
            MediaSegmentTag::ExtXMap(ref t) => t.fmt(f),
            MediaSegmentTag::ExtXProgramDateTime(ref t) => t.fmt(f),
        }
    }
}
impl_from!(MediaSegmentTag, ExtInf);
impl_from!(MediaSegmentTag, ExtXByteRange);
impl_from!(MediaSegmentTag, ExtXDateRange);
impl_from!(MediaSegmentTag, ExtXDiscontinuity);
impl_from!(MediaSegmentTag, ExtXKey);
impl_from!(MediaSegmentTag, ExtXMap);
impl_from!(MediaSegmentTag, ExtXProgramDateTime);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    ExtM3u(ExtM3u),
    ExtXVersion(ExtXVersion),
    ExtInf(ExtInf),
    ExtXByteRange(ExtXByteRange),
    ExtXDiscontinuity(ExtXDiscontinuity),
    ExtXKey(ExtXKey),
    ExtXMap(ExtXMap),
    ExtXProgramDateTime(ExtXProgramDateTime),
    ExtXDateRange(ExtXDateRange),
    ExtXTargetDuration(ExtXTargetDuration),
    ExtXMediaSequence(ExtXMediaSequence),
    ExtXDiscontinuitySequence(ExtXDiscontinuitySequence),
    ExtXEndList(ExtXEndList),
    ExtXPlaylistType(ExtXPlaylistType),
    ExtXIFramesOnly(ExtXIFramesOnly),
    ExtXMedia(ExtXMedia),
    ExtXStreamInf(ExtXStreamInf),
    ExtXIFrameStreamInf(ExtXIFrameStreamInf),
    ExtXSessionData(ExtXSessionData),
    ExtXSessionKey(ExtXSessionKey),
    ExtXIndependentSegments(ExtXIndependentSegments),
    ExtXStart(ExtXStart),
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
            Tag::ExtXMap(ref t) => t.fmt(f),
            Tag::ExtXProgramDateTime(ref t) => t.fmt(f),
            Tag::ExtXDateRange(ref t) => t.fmt(f),
            Tag::ExtXTargetDuration(ref t) => t.fmt(f),
            Tag::ExtXMediaSequence(ref t) => t.fmt(f),
            Tag::ExtXDiscontinuitySequence(ref t) => t.fmt(f),
            Tag::ExtXEndList(ref t) => t.fmt(f),
            Tag::ExtXPlaylistType(ref t) => t.fmt(f),
            Tag::ExtXIFramesOnly(ref t) => t.fmt(f),
            Tag::ExtXMedia(ref t) => t.fmt(f),
            Tag::ExtXStreamInf(ref t) => t.fmt(f),
            Tag::ExtXIFrameStreamInf(ref t) => t.fmt(f),
            Tag::ExtXSessionData(ref t) => t.fmt(f),
            Tag::ExtXSessionKey(ref t) => t.fmt(f),
            Tag::ExtXIndependentSegments(ref t) => t.fmt(f),
            Tag::ExtXStart(ref t) => t.fmt(f),
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
        } else if s.starts_with(ExtXMap::PREFIX) {
            track!(s.parse().map(Tag::ExtXMap))
        } else if s.starts_with(ExtXProgramDateTime::PREFIX) {
            track!(s.parse().map(Tag::ExtXProgramDateTime))
        } else if s.starts_with(ExtXTargetDuration::PREFIX) {
            track!(s.parse().map(Tag::ExtXTargetDuration))
        } else if s.starts_with(ExtXDateRange::PREFIX) {
            track!(s.parse().map(Tag::ExtXDateRange))
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
        } else if s.starts_with(ExtXMedia::PREFIX) {
            track!(s.parse().map(Tag::ExtXMedia))
        } else if s.starts_with(ExtXStreamInf::PREFIX) {
            track!(s.parse().map(Tag::ExtXStreamInf))
        } else if s.starts_with(ExtXIFrameStreamInf::PREFIX) {
            track!(s.parse().map(Tag::ExtXIFrameStreamInf))
        } else if s.starts_with(ExtXSessionData::PREFIX) {
            track!(s.parse().map(Tag::ExtXSessionData))
        } else if s.starts_with(ExtXSessionKey::PREFIX) {
            track!(s.parse().map(Tag::ExtXSessionKey))
        } else if s.starts_with(ExtXIndependentSegments::PREFIX) {
            track!(s.parse().map(Tag::ExtXIndependentSegments))
        } else if s.starts_with(ExtXStart::PREFIX) {
            track!(s.parse().map(Tag::ExtXStart))
        } else {
            // TODO: ignore any unrecognized tags. (section-6.3.1)
            track_panic!(ErrorKind::InvalidInput, "Unknown tag: {:?}", s)
        }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MediaType {
    Audio,
    Video,
    Subtitles,
    ClosedCaptions,
}
impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MediaType::Audio => "AUDIO".fmt(f),
            MediaType::Video => "VIDEO".fmt(f),
            MediaType::Subtitles => "SUBTITLES".fmt(f),
            MediaType::ClosedCaptions => "CLOSED-CAPTIONS".fmt(f),
        }
    }
}
impl FromStr for MediaType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "AUDIO" => MediaType::Audio,
            "VIDEO" => MediaType::Video,
            "SUBTITLES" => MediaType::Subtitles,
            "CLOSED-CAPTIONS" => MediaType::ClosedCaptions,
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown media type: {:?}", s),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YesOrNo {
    Yes,
    No,
}
impl fmt::Display for YesOrNo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            YesOrNo::Yes => "YES".fmt(f),
            YesOrNo::No => "NO".fmt(f),
        }
    }
}
impl FromStr for YesOrNo {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "YES" => Ok(YesOrNo::Yes),
            "NO" => Ok(YesOrNo::No),
            _ => track_panic!(
                ErrorKind::InvalidInput,
                "Unexpected enumerated-string: {:?}",
                s
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXMedia {
    media_type: MediaType,
    uri: Option<QuotedString>,
    group_id: QuotedString,
    language: Option<QuotedString>,
    assoc_language: Option<QuotedString>,
    name: QuotedString,
    default: YesOrNo,
    autoselect: YesOrNo,
    forced: Option<YesOrNo>,
    instream_id: Option<QuotedString>, // TODO: `InStreamId` type
    characteristics: Option<QuotedString>,
    channels: Option<QuotedString>,
}
impl ExtXMedia {
    const PREFIX: &'static str = "#EXT-X-MEDIA:";
}
impl fmt::Display for ExtXMedia {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "TYPE={}", self.media_type)?;
        if let Some(ref x) = self.uri {
            write!(f, ",URI={}", x)?;
        }
        write!(f, ",GROUP_ID={}", self.group_id)?;
        if let Some(ref x) = self.language {
            write!(f, ",LANGUAGE={}", x)?;
        }
        if let Some(ref x) = self.assoc_language {
            write!(f, ",ASSOC-LANGUAGE={}", x)?;
        }
        write!(f, ",NAME={}", self.name)?;
        if YesOrNo::Yes == self.default {
            write!(f, ",DEFAULT={}", self.default)?;
        }
        if YesOrNo::Yes == self.autoselect {
            write!(f, ",AUTOSELECT={}", self.autoselect)?;
        }
        if let Some(ref x) = self.forced {
            write!(f, ",FORCED={}", x)?;
        }
        if let Some(ref x) = self.instream_id {
            write!(f, ",INSTREAM-ID={}", x)?;
        }
        if let Some(ref x) = self.characteristics {
            write!(f, ",CHARACTERISTICS={}", x)?;
        }
        if let Some(ref x) = self.channels {
            write!(f, ",CHANNELS={}", x)?;
        }
        Ok(())
    }
}
impl FromStr for ExtXMedia {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut media_type = None;
        let mut uri = None;
        let mut group_id = None;
        let mut language = None;
        let mut assoc_language = None;
        let mut name = None;
        let mut default = None;
        let mut autoselect = None;
        let mut forced = None;
        let mut instream_id = None;
        let mut characteristics = None;
        let mut channels = None;
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "TYPE" => media_type = Some(track!(value.parse())?),
                "URI" => uri = Some(track!(value.parse())?),
                "GROUP-ID" => group_id = Some(track!(value.parse())?),
                "LANGUAGE" => language = Some(track!(value.parse())?),
                "ASSOC-LANGUAGE" => assoc_language = Some(track!(value.parse())?),
                "NAME" => name = Some(track!(value.parse())?),
                "DEFAULT" => default = Some(track!(value.parse())?),
                "AUTOSELECT" => autoselect = Some(track!(value.parse())?),
                "FORCED" => forced = Some(track!(value.parse())?),
                "INSTREAM-ID" => instream_id = Some(track!(value.parse())?),
                "CHARACTERISTICS" => characteristics = Some(track!(value.parse())?),
                "CHANNELS" => channels = Some(track!(value.parse())?),
                _ => {
                    // [6.3.1] ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }
        let media_type = track_assert_some!(media_type, ErrorKind::InvalidInput);
        let group_id = track_assert_some!(group_id, ErrorKind::InvalidInput);
        let name = track_assert_some!(name, ErrorKind::InvalidInput);
        if MediaType::ClosedCaptions == media_type {
            track_assert_ne!(uri, None, ErrorKind::InvalidInput);
            track_assert!(instream_id.is_some(), ErrorKind::InvalidInput);
        } else {
            track_assert!(instream_id.is_none(), ErrorKind::InvalidInput);
        }
        if MediaType::Subtitles != media_type {
            track_assert_eq!(forced, None, ErrorKind::InvalidInput);
        }
        Ok(ExtXMedia {
            media_type,
            uri,
            group_id,
            language,
            assoc_language,
            name,
            default: default.unwrap_or(YesOrNo::No),
            autoselect: autoselect.unwrap_or(YesOrNo::No),
            forced,
            instream_id,
            characteristics,
            channels,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HdcpLevel {
    Type0,
    None,
}
impl fmt::Display for HdcpLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HdcpLevel::Type0 => "TYPE-0".fmt(f),
            HdcpLevel::None => "NONE".fmt(f),
        }
    }
}
impl FromStr for HdcpLevel {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "TYPE-0" => Ok(HdcpLevel::Type0),
            "NONE" => Ok(HdcpLevel::None),
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown HDCP level: {:?}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClosedCaptions {
    GroupId(QuotedString),
    None,
}
impl fmt::Display for ClosedCaptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClosedCaptions::GroupId(ref x) => x.fmt(f),
            ClosedCaptions::None => "NONE".fmt(f),
        }
    }
}
impl FromStr for ClosedCaptions {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s == "NONE" {
            Ok(ClosedCaptions::None)
        } else {
            Ok(ClosedCaptions::GroupId(track!(s.parse())?))
        }
    }
}

// TODO:  The URI line is REQUIRED.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXStreamInf {
    bandwidth: DecimalInteger,
    average_bandwidth: Option<DecimalInteger>,
    codecs: Option<QuotedString>,
    resolution: Option<DecimalResolution>,

    // TODO: rounded to three decimal places
    frame_rate: Option<DecimalFloatingPoint>,
    hdcp_level: Option<HdcpLevel>,
    audio: Option<QuotedString>,
    video: Option<QuotedString>,
    subtitles: Option<QuotedString>,
    closed_captions: Option<ClosedCaptions>,
}
impl ExtXStreamInf {
    const PREFIX: &'static str = "#EXT-X-STREAM-INF:";
}
impl fmt::Display for ExtXStreamInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "BANDWIDTH={}", self.bandwidth)?;
        if let Some(ref x) = self.average_bandwidth {
            write!(f, ",AVERAGE-BANDWIDTH={}", x)?;
        }
        if let Some(ref x) = self.codecs {
            write!(f, ",CODECS={}", x)?;
        }
        if let Some(ref x) = self.resolution {
            write!(f, ",RESOLUTION={}", x)?;
        }
        if let Some(ref x) = self.frame_rate {
            write!(f, ",FRAME-RATE={}", x)?;
        }
        if let Some(ref x) = self.hdcp_level {
            write!(f, ",HDCP-LEVEL={}", x)?;
        }
        if let Some(ref x) = self.audio {
            write!(f, ",AUDIO={}", x)?;
        }
        if let Some(ref x) = self.video {
            write!(f, ",VIDEO={}", x)?;
        }
        if let Some(ref x) = self.subtitles {
            write!(f, ",SUBTITLES={}", x)?;
        }
        if let Some(ref x) = self.closed_captions {
            write!(f, ",CLOSED-CAPTIONS={}", x)?;
        }
        Ok(())
    }
}
impl FromStr for ExtXStreamInf {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut bandwidth = None;
        let mut average_bandwidth = None;
        let mut codecs = None;
        let mut resolution = None;
        let mut frame_rate = None;
        let mut hdcp_level = None;
        let mut audio = None;
        let mut video = None;
        let mut subtitles = None;
        let mut closed_captions = None;
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "BANDWIDTH" => bandwidth = Some(track!(value.parse())?),
                "AVERAGE-BANDWIDTH" => average_bandwidth = Some(track!(value.parse())?),
                "CODECS" => codecs = Some(track!(value.parse())?),
                "RESOLUTION" => resolution = Some(track!(value.parse())?),
                "FRAME-RATE" => frame_rate = Some(track!(value.parse())?),
                "HDCP-LEVEL" => hdcp_level = Some(track!(value.parse())?),
                "AUDIO" => {
                    // TODO: It MUST match the value of the GROUP-ID attribute of an EXT-X-MEDIA tag
                    audio = Some(track!(value.parse())?);
                }
                "VIDEO" => {
                    // TODO: It MUST match the value of the GROUP-ID attribute of an EXT-X-MEDIA tag
                    video = Some(track!(value.parse())?);
                }
                "SUBTITLES" => {
                    // TODO: It MUST match the value of the GROUP-ID attribute of an EXT-X-MEDIA tag
                    subtitles = Some(track!(value.parse())?);
                }
                "CLOSED-CAPTIONS" => {
                    // TODO: validate
                    closed_captions = Some(track!(value.parse())?);
                }
                _ => {
                    // [6.3.1] ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }
        let bandwidth = track_assert_some!(bandwidth, ErrorKind::InvalidInput);
        Ok(ExtXStreamInf {
            bandwidth,
            average_bandwidth,
            codecs,
            resolution,
            frame_rate,
            hdcp_level,
            audio,
            video,
            subtitles,
            closed_captions,
        })
    }
}

// TODO: That Playlist file MUST contain an EXT-X-I-FRAMES-ONLY tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXIFrameStreamInf {
    uri: QuotedString,
    bandwidth: DecimalInteger,
    average_bandwidth: Option<DecimalInteger>,
    codecs: Option<QuotedString>,
    resolution: Option<DecimalResolution>,
    hdcp_level: Option<HdcpLevel>,
    video: Option<QuotedString>,
}
impl ExtXIFrameStreamInf {
    const PREFIX: &'static str = "#EXT-X-I-FRAME-STREAM-INF:";
}
impl fmt::Display for ExtXIFrameStreamInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "URI={}", self.uri)?;
        write!(f, ",BANDWIDTH={}", self.bandwidth)?;
        if let Some(ref x) = self.average_bandwidth {
            write!(f, ",AVERAGE-BANDWIDTH={}", x)?;
        }
        if let Some(ref x) = self.codecs {
            write!(f, ",CODECS={}", x)?;
        }
        if let Some(ref x) = self.resolution {
            write!(f, ",RESOLUTION={}", x)?;
        }
        if let Some(ref x) = self.hdcp_level {
            write!(f, ",HDCP-LEVEL={}", x)?;
        }
        if let Some(ref x) = self.video {
            write!(f, ",VIDEO={}", x)?;
        }
        Ok(())
    }
}
impl FromStr for ExtXIFrameStreamInf {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut uri = None;
        let mut bandwidth = None;
        let mut average_bandwidth = None;
        let mut codecs = None;
        let mut resolution = None;
        let mut hdcp_level = None;
        let mut video = None;
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "URI" => uri = Some(track!(value.parse())?),
                "BANDWIDTH" => bandwidth = Some(track!(value.parse())?),
                "AVERAGE-BANDWIDTH" => average_bandwidth = Some(track!(value.parse())?),
                "CODECS" => codecs = Some(track!(value.parse())?),
                "RESOLUTION" => resolution = Some(track!(value.parse())?),
                "HDCP-LEVEL" => hdcp_level = Some(track!(value.parse())?),
                "VIDEO" => {
                    // TODO:
                    video = Some(track!(value.parse())?);
                }
                _ => {
                    // [6.3.1] ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let uri = track_assert_some!(uri, ErrorKind::InvalidInput);
        let bandwidth = track_assert_some!(bandwidth, ErrorKind::InvalidInput);
        Ok(ExtXIFrameStreamInf {
            uri,
            bandwidth,
            average_bandwidth,
            codecs,
            resolution,
            hdcp_level,
            video,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionData {
    Value(QuotedString),
    Uri(QuotedString),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXSessionData {
    pub data_id: QuotedString,
    pub data: SessionData,
    pub language: Option<QuotedString>,
}
impl ExtXSessionData {
    const PREFIX: &'static str = "#EXT-X-SESSION-DATA:";
}
impl fmt::Display for ExtXSessionData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "DATA-ID={}", self.data_id)?;
        match self.data {
            SessionData::Value(ref x) => write!(f, ",VALUE={}", x)?,
            SessionData::Uri(ref x) => write!(f, ",URI={}", x)?,
        }
        if let Some(ref x) = self.language {
            write!(f, ",LANGUAGE={}", x)?;
        }
        Ok(())
    }
}
impl FromStr for ExtXSessionData {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut data_id = None;
        let mut session_value = None;
        let mut uri = None;
        let mut language = None;
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "DATA-ID" => data_id = Some(track!(value.parse())?),
                "VALUE" => session_value = Some(track!(value.parse())?),
                "URI" => uri = Some(track!(value.parse())?),
                "LANGUAGE" => language = Some(track!(value.parse())?),
                _ => {
                    // [6.3.1] ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let data_id = track_assert_some!(data_id, ErrorKind::InvalidInput);
        let data = if let Some(value) = session_value {
            track_assert_eq!(uri, None, ErrorKind::InvalidInput);
            SessionData::Value(value)
        } else if let Some(uri) = uri {
            SessionData::Uri(uri)
        } else {
            track_panic!(ErrorKind::InvalidInput);
        };
        Ok(ExtXSessionData {
            data_id,
            data,
            language,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXSessionKey {
    pub method: SessionEncryptionMethod,
    pub uri: QuotedString,
    pub iv: Option<HexadecimalSequence>,
    pub key_format: Option<QuotedString>,
    pub key_format_versions: Option<QuotedString>,
}
impl ExtXSessionKey {
    const PREFIX: &'static str = "#EXT-X-SESSION-KEY:";
}
impl fmt::Display for ExtXSessionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "METHOD={}", self.method)?;
        write!(f, ",URI={}", self.uri)?;
        if let Some(ref x) = self.iv {
            write!(f, ",IV={}", x)?;
        }
        if let Some(ref x) = self.key_format {
            write!(f, ",KEYFORMAT={}", x)?;
        }
        if let Some(ref x) = self.key_format_versions {
            write!(f, ",KEYFORMATVERSIONS={}", x)?;
        }
        Ok(())
    }
}
impl FromStr for ExtXSessionKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut method = None;
        let mut uri = None;
        let mut iv = None;
        let mut key_format = None;
        let mut key_format_versions = None;
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "METHOD" => {
                    track_assert_eq!(method, None, ErrorKind::InvalidInput);
                    method = Some(track!(value.parse())?);
                }
                "URI" => {
                    track_assert_eq!(uri, None, ErrorKind::InvalidInput);
                    uri = Some(track!(value.parse())?);
                }
                "IV" => {
                    // TODO: validate length(128-bit)
                    track_assert_eq!(iv, None, ErrorKind::InvalidInput);
                    iv = Some(track!(value.parse())?);
                }
                "KEYFORMAT" => {
                    track_assert_eq!(key_format, None, ErrorKind::InvalidInput);
                    key_format = Some(track!(value.parse())?);
                }
                "KEYFORMATVERSIONS" => {
                    track_assert_eq!(key_format_versions, None, ErrorKind::InvalidInput);
                    key_format_versions = Some(track!(value.parse())?);
                }
                _ => {
                    // [6.3.1] ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }
        let method = track_assert_some!(method, ErrorKind::InvalidInput);
        let uri = track_assert_some!(uri, ErrorKind::InvalidInput);
        Ok(ExtXSessionKey {
            method,
            uri,
            iv,
            key_format,
            key_format_versions,
        })
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtXStart {
    pub time_offset: SignedDecimalFloatingPoint,
    pub precise: YesOrNo,
}
impl ExtXStart {
    const PREFIX: &'static str = "#EXT-X-START:";
}
impl fmt::Display for ExtXStart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "TIME-OFFSET={}", self.time_offset)?;
        if self.precise == YesOrNo::Yes {
            write!(f, ",PRECISE={}", self.precise)?;
        }
        Ok(())
    }
}
impl FromStr for ExtXStart {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);

        let mut time_offset = None;
        let mut precise = None;
        let attrs = AttributePairs::parse(s.split_at(Self::PREFIX.len()).1);
        for attr in attrs {
            let (key, value) = track!(attr)?;
            match key {
                "TIME-OFFSET" => time_offset = Some(track!(value.parse())?),
                "PRECISE" => precise = Some(track!(value.parse())?),
                _ => {
                    // [6.3.1] ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let time_offset = track_assert_some!(time_offset, ErrorKind::InvalidInput);
        Ok(ExtXStart {
            time_offset,
            precise: precise.unwrap_or(YesOrNo::No),
        })
    }
}

// TODO: move
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionEncryptionMethod {
    Aes128,
    SampleAes,
}
impl fmt::Display for SessionEncryptionMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SessionEncryptionMethod::Aes128 => "AES-128".fmt(f),
            SessionEncryptionMethod::SampleAes => "SAMPLE-AES".fmt(f),
        }
    }
}
impl FromStr for SessionEncryptionMethod {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "AES-128" => Ok(SessionEncryptionMethod::Aes128),
            "SAMPLE-AES" => Ok(SessionEncryptionMethod::SampleAes),
            _ => track_panic!(
                ErrorKind::InvalidInput,
                "Unknown encryption method: {:?}",
                s
            ),
        }
    }
}
