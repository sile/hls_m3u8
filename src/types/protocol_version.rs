use std::fmt;
use std::str::FromStr;

use crate::Error;

/// The [`ProtocolVersion`] specifies which `m3u8` revision is required, to
/// parse a certain tag correctly.
#[non_exhaustive]
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

impl ProtocolVersion {
    /// Returns the latest [`ProtocolVersion`] that is supported by
    /// this library.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::ProtocolVersion;
    /// assert_eq!(ProtocolVersion::latest(), ProtocolVersion::V7);
    /// ```
    pub const fn latest() -> Self { Self::V7 }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::V1 => write!(f, "1"),
            Self::V2 => write!(f, "2"),
            Self::V3 => write!(f, "3"),
            Self::V4 => write!(f, "4"),
            Self::V5 => write!(f, "5"),
            Self::V6 => write!(f, "6"),
            Self::V7 => write!(f, "7"),
        }
    }
}

impl FromStr for ProtocolVersion {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok({
            match input.trim() {
                "1" => Self::V1,
                "2" => Self::V2,
                "3" => Self::V3,
                "4" => Self::V4,
                "5" => Self::V5,
                "6" => Self::V6,
                "7" => Self::V7,
                _ => return Err(Error::unknown_protocol_version(input)),
            }
        })
    }
}

/// The default is [`ProtocolVersion::V1`].
impl Default for ProtocolVersion {
    fn default() -> Self { Self::V1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(ProtocolVersion::V1.to_string(), "1".to_string());
        assert_eq!(ProtocolVersion::V2.to_string(), "2".to_string());
        assert_eq!(ProtocolVersion::V3.to_string(), "3".to_string());
        assert_eq!(ProtocolVersion::V4.to_string(), "4".to_string());
        assert_eq!(ProtocolVersion::V5.to_string(), "5".to_string());
        assert_eq!(ProtocolVersion::V6.to_string(), "6".to_string());
        assert_eq!(ProtocolVersion::V7.to_string(), "7".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!(ProtocolVersion::V1, "1".parse().unwrap());
        assert_eq!(ProtocolVersion::V2, "2".parse().unwrap());
        assert_eq!(ProtocolVersion::V3, "3".parse().unwrap());
        assert_eq!(ProtocolVersion::V4, "4".parse().unwrap());
        assert_eq!(ProtocolVersion::V5, "5".parse().unwrap());
        assert_eq!(ProtocolVersion::V6, "6".parse().unwrap());
        assert_eq!(ProtocolVersion::V7, "7".parse().unwrap());

        assert_eq!(ProtocolVersion::V7, " 7 ".parse().unwrap());
        assert!("garbage".parse::<ProtocolVersion>().is_err());
    }

    #[test]
    fn test_default() {
        assert_eq!(ProtocolVersion::default(), ProtocolVersion::V1);
    }

    #[test]
    fn test_latest() {
        assert_eq!(ProtocolVersion::latest(), ProtocolVersion::V7);
    }
}
