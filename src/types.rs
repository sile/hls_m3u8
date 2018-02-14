//! Miscellaneous types.
use std::fmt;
use std::ops::Deref;
use std::str::{self, FromStr};
use std::time::Duration;
use trackable::error::ErrorKindExt;

use {Error, ErrorKind, Result};
use attribute::AttributePairs;

/// String that represents a single line in a playlist file.
///
/// See: [4.1. Definition of a Playlist]
///
/// [4.1. Definition of a Playlist]: https://tools.ietf.org/html/rfc8216#section-4.1
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SingleLineString(String);
impl SingleLineString {
    /// Makes a new `SingleLineString` instance.
    ///
    /// # Errors
    ///
    /// If the given string contains any control characters,
    /// this function will return an error which has the kind `ErrorKind::InvalidInput`.
    pub fn new<T: Into<String>>(s: T) -> Result<Self> {
        let s = s.into();
        track_assert!(!s.chars().any(|c| c.is_control()), ErrorKind::InvalidInput);
        Ok(SingleLineString(s))
    }
}
impl Deref for SingleLineString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<str> for SingleLineString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for SingleLineString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Quoted string.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuotedString(String);
impl QuotedString {
    /// Makes a new `QuotedString` instance.
    ///
    /// # Errors
    ///
    /// If the given string contains any control characters or double-quote character,
    /// this function will return an error which has the kind `ErrorKind::InvalidInput`.
    pub fn new<T: Into<String>>(s: T) -> Result<Self> {
        let s = s.into();
        track_assert!(
            !s.chars().any(|c| c.is_control() || c == '"'),
            ErrorKind::InvalidInput
        );
        Ok(QuotedString(s))
    }
}
impl Deref for QuotedString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<str> for QuotedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for QuotedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl FromStr for QuotedString {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let len = s.len();
        let bytes = s.as_bytes();
        track_assert!(len >= 2, ErrorKind::InvalidInput);
        track_assert_eq!(bytes[0], b'"', ErrorKind::InvalidInput);
        track_assert_eq!(bytes[len - 1], b'"', ErrorKind::InvalidInput);

        let s = unsafe { str::from_utf8_unchecked(&bytes[1..len - 1]) };
        track!(QuotedString::new(s))
    }
}

/// Decimal resolution.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimalResolution {
    /// Horizontal pixel dimension.
    pub width: usize,

    /// Vertical pixel dimension.
    pub height: usize,
}
impl fmt::Display for DecimalResolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}
impl FromStr for DecimalResolution {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut tokens = s.splitn(2, 'x');
        let width = tokens.next().expect("Never fails");
        let height = track_assert_some!(tokens.next(), ErrorKind::InvalidInput);
        Ok(DecimalResolution {
            width: track!(width.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?,
            height: track!(height.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?,
        })
    }
}

/// Non-negative decimal floating-point number.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DecimalFloatingPoint(f64);
impl DecimalFloatingPoint {
    /// Makes a new `DecimalFloatingPoint` instance.
    ///
    /// # Errors
    ///
    /// The given value must have a positive sign and be finite,
    /// otherwise this function will return an error that has the kind `ErrorKind::InvalidInput`.
    pub fn new(n: f64) -> Result<Self> {
        track_assert!(n.is_sign_positive(), ErrorKind::InvalidInput);
        track_assert!(n.is_finite(), ErrorKind::InvalidInput);
        Ok(DecimalFloatingPoint(n))
    }

    /// Converts `DecimalFloatingPoint` to `f64`.
    pub fn as_f64(&self) -> f64 {
        self.0
    }

    pub(crate) fn to_duration(&self) -> Duration {
        let secs = self.0 as u64;
        let nanos = (self.0.fract() * 1_000_000_000.0) as u32;
        Duration::new(secs, nanos)
    }

    pub(crate) fn from_duration(duration: Duration) -> Self {
        let n =
            (duration.as_secs() as f64) + (f64::from(duration.subsec_nanos()) / 1_000_000_000.0);
        DecimalFloatingPoint(n)
    }
}
impl From<u32> for DecimalFloatingPoint {
    fn from(f: u32) -> Self {
        DecimalFloatingPoint(f64::from(f))
    }
}
impl Eq for DecimalFloatingPoint {}
impl fmt::Display for DecimalFloatingPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl FromStr for DecimalFloatingPoint {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(
            s.chars().all(|c| c.is_digit(10) || c == '.'),
            ErrorKind::InvalidInput
        );
        let n = track!(s.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
        Ok(DecimalFloatingPoint(n))
    }
}

/// Signed decimal floating-point number.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SignedDecimalFloatingPoint(f64);
impl SignedDecimalFloatingPoint {
    /// Makes a new `SignedDecimalFloatingPoint` instance.
    ///
    /// # Errors
    ///
    /// The given value must be finite,
    /// otherwise this function will return an error that has the kind `ErrorKind::InvalidInput`.
    pub fn new(n: f64) -> Result<Self> {
        track_assert!(n.is_finite(), ErrorKind::InvalidInput);
        Ok(SignedDecimalFloatingPoint(n))
    }

    /// Converts `DecimalFloatingPoint` to `f64`.
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}
impl From<i32> for SignedDecimalFloatingPoint {
    fn from(f: i32) -> Self {
        SignedDecimalFloatingPoint(f64::from(f))
    }
}
impl Eq for SignedDecimalFloatingPoint {}
impl fmt::Display for SignedDecimalFloatingPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl FromStr for SignedDecimalFloatingPoint {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(
            s.chars().all(|c| c.is_digit(10) || c == '.' || c == '-'),
            ErrorKind::InvalidInput
        );
        let n = track!(s.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
        Ok(SignedDecimalFloatingPoint(n))
    }
}

/// Hexadecimal sequence.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HexadecimalSequence(Vec<u8>);
impl HexadecimalSequence {
    /// Makes a new `HexadecimalSequence` instance.
    pub fn new<T: Into<Vec<u8>>>(v: T) -> Self {
        HexadecimalSequence(v.into())
    }

    /// Converts into the underlying byte sequence.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}
impl Deref for HexadecimalSequence {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<[u8]> for HexadecimalSequence {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl fmt::Display for HexadecimalSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x")?;
        for b in &self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}
impl FromStr for HexadecimalSequence {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(
            s.starts_with("0x") || s.starts_with("0X"),
            ErrorKind::InvalidInput
        );
        track_assert!(s.len() % 2 == 0, ErrorKind::InvalidInput);

        let mut v = Vec::with_capacity(s.len() / 2 - 1);
        for c in s.as_bytes().chunks(2).skip(1) {
            let d = track!(str::from_utf8(c).map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
            let b =
                track!(u8::from_str_radix(d, 16).map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
            v.push(b);
        }
        Ok(HexadecimalSequence(v))
    }
}

/// Initialization vector.
///
/// See: [4.3.2.4. EXT-X-KEY]
///
/// [4.3.2.4. EXT-X-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.4
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InitializationVector(pub [u8; 16]);
impl Deref for InitializationVector {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<[u8]> for InitializationVector {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl fmt::Display for InitializationVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x")?;
        for b in &self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}
impl FromStr for InitializationVector {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(
            s.starts_with("0x") || s.starts_with("0X"),
            ErrorKind::InvalidInput
        );
        track_assert_eq!(s.len() - 2, 32, ErrorKind::InvalidInput);

        let mut v = [0; 16];
        for (i, c) in s.as_bytes().chunks(2).skip(1).enumerate() {
            let d = track!(str::from_utf8(c).map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
            let b =
                track!(u8::from_str_radix(d, 16).map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
            v[i] = b;
        }
        Ok(InitializationVector(v))
    }
}

/// [7. Protocol Version Compatibility]
///
/// [7. Protocol Version Compatibility]: https://tools.ietf.org/html/rfc8216#section-7
#[allow(missing_docs)]
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

/// Byte range.
///
/// See: [4.3.2.2. EXT-X-BYTERANGE]
///
/// [4.3.2.2. EXT-X-BYTERANGE]: https://tools.ietf.org/html/rfc8216#section-4.3.2.2
#[allow(missing_docs)]
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

/// Decryption key.
///
/// See: [4.3.2.4. EXT-X-KEY]
///
/// [4.3.2.4. EXT-X-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.4
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecryptionKey {
    pub method: EncryptionMethod,
    pub uri: QuotedString,
    pub iv: Option<InitializationVector>,
    pub key_format: Option<QuotedString>,
    pub key_format_versions: Option<QuotedString>,
}
impl DecryptionKey {
    pub(crate) fn requires_version(&self) -> ProtocolVersion {
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
                "METHOD" => method = Some(track!(value.parse())?),
                "URI" => uri = Some(track!(value.parse())?),
                "IV" => iv = Some(track!(value.parse())?),
                "KEYFORMAT" => key_format = Some(track!(value.parse())?),
                "KEYFORMATVERSIONS" => key_format_versions = Some(track!(value.parse())?),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
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

/// Encryption method.
///
/// See: [4.3.2.4. EXT-X-KEY]
///
/// [4.3.2.4. EXT-X-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.4
#[allow(missing_docs)]
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

/// Playlist type.
///
/// See: [4.3.3.5. EXT-X-PLAYLIST-TYPE]
///
/// [4.3.3.5. EXT-X-PLAYLIST-TYPE]: https://tools.ietf.org/html/rfc8216#section-4.3.3.5
#[allow(missing_docs)]
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

/// Media type.
///
/// See: [4.3.4.1. EXT-X-MEDIA]
///
/// [4.3.4.1. EXT-X-MEDIA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.1
#[allow(missing_docs)]
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

/// Identifier of a rendition within the segments in a media playlist.
///
/// See: [4.3.4.1. EXT-X-MEDIA]
///
/// [4.3.4.1. EXT-X-MEDIA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.1
#[allow(missing_docs)]
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

/// HDCP level.
///
/// See: [4.3.4.2. EXT-X-STREAM-INF]
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[allow(missing_docs)]
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

/// The identifier of a closed captions group or its absence.
///
/// See: [4.3.4.2. EXT-X-STREAM-INF]
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[allow(missing_docs)]
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

/// Session data.
///
/// See: [4.3.4.4. EXT-X-SESSION-DATA]
///
/// [4.3.4.4. EXT-X-SESSION-DATA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.4
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SessionData {
    Value(QuotedString),
    Uri(QuotedString),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_line_string() {
        assert!(SingleLineString::new("foo").is_ok());
        assert!(SingleLineString::new("b\rar").is_err());
    }
}
