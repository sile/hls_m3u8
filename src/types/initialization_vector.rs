use core::fmt;
use core::str::FromStr;

use crate::Error;

/// An initialization vector (IV) is a fixed size input that can be used along
/// with a secret key for data encryption.
///
/// The use of an IV prevents repetition in encrypted data, making it more
/// difficult for a hacker using a dictionary attack to find patterns and break
/// a cipher. For example, a sequence might appear twice or more within the body
/// of a message. If there are repeated sequences in encrypted data, an attacker
/// could assume that the corresponding sequences in the message were also
/// identical. The IV prevents the appearance of corresponding duplicate
/// character sequences in the ciphertext.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum InitializationVector {
    /// An IV for use with Aes128.
    Aes128([u8; 0x10]),
    /// An [`ExtXKey`] tag with [`KeyFormat::Identity`] that does not have an IV
    /// field indicates that the [`MediaSegment::number`] is to be used as the
    /// IV when decrypting a `MediaSegment`.
    ///
    /// [`ExtXKey`]: crate::tags::ExtXKey
    /// [`KeyFormat::Identity`]: crate::types::KeyFormat::Identity
    /// [`MediaSegment::number`]: crate::MediaSegment::number
    Number(u128),
    /// Signals that an IV is missing.
    Missing,
}

impl InitializationVector {
    /// Returns the IV as an [`u128`]. `None` is returned for
    /// [`InitializationVector::Missing`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::InitializationVector;
    /// assert_eq!(
    ///     InitializationVector::Aes128([
    ///         0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78,
    ///         0x90, 0x12
    ///     ])
    ///     .to_u128(),
    ///     Some(0x12345678901234567890123456789012)
    /// );
    ///
    /// assert_eq!(InitializationVector::Number(0x10).to_u128(), Some(0x10));
    ///
    /// assert_eq!(InitializationVector::Missing.to_u128(), None);
    /// ```
    #[must_use]
    pub fn to_u128(&self) -> Option<u128> {
        match *self {
            Self::Aes128(v) => Some(u128::from_be_bytes(v)),
            Self::Number(n) => Some(n),
            Self::Missing => None,
        }
    }

    /// Returns the IV as a slice, which can be used to for example decrypt
    /// a [`MediaSegment`]. `None` is returned for
    /// [`InitializationVector::Missing`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::InitializationVector;
    /// assert_eq!(
    ///     InitializationVector::Aes128([
    ///         0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
    ///         0x0F, 0x10,
    ///     ])
    ///     .to_slice(),
    ///     Some([
    ///         0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
    ///         0x0F, 0x10,
    ///     ])
    /// );
    ///
    /// assert_eq!(
    ///     InitializationVector::Number(0x12345678901234567890123456789012).to_slice(),
    ///     Some([
    ///         0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78,
    ///         0x90, 0x12
    ///     ])
    /// );
    ///
    /// assert_eq!(InitializationVector::Missing.to_slice(), None);
    /// ```
    ///
    /// [`MediaSegment`]: crate::MediaSegment
    #[must_use]
    pub fn to_slice(&self) -> Option<[u8; 0x10]> {
        match &self {
            Self::Aes128(v) => Some(*v),
            Self::Number(v) => Some(v.to_be_bytes()),
            Self::Missing => None,
        }
    }

    /// Returns `true` if the initialization vector is not missing.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::InitializationVector;
    /// assert_eq!(
    ///     InitializationVector::Aes128([
    ///         0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
    ///         0x0F, 0x10,
    ///     ])
    ///     .is_some(),
    ///     true
    /// );
    ///
    /// assert_eq!(InitializationVector::Number(4).is_some(), true);
    ///
    /// assert_eq!(InitializationVector::Missing.is_some(), false);
    /// ```
    #[must_use]
    #[inline]
    pub fn is_some(&self) -> bool { *self != Self::Missing }

    /// Returns `true` if the initialization vector is missing.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::InitializationVector;
    /// assert_eq!(
    ///     InitializationVector::Aes128([
    ///         0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
    ///         0x0F, 0x10,
    ///     ])
    ///     .is_none(),
    ///     false
    /// );
    ///
    /// assert_eq!(InitializationVector::Number(4).is_none(), false);
    ///
    /// assert_eq!(InitializationVector::Missing.is_none(), true);
    /// ```
    #[must_use]
    #[inline]
    pub fn is_none(&self) -> bool { *self == Self::Missing }
}

impl Default for InitializationVector {
    fn default() -> Self { Self::Missing }
}

impl From<[u8; 0x10]> for InitializationVector {
    fn from(value: [u8; 0x10]) -> Self { Self::Aes128(value) }
}

impl From<Option<[u8; 0x10]>> for InitializationVector {
    fn from(value: Option<[u8; 0x10]>) -> Self {
        match value {
            Some(v) => Self::Aes128(v),
            None => Self::Missing,
        }
    }
}

impl fmt::Display for InitializationVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Aes128(buffer) => {
                let mut result = [0; 0x10 * 2];
                ::hex::encode_to_slice(buffer, &mut result).unwrap();

                write!(f, "0x{}", ::core::str::from_utf8(&result).unwrap())?;
            }
            Self::Number(num) => {
                write!(f, "InitializationVector::Number({})", num)?;
            }
            Self::Missing => {
                write!(f, "InitializationVector::Missing")?;
            }
        }

        Ok(())
    }
}

impl FromStr for InitializationVector {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if !(input.starts_with("0x") || input.starts_with("0X")) {
            return Err(Error::custom("An IV should either start with `0x` or `0X`"));
        }

        if input.len() - 2 != 32 {
            return Err(Error::custom(
                "An IV must be 32 bytes long + 2 bytes for 0x/0X",
            ));
        }

        let mut result = [0; 16];

        ::hex::decode_to_slice(&input.as_bytes()[2..], &mut result).map_err(Error::hex)?;

        Ok(Self::Aes128(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_default() {
        assert_eq!(
            InitializationVector::default(),
            InitializationVector::Missing
        );
    }

    #[test]
    fn test_from() {
        assert_eq!(
            InitializationVector::from([
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF
            ]),
            InitializationVector::Aes128([
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF
            ])
        );

        assert_eq!(
            InitializationVector::from(None),
            InitializationVector::Missing
        );

        assert_eq!(
            InitializationVector::from(Some([
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF
            ])),
            InitializationVector::Aes128([
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF
            ])
        )
    }

    #[test]
    fn test_display() {
        assert_eq!(
            InitializationVector::Aes128([
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF
            ])
            .to_string(),
            "0xffffffffffffffffffffffffffffffff".to_string()
        );

        assert_eq!(
            InitializationVector::Number(5).to_string(),
            "InitializationVector::Number(5)".to_string()
        );

        assert_eq!(
            InitializationVector::Missing.to_string(),
            "InitializationVector::Missing".to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            InitializationVector::Aes128([
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF
            ]),
            "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF".parse().unwrap()
        );

        assert_eq!(
            InitializationVector::Aes128([
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF
            ]),
            "0XFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF".parse().unwrap()
        );

        // missing `0x` at the start:
        assert!("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
            .parse::<InitializationVector>()
            .is_err());
        // too small:
        assert!("0xFF".parse::<InitializationVector>().is_err());
        // too large:
        assert!("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
            .parse::<InitializationVector>()
            .is_err());
    }
}
