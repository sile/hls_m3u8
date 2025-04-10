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
    /// assert_eq!(channels.has_joc_content(), false);
    ///
    /// channels.set_number(5);
    /// channels.set_has_joc_content(true);
    /// assert_eq!(channels.number(), 5);
    /// assert_eq!(channels.has_joc_content(), true);
    /// ```
    number: u64,
    has_joc_content: bool,
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
    /// #[inline]
    #[must_use]
    pub const fn new(number: u64) -> Self {
        Self {
            number,
            has_joc_content: false,
        }
    }
}

impl FromStr for Channels {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Lots of extra logic to deal with Dolby Atmos
        let split: Vec<&str> = input.splitn(2, "/").collect::<Vec<&str>>();
        let num_str = split
            .get(0)
            .ok_or_else(|| Error::missing_value("Missing Channel value"))?;

        let mut new_channels =
            Self::new(num_str.parse().map_err(|e| Error::parse_int(num_str, e))?);

        match split.get(1) {
            Some(&"JOC") => new_channels.set_has_joc_content(true),
            Some(_) => return Err(Error::invalid_input()),
            None => &mut new_channels,
        };

        Ok(new_channels)
    }
}

impl fmt::Display for Channels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.has_joc_content {
            true => write!(f, "{}/JOC", self.number)?,
            false => write!(f, "{}", self.number)?,
        }
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

        let test_channel = Channels::from_str("7/JOC").unwrap();

        assert_eq!(test_channel.to_string(), "7/JOC".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!(Channels::new(6), Channels::from_str("6").unwrap());

        assert!(Channels::from_str("garbage").is_err());
        assert!(Channels::from_str("").is_err());
    }

    #[test]
    fn test_parser_dolby_atmos() {
        let mut test_channels = Channels::new(16);
        test_channels.set_has_joc_content(true);

        assert_eq!(test_channels, Channels::from_str("16/JOC").unwrap());

        assert!(Channels::from_str("16/JOKE").is_err());
        assert!(Channels::from_str("16/JOC/4").is_err());
    }
}
