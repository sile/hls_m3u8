use core::fmt;
use core::str::FromStr;

use crate::Error;

/// Specifies a list of parameters.
///
/// # `MediaType::Audio`
/// The first parameter is a count of audio channels expressed as a [`u64`],
/// indicating the maximum number of independent, simultaneous audio channels
/// present in any [`MediaSegment`] in the rendition. For example, an
/// `AC-3 5.1` rendition would have a `CHANNELS="6"` attribute.
///
/// # Example
/// Creating a `CHANNELS="6"` attribute
/// ```
/// # use hls_m3u8::types::Channels;
/// let mut channels = Channels::new(6);
///
/// assert_eq!(
///     format!("CHANNELS=\"{}\"", channels),
///     "CHANNELS=\"6\"".to_string()
/// );
/// ```
///
/// [`MediaSegment`]: crate::MediaSegment
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Channels {
    channel_number: u64,
    unknown: Vec<String>,
}

impl Channels {
    /// Makes a new [`Channels`] struct.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::Channels;
    /// let mut channels = Channels::new(6);
    /// ```
    pub fn new(value: u64) -> Self {
        Self {
            channel_number: value,
            unknown: vec![],
        }
    }

    /// Returns the channel number.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::Channels;
    /// let mut channels = Channels::new(6);
    ///
    /// assert_eq!(channels.channel_number(), 6);
    /// ```
    pub const fn channel_number(&self) -> u64 { self.channel_number }

    /// Sets the channel number.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::Channels;
    /// let mut channels = Channels::new(3);
    ///
    /// channels.set_channel_number(6);
    /// assert_eq!(channels.channel_number(), 6)
    /// ```
    pub fn set_channel_number(&mut self, value: u64) -> &mut Self {
        self.channel_number = value;
        self
    }
}

impl FromStr for Channels {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parameters = input.split('/').collect::<Vec<_>>();
        let channel_number = parameters
            .first()
            .ok_or_else(|| Error::missing_attribute("First parameter of channels!"))?
            .parse()
            .map_err(Error::parse_int)?;

        Ok(Self {
            channel_number,
            unknown: parameters[1..].iter().map(|v| v.to_string()).collect(),
        })
    }
}

impl fmt::Display for Channels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.channel_number)?;
        if !self.unknown.is_empty() {
            write!(f, "{}", self.unknown.join(","))?;
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
        let mut channels = Channels::new(6);
        assert_eq!(channels.to_string(), "6".to_string());

        channels.set_channel_number(7);
        assert_eq!(channels.to_string(), "7".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!("6".parse::<Channels>().unwrap(), Channels::new(6));
        assert!("garbage".parse::<Channels>().is_err());
        assert!("".parse::<Channels>().is_err());
    }
}
