use crate::Error;

#[inline]
fn from_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn invalid_char(byte: u8, index: usize) -> Error {
    Error::invalid_hex(format!("invalid character {byte:#04x} at index {index}"))
}

pub(crate) fn decode(input: &str) -> Result<Vec<u8>, Error> {
    let bytes = input.as_bytes();
    if !bytes.len().is_multiple_of(2) {
        return Err(Error::invalid_hex("odd string length"));
    }

    let mut out = Vec::with_capacity(bytes.len() / 2);
    for (i, chunk) in bytes.chunks_exact(2).enumerate() {
        let hi = from_nibble(chunk[0]).ok_or_else(|| invalid_char(chunk[0], i * 2))?;
        let lo = from_nibble(chunk[1]).ok_or_else(|| invalid_char(chunk[1], i * 2 + 1))?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

pub(crate) fn decode_to_slice(input: &[u8], out: &mut [u8]) -> Result<(), Error> {
    if input.len() != out.len() * 2 {
        return Err(Error::invalid_hex(format!(
            "expected {} characters, got {}",
            out.len() * 2,
            input.len()
        )));
    }

    for (i, byte) in out.iter_mut().enumerate() {
        let hi = from_nibble(input[i * 2]).ok_or_else(|| invalid_char(input[i * 2], i * 2))?;
        let lo = from_nibble(input[i * 2 + 1])
            .ok_or_else(|| invalid_char(input[i * 2 + 1], i * 2 + 1))?;
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

        assert!(decode("0").is_err());
        assert!(decode("0Z").is_err());
    }

    #[test]
    fn test_decode_to_slice() {
        let mut out = [0u8; 3];
        decode_to_slice(b"010203", &mut out).unwrap();
        assert_eq!(out, [1, 2, 3]);

        assert!(decode_to_slice(b"01", &mut [0u8; 3]).is_err());
        assert!(decode_to_slice(b"01ZZ02", &mut out).is_err());
    }
}
