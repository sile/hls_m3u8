use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::ops::Deref;
use std::str::{self, FromStr};
use trackable::error::ErrorKindExt;

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
