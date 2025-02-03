use core::fmt;
use core::str::FromStr;

use shorthand::ShortHand;

use crate::Error;

/// The maximum number of independent, simultaneous audio channels present in
/// any [`MediaSegment`] in the rendition.
///
/// For example, an `AC-3 5.1` rendition would have a maximum channel number of
/// 6.
///
/// [`MediaSegment`]: crate::MediaSegment
#[derive(ShortHand, Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[shorthand(enable(must_use))]
pub struct Channels {
    /// The maximum number of independent simultaneous audio channels.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::Channels;
    /// let mut channels = Channels::new(6);
    /// # assert_eq!(channels.number(), 6);
    ///
    /// channels.set_number(5);
    /// assert_eq!(channels.number(), 5);
    /// ```
    number: u64,
}

impl Channels {
    /// Makes a new [`Channels`] struct.
    ///
    /// # Example
    ///
    /// ```
    /// # use hls_m3u8::types::Channels;
    /// let channels = Channels::new(6);
    ///
    /// println!("CHANNELS=\"{}\"", channels);
    /// # assert_eq!(format!("CHANNELS=\"{}\"", channels), "CHANNELS=\"6\"".to_string());
    /// ```
    //#[inline]
    #[must_use]
    pub const fn new(number: u64) -> Self {
        Self { number }
    }
}

impl FromStr for Channels {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            input.parse().map_err(|e| Error::parse_int(input, e))?,
        ))
    }
}

impl fmt::Display for Channels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.number)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(Channels::new(6).to_string(), "6".to_string());

        assert_eq!(Channels::new(7).to_string(), "7".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!(Channels::new(6), Channels::from_str("6").unwrap());

        assert!(Channels::from_str("garbage").is_err());
        assert!(Channels::from_str("").is_err());
    }
}
