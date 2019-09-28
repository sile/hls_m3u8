use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use crate::Error;

/// Initialization vector.
///
/// See: [4.3.2.4. EXT-X-KEY]
///
/// [4.3.2.4. EXT-X-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.4
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InitializationVector(pub [u8; 16]);

impl InitializationVector {
    /// Converts the [InitializationVector] to a slice.
    pub const fn to_slice(&self) -> [u8; 16] {
        self.0
    }
}

impl From<[u8; 16]> for InitializationVector {
    fn from(value: [u8; 16]) -> Self {
        Self(value)
    }
}

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

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if !(input.starts_with("0x") || input.starts_with("0X")) {
            return Err(Error::invalid_input());
        }
        if input.len() - 2 != 32 {
            return Err(Error::invalid_input());
        }

        let mut result = [0; 16];
        for (i, c) in input.as_bytes().chunks(2).skip(1).enumerate() {
            let d = std::str::from_utf8(c).map_err(Error::custom)?;
            let b = u8::from_str_radix(d, 16).map_err(Error::custom)?;
            result[i] = b;
        }

        Ok(Self(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            "0x10ef8f758ca555115584bb5b3c687f52".to_string(),
            InitializationVector([
                16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82
            ])
            .to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            "0x10ef8f758ca555115584bb5b3c687f52"
                .parse::<InitializationVector>()
                .unwrap(),
            InitializationVector([
                16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82
            ])
        );

        assert_eq!(
            "0X10ef8f758ca555115584bb5b3c687f52"
                .parse::<InitializationVector>()
                .unwrap(),
            InitializationVector([
                16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82
            ])
        );

        assert_eq!(
            "0X10EF8F758CA555115584BB5B3C687F52"
                .parse::<InitializationVector>()
                .unwrap(),
            InitializationVector([
                16, 239, 143, 117, 140, 165, 85, 17, 85, 132, 187, 91, 60, 104, 127, 82
            ])
        );

        assert!("garbage".parse::<InitializationVector>().is_err());
        assert!("0xgarbage".parse::<InitializationVector>().is_err());
        assert!("0x12".parse::<InitializationVector>().is_err());
        assert!("0X10EF8F758CA555115584BB5B3C687F5Z"
            .parse::<InitializationVector>()
            .is_err());
    }

    #[test]
    fn test_as_ref() {
        assert_eq!(
            InitializationVector([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).as_ref(),
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_deref() {
        assert_eq!(
            InitializationVector([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).deref(),
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_from() {
        assert_eq!(
            InitializationVector::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            InitializationVector([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
        );
    }

    #[test]
    fn test_to_slice() {
        assert_eq!(
            InitializationVector([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).to_slice(),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
