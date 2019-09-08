use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::ops::Deref;
use std::str::{self, FromStr};
use trackable::error::ErrorKindExt;

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
