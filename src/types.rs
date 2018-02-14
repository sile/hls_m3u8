//! Miscellaneous types.
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use trackable::error::ErrorKindExt;

use {Error, ErrorKind, Result};

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
