use std::str::FromStr;

use derive_more::Display;
use shorthand::ShortHand;

use crate::Error;

/// This is a simple wrapper type for the display resolution.
///
/// For example Full HD has a resolution of 1920x1080.
///
/// See: [4.2. Attribute Lists]
///
/// [4.2. Attribute Lists]: https://tools.ietf.org/html/rfc8216#section-4.2
#[derive(ShortHand, Ord, PartialOrd, Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{}x{}", width, height)]
#[shorthand(enable(must_use))]
pub struct Resolution {
    /// Horizontal pixel dimension.
    width: usize,
    /// Vertical pixel dimension.
    height: usize,
}

impl Resolution {
    /// Creates a new [`Resolution`].
    pub const fn new(width: usize, height: usize) -> Self { Self { width, height } }
}

/// A [`Resolution`] can be constructed from a tuple `(width, height)`.
impl From<(usize, usize)> for Resolution {
    fn from(value: (usize, usize)) -> Self { Self::new(value.0, value.1) }
}

impl FromStr for Resolution {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut input = input.splitn(2, 'x');

        let width = input
            .next()
            .ok_or_else(|| Error::custom("missing width for `Resolution` or an invalid input"))
            .and_then(|v| v.parse().map_err(Error::parse_int))?;

        let height = input
            .next()
            .ok_or_else(|| Error::custom("missing height for `Resolution` or an invalid input"))
            .and_then(|v| v.parse().map_err(Error::parse_int))?;

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
}
