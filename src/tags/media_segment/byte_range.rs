use crate::types::{ByteRange, ProtocolVersion};
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;
use trackable::error::ErrorKindExt;

/// [4.3.2.2. EXT-X-BYTERANGE]
///
/// [4.3.2.2. EXT-X-BYTERANGE]: https://tools.ietf.org/html/rfc8216#section-4.3.2.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXByteRange {
    range: ByteRange,
}

impl ExtXByteRange {
    pub(crate) const PREFIX: &'static str = "#EXT-X-BYTERANGE:";

    /// Makes a new `ExtXByteRange` tag.
    pub const fn new(range: ByteRange) -> Self {
        ExtXByteRange { range }
    }

    /// Returns the range of the associated media segment.
    pub const fn range(&self) -> ByteRange {
        self.range
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V4
    }
}

impl fmt::Display for ExtXByteRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.range)
    }
}

impl FromStr for ExtXByteRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let range = may_invalid!(s.split_at(Self::PREFIX.len()).1.parse())?;
        Ok(ExtXByteRange { range })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_byterange() {
        let tag = ExtXByteRange::new(ByteRange {
            length: 3,
            start: None,
        });
        assert_eq!("#EXT-X-BYTERANGE:3".parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), "#EXT-X-BYTERANGE:3");
        assert_eq!(tag.requires_version(), ProtocolVersion::V4);

        let tag = ExtXByteRange::new(ByteRange {
            length: 3,
            start: Some(5),
        });
        assert_eq!("#EXT-X-BYTERANGE:3@5".parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), "#EXT-X-BYTERANGE:3@5");
        assert_eq!(tag.requires_version(), ProtocolVersion::V4);
    }
}
