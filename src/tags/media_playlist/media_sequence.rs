use crate::types::ProtocolVersion;
use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::FromStr;
use trackable::error::ErrorKindExt;

/// [4.3.3.2. EXT-X-MEDIA-SEQUENCE]
///
/// [4.3.3.2. EXT-X-MEDIA-SEQUENCE]: https://tools.ietf.org/html/rfc8216#section-4.3.3.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtXMediaSequence {
    seq_num: u64,
}

impl ExtXMediaSequence {
    pub(crate) const PREFIX: &'static str = "#EXT-X-MEDIA-SEQUENCE:";

    /// Makes a new `ExtXMediaSequence` tag.
    pub const fn new(seq_num: u64) -> Self {
        ExtXMediaSequence { seq_num }
    }

    /// Returns the sequence number of the first media segment that appears in the associated playlist.
    pub const fn seq_num(self) -> u64 {
        self.seq_num
    }

    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXMediaSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.seq_num)
    }
}

impl FromStr for ExtXMediaSequence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        track_assert!(s.starts_with(Self::PREFIX), ErrorKind::InvalidInput);
        let seq_num = may_invalid!(s.split_at(Self::PREFIX.len()).1.parse())?;
        Ok(ExtXMediaSequence { seq_num })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ext_x_media_sequence() {
        let tag = ExtXMediaSequence::new(123);
        let text = "#EXT-X-MEDIA-SEQUENCE:123";
        assert_eq!(text.parse().ok(), Some(tag));
        assert_eq!(tag.to_string(), text);
        assert_eq!(tag.requires_version(), ProtocolVersion::V1);
    }
}
