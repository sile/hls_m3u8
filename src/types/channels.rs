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
    /// let mut channels = Channels::new(6, false);
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
    /// let channels = Channels::new(6, false);
    ///
    /// println!("CHANNELS=\"{}\"", channels);
    /// # assert_eq!(format!("CHANNELS=\"{}\"", channels), "CHANNELS=\"6\"".to_string());
    /// ```
    #[must_use]
    pub const fn new(number: u64, has_joc_content: bool) -> Self {
        Self {
            number,
            has_joc_content,
        }
    }
}

impl FromStr for Channels {
    type Err = Error;

    /// Makes a new [`Channels`] struct from a str.
    ///
    /// Has significant extra logic to account for the addition of Enhanced AC-3 audio with JOC, which
    /// allows for strings like "16/JOC".
    /// #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Lots of extra logic to deal with Dolby Atmos
        let split: Vec<&str> = input.split("/").collect::<Vec<&str>>();
        let num_str = split
            .get(0)
            .ok_or_else(|| Error::missing_value("Missing Channel value"))?;

        let joc_coding = match split.get(1) {
            Some(&"JOC") => true,
            Some(_) => return Err(Error::invalid_input()),
            None => false,
        };

        Ok(Self::new(
            num_str.parse().map_err(|e| Error::parse_int(num_str, e))?,
            joc_coding,
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
        assert_eq!(Channels::new(6, false).to_string(), "6".to_string());

        assert_eq!(Channels::new(7, true).to_string(), "7".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!(Channels::new(6, false), Channels::from_str("6").unwrap());

        assert!(Channels::from_str("garbage").is_err());
        assert!(Channels::from_str("").is_err());
    }

    #[test]
    fn test_parser_dolby_atmos() {
        assert_eq!(
            Channels::new(16, true),
            Channels::from_str("16/JOC").unwrap()
        );
        assert!(Channels::from_str("16/JOKE").is_err());
    }
}
