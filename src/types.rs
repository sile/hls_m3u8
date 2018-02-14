//! Miscellaneous types.
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use trackable::error::ErrorKindExt;

use {Error, ErrorKind, Result};
use attribute::{AttributePairs, HexadecimalSequence, QuotedString};

// TODO: rename
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct M3u8String(String);
impl M3u8String {
    pub fn new<T: Into<String>>(s: T) -> Result<Self> {
        // TODO: validate
        Ok(M3u8String(s.into()))
    }
    pub unsafe fn new_unchecked<T: Into<String>>(s: T) -> Self {
        M3u8String(s.into())
    }
}
impl Deref for M3u8String {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<str> for M3u8String {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for M3u8String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Yes;
impl fmt::Display for Yes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "YES".fmt(f)
    }
}
impl FromStr for Yes {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert_eq!(s, "YES", ErrorKind::InvalidInput);
        Ok(Yes)
    }
}

// https://tools.ietf.org/html/rfc8216#section-7
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProtocolVersion {
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
}
impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = match *self {
            ProtocolVersion::V1 => 1,
            ProtocolVersion::V2 => 2,
            ProtocolVersion::V3 => 3,
            ProtocolVersion::V4 => 4,
            ProtocolVersion::V5 => 5,
            ProtocolVersion::V6 => 6,
            ProtocolVersion::V7 => 7,
        };
        write!(f, "{}", n)
    }
}
impl FromStr for ProtocolVersion {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "1" => ProtocolVersion::V1,
            "2" => ProtocolVersion::V2,
            "3" => ProtocolVersion::V3,
            "4" => ProtocolVersion::V4,
            "5" => ProtocolVersion::V5,
            "6" => ProtocolVersion::V6,
            "7" => ProtocolVersion::V7,
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown protocol version: {:?}", s),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ByteRange {
    pub length: usize,
    pub start: Option<usize>,
}
impl fmt::Display for ByteRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.length)?;
        if let Some(x) = self.start {
            write!(f, "@{}", x)?;
        }
        Ok(())
    }
}
impl FromStr for ByteRange {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut tokens = s.splitn(2, '@');
        let length = tokens.next().expect("Never fails");
        let start = if let Some(start) = tokens.next() {
            Some(track!(
                start.parse().map_err(|e| ErrorKind::InvalidInput.cause(e))
            )?)
        } else {
            None
        };
        Ok(ByteRange {
            length: track!(length.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?,
            start,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecryptionKey {
    pub method: EncryptionMethod,
    pub uri: QuotedString,
    pub iv: Option<HexadecimalSequence>,
    pub key_format: Option<QuotedString>,
    pub key_format_versions: Option<QuotedString>,
}
impl DecryptionKey {
    pub fn requires_version(&self) -> ProtocolVersion {
        if self.key_format.is_some() | self.key_format_versions.is_some() {
            ProtocolVersion::V5
        } else if self.iv.is_some() {
            ProtocolVersion::V2
        } else {
            ProtocolVersion::V1
        }
    }
}
impl fmt::Display for DecryptionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
impl FromStr for DecryptionKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut method = None;
        let mut uri = None;
        let mut iv = None;
        let mut key_format = None;
        let mut key_format_versions = None;
        let attrs = AttributePairs::parse(s);
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
        Ok(DecryptionKey {
            method,
            uri,
            iv,
            key_format,
            key_format_versions,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncryptionMethod {
    Aes128,
    SampleAes,
}
impl fmt::Display for EncryptionMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncryptionMethod::Aes128 => "AES-128".fmt(f),
            EncryptionMethod::SampleAes => "SAMPLE-AES".fmt(f),
        }
    }
}
impl FromStr for EncryptionMethod {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

// TODO: remove
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InStreamId {
    Cc1,
    Cc2,
    Cc3,
    Cc4,
    Service1,
    Service2,
    Service3,
    Service4,
    Service5,
    Service6,
    Service7,
    Service8,
    Service9,
    Service10,
    Service11,
    Service12,
    Service13,
    Service14,
    Service15,
    Service16,
    Service17,
    Service18,
    Service19,
    Service20,
    Service21,
    Service22,
    Service23,
    Service24,
    Service25,
    Service26,
    Service27,
    Service28,
    Service29,
    Service30,
    Service31,
    Service32,
    Service33,
    Service34,
    Service35,
    Service36,
    Service37,
    Service38,
    Service39,
    Service40,
    Service41,
    Service42,
    Service43,
    Service44,
    Service45,
    Service46,
    Service47,
    Service48,
    Service49,
    Service50,
    Service51,
    Service52,
    Service53,
    Service54,
    Service55,
    Service56,
    Service57,
    Service58,
    Service59,
    Service60,
    Service61,
    Service62,
    Service63,
}
impl fmt::Display for InStreamId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format!("{:?}", self).to_uppercase().fmt(f)
    }
}
impl FromStr for InStreamId {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "CC1" => InStreamId::Cc1,
            "CC2" => InStreamId::Cc2,
            "CC3" => InStreamId::Cc3,
            "CC4" => InStreamId::Cc4,
            "SERVICE1" => InStreamId::Service1,
            "SERVICE2" => InStreamId::Service2,
            "SERVICE3" => InStreamId::Service3,
            "SERVICE4" => InStreamId::Service4,
            "SERVICE5" => InStreamId::Service5,
            "SERVICE6" => InStreamId::Service6,
            "SERVICE7" => InStreamId::Service7,
            "SERVICE8" => InStreamId::Service8,
            "SERVICE9" => InStreamId::Service9,
            "SERVICE10" => InStreamId::Service10,
            "SERVICE11" => InStreamId::Service11,
            "SERVICE12" => InStreamId::Service12,
            "SERVICE13" => InStreamId::Service13,
            "SERVICE14" => InStreamId::Service14,
            "SERVICE15" => InStreamId::Service15,
            "SERVICE16" => InStreamId::Service16,
            "SERVICE17" => InStreamId::Service17,
            "SERVICE18" => InStreamId::Service18,
            "SERVICE19" => InStreamId::Service19,
            "SERVICE20" => InStreamId::Service20,
            "SERVICE21" => InStreamId::Service21,
            "SERVICE22" => InStreamId::Service22,
            "SERVICE23" => InStreamId::Service23,
            "SERVICE24" => InStreamId::Service24,
            "SERVICE25" => InStreamId::Service25,
            "SERVICE26" => InStreamId::Service26,
            "SERVICE27" => InStreamId::Service27,
            "SERVICE28" => InStreamId::Service28,
            "SERVICE29" => InStreamId::Service29,
            "SERVICE30" => InStreamId::Service30,
            "SERVICE31" => InStreamId::Service31,
            "SERVICE32" => InStreamId::Service32,
            "SERVICE33" => InStreamId::Service33,
            "SERVICE34" => InStreamId::Service34,
            "SERVICE35" => InStreamId::Service35,
            "SERVICE36" => InStreamId::Service36,
            "SERVICE37" => InStreamId::Service37,
            "SERVICE38" => InStreamId::Service38,
            "SERVICE39" => InStreamId::Service39,
            "SERVICE40" => InStreamId::Service40,
            "SERVICE41" => InStreamId::Service41,
            "SERVICE42" => InStreamId::Service42,
            "SERVICE43" => InStreamId::Service43,
            "SERVICE44" => InStreamId::Service44,
            "SERVICE45" => InStreamId::Service45,
            "SERVICE46" => InStreamId::Service46,
            "SERVICE47" => InStreamId::Service47,
            "SERVICE48" => InStreamId::Service48,
            "SERVICE49" => InStreamId::Service49,
            "SERVICE50" => InStreamId::Service50,
            "SERVICE51" => InStreamId::Service51,
            "SERVICE52" => InStreamId::Service52,
            "SERVICE53" => InStreamId::Service53,
            "SERVICE54" => InStreamId::Service54,
            "SERVICE55" => InStreamId::Service55,
            "SERVICE56" => InStreamId::Service56,
            "SERVICE57" => InStreamId::Service57,
            "SERVICE58" => InStreamId::Service58,
            "SERVICE59" => InStreamId::Service59,
            "SERVICE60" => InStreamId::Service60,
            "SERVICE61" => InStreamId::Service61,
            "SERVICE62" => InStreamId::Service62,
            "SERVICE63" => InStreamId::Service63,
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown instream id: {:?}", s),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SessionData {
    Value(QuotedString),
    Uri(QuotedString),
}
