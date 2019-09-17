use std::fmt;
use std::str::FromStr;

use crate::Error;

/// Encryption method.
///
/// See: [4.3.2.4. EXT-X-KEY]
///
/// [4.3.2.4. EXT-X-KEY]: https://tools.ietf.org/html/rfc8216#section-4.3.2.4
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncryptionMethod {
    /// `None` means that [MediaSegment]s are not encrypted.
    ///
    /// [MediaSegment]: crate::MediaSegment
    None,
    /// `Aes128` signals that the [MediaSegment]s are completely encrypted
    /// using the Advanced Encryption Standard ([AES_128]) with a 128-bit
    /// key, Cipher Block Chaining (CBC), and
    /// [Public-Key Cryptography Standards #7 (PKCS7)] padding.
    /// CBC is restarted on each segment boundary, using either the
    /// Initialization Vector (IV) attribute value or the Media Sequence
    /// Number as the IV.
    ///
    /// [MediaSegment]: crate::MediaSegment
    /// [AES_128]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf
    /// [Public-Key Cryptography Standards #7 (PKCS7)]: https://tools.ietf.org/html/rfc5652
    Aes128,
    /// `SampleAes` means that the [MediaSegment]s
    /// contain media samples, such as audio or video, that are encrypted
    /// using the Advanced Encryption Standard ([AES_128]). How these media
    /// streams are encrypted and encapsulated in a segment depends on the
    /// media encoding and the media format of the segment. fMP4 Media
    /// Segments are encrypted using the 'cbcs' scheme of
    /// [Common Encryption].  Encryption of other Media Segment
    /// formats containing [H.264], [AAC], [AC-3],
    /// and Enhanced [AC-3] media streams is described in the HTTP
    /// Live Streaming (HLS) [SampleEncryption specification].
    ///
    /// [MediaSegment]: crate::MediaSegment
    /// [AES_128]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf
    /// [Common Encryption]: https://tools.ietf.org/html/rfc8216#ref-COMMON_ENC
    /// [H.264]: https://tools.ietf.org/html/rfc8216#ref-H_264
    /// [AAC]: https://tools.ietf.org/html/rfc8216#ref-ISO_14496
    /// [AC-3]: https://tools.ietf.org/html/rfc8216#ref-AC_3
    /// [SampleEncryption specification]: https://tools.ietf.org/html/rfc8216#ref-SampleEnc
    SampleAes,
}

impl fmt::Display for EncryptionMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            EncryptionMethod::Aes128 => "AES-128".fmt(f),
            EncryptionMethod::SampleAes => "SAMPLE-AES".fmt(f),
            EncryptionMethod::None => "NONE".fmt(f),
        }
    }
}

impl FromStr for EncryptionMethod {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "AES-128" => Ok(EncryptionMethod::Aes128),
            "SAMPLE-AES" => Ok(EncryptionMethod::SampleAes),
            "NONE" => Ok(EncryptionMethod::None),
            _ => Err(Error::custom(format!(
                "Unknown encryption method: {:?}",
                input
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(EncryptionMethod::Aes128.to_string(), "AES-128".to_string());
        assert_eq!(
            EncryptionMethod::SampleAes.to_string(),
            "SAMPLE-AES".to_string()
        );
        assert_eq!(EncryptionMethod::None.to_string(), "NONE".to_string());
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            EncryptionMethod::Aes128,
            "AES-128".parse::<EncryptionMethod>().unwrap()
        );

        assert_eq!(
            EncryptionMethod::SampleAes,
            "SAMPLE-AES".parse::<EncryptionMethod>().unwrap()
        );

        assert_eq!(
            EncryptionMethod::None,
            "NONE".parse::<EncryptionMethod>().unwrap()
        );
    }
}
