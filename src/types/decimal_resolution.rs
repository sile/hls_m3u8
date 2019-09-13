use std::fmt;
use std::str::{self, FromStr};

use crate::Error;

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

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut tokens = input.splitn(2, 'x');
        let width = tokens.next().ok_or(Error::missing_value("width"))?;
        let height = tokens.next().ok_or(Error::missing_value("height"))?;

        Ok(DecimalResolution {
            width: width.parse().map_err(|e| Error::custom(e))?,
            height: height.parse().map_err(|e| Error::custom(e))?,
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
