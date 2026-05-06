use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DecodeError {
    InvalidChar { byte: u8, index: usize },
    OddLength,
    LengthMismatch { expected: usize, got: usize },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::InvalidChar { byte, index } => {
                write!(f, "invalid hex character {byte:#04x} at index {index}")
            }
            Self::OddLength => write!(f, "odd hex string length"),
            Self::LengthMismatch { expected, got } => {
                write!(f, "expected {expected} hex bytes, got {got}")
            }
        }
    }
}

impl std::error::Error for DecodeError {}

#[inline]
fn from_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

pub(crate) fn decode(input: &str) -> Result<Vec<u8>, DecodeError> {
    let bytes = input.as_bytes();
    if !bytes.len().is_multiple_of(2) {
        return Err(DecodeError::OddLength);
    }

    let mut out = Vec::with_capacity(bytes.len() / 2);
    for (i, chunk) in bytes.chunks_exact(2).enumerate() {
        let hi = from_nibble(chunk[0]).ok_or(DecodeError::InvalidChar {
            byte: chunk[0],
            index: i * 2,
        })?;
        let lo = from_nibble(chunk[1]).ok_or(DecodeError::InvalidChar {
            byte: chunk[1],
            index: i * 2 + 1,
        })?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

pub(crate) fn decode_to_slice(input: &[u8], out: &mut [u8]) -> Result<(), DecodeError> {
    if input.len() != out.len() * 2 {
        return Err(DecodeError::LengthMismatch {
            expected: out.len() * 2,
            got: input.len(),
        });
    }

    for (i, byte) in out.iter_mut().enumerate() {
        let hi = from_nibble(input[i * 2]).ok_or(DecodeError::InvalidChar {
            byte: input[i * 2],
            index: i * 2,
        })?;
        let lo = from_nibble(input[i * 2 + 1]).ok_or(DecodeError::InvalidChar {
            byte: input[i * 2 + 1],
            index: i * 2 + 1,
        })?;
        *byte = (hi << 4) | lo;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_decode() {
        assert_eq!(decode("").unwrap(), Vec::<u8>::new());
        assert_eq!(decode("00").unwrap(), vec![0]);
        assert_eq!(decode("ABCDef").unwrap(), vec![0xAB, 0xCD, 0xEF]);
        assert_eq!(decode("010203").unwrap(), vec![1, 2, 3]);

        assert_eq!(decode("0").unwrap_err(), DecodeError::OddLength);
        assert!(matches!(
            decode("0Z").unwrap_err(),
            DecodeError::InvalidChar { .. }
        ));
    }

    #[test]
    fn test_decode_to_slice() {
        let mut out = [0u8; 3];
        decode_to_slice(b"010203", &mut out).unwrap();
        assert_eq!(out, [1, 2, 3]);

        assert!(matches!(
            decode_to_slice(b"01", &mut [0u8; 3]).unwrap_err(),
            DecodeError::LengthMismatch {
                expected: 6,
                got: 2
            }
        ));
        assert!(matches!(
            decode_to_slice(b"01ZZ02", &mut out).unwrap_err(),
            DecodeError::InvalidChar { .. }
        ));
    }
}
