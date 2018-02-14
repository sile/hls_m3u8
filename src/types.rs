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
