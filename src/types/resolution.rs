use std::str::FromStr;

use derive_more::Display;
use shorthand::ShortHand;

use crate::Error;

/// The number of distinct pixels in each dimension that can be displayed (e.g.
/// 1920x1080).
///
/// For example Full HD has a resolution of 1920x1080.
#[derive(ShortHand, Ord, PartialOrd, Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{}x{}", width, height)]
#[shorthand(enable(must_use))]
pub struct Resolution {
    /// Horizontal pixel dimension.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::Resolution;
    /// let mut resolution = Resolution::new(1280, 720);
    ///
    /// resolution.set_width(1000);
    /// assert_eq!(resolution.width(), 1000);
    /// ```
    width: usize,
    /// Vertical pixel dimension.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::Resolution;
    /// let mut resolution = Resolution::new(1280, 720);
    ///
    /// resolution.set_height(800);
    /// assert_eq!(resolution.height(), 800);
    /// ```
    height: usize,
}

impl Resolution {
    /// Constructs a new [`Resolution`].
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::Resolution;
    /// let resolution = Resolution::new(1920, 1080);
    /// ```
    #[must_use]
    pub const fn new(width: usize, height: usize) -> Self { Self { width, height } }
}

impl From<(usize, usize)> for Resolution {
    fn from(value: (usize, usize)) -> Self { Self::new(value.0, value.1) }
}

impl Into<(usize, usize)> for Resolution {
    fn into(self) -> (usize, usize) { (self.width, self.height) }
}

impl FromStr for Resolution {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut input = input.splitn(2, 'x');

        let width = input
            .next()
            .ok_or_else(|| Error::custom("missing width for `Resolution` or an invalid input"))
            .and_then(|v| v.parse().map_err(|e| Error::parse_int(v, e)))?;

        let height = input
            .next()
            .ok_or_else(|| Error::custom("missing height for `Resolution` or an invalid input"))
            .and_then(|v| v.parse().map_err(|e| Error::parse_int(v, e)))?;

        Ok(Self { width, height })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            Resolution::new(1920, 1080).to_string(),
            "1920x1080".to_string()
        );

        assert_eq!(
            Resolution::new(1280, 720).to_string(),
            "1280x720".to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            Resolution::new(1920, 1080),
            "1920x1080".parse::<Resolution>().unwrap()
        );

        assert_eq!(
            Resolution::new(1280, 720),
            "1280x720".parse::<Resolution>().unwrap()
        );

        assert!("1280".parse::<Resolution>().is_err());
    }

    #[test]
    fn test_width() {
        assert_eq!(Resolution::new(1920, 1080).width(), 1920);
        assert_eq!(Resolution::new(1920, 1080).set_width(12).width(), 12);
    }

    #[test]
    fn test_height() {
        assert_eq!(Resolution::new(1920, 1080).height(), 1080);
        assert_eq!(Resolution::new(1920, 1080).set_height(12).height(), 12);
    }

    #[test]
    fn test_from() {
        assert_eq!(Resolution::from((1920, 1080)), Resolution::new(1920, 1080));
    }

    #[test]
    fn test_into() {
        assert_eq!((1920, 1080), Resolution::new(1920, 1080).into());
    }
}
