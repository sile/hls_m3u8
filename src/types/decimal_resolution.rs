use crate::{Error, ErrorKind, Result};
use std::fmt;
use std::str::{self, FromStr};
use trackable::error::ErrorKindExt;

/// Decimal resolution.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimalResolution {
    /// Horizontal pixel dimension.
    pub width: usize,

    /// Vertical pixel dimension.
    pub height: usize,
}

impl fmt::Display for DecimalResolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl FromStr for DecimalResolution {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut tokens = s.splitn(2, 'x');
        let width = tokens.next().expect("Never fails");
        let height = track_assert_some!(tokens.next(), ErrorKind::InvalidInput);
        Ok(DecimalResolution {
            width: track!(width.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?,
            height: track!(height.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let decimal_resolution = DecimalResolution {
            width: 1920,
            height: 1080,
        };
        assert_eq!(decimal_resolution.to_string(), "1920x1080".to_string());

        let decimal_resolution = DecimalResolution {
            width: 1280,
            height: 720,
        };
        assert_eq!(decimal_resolution.to_string(), "1280x720".to_string());
    }

    #[test]
    fn test_parse() {
        let decimal_resolution = DecimalResolution {
            width: 1920,
            height: 1080,
        };
        assert_eq!(
            decimal_resolution,
            "1920x1080".parse::<DecimalResolution>().unwrap()
        );

        let decimal_resolution = DecimalResolution {
            width: 1280,
            height: 720,
        };
        assert_eq!(
            decimal_resolution,
            "1280x720".parse::<DecimalResolution>().unwrap()
        );
    }
}
