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
/// The second parameter identifies the encoding of object-based audio used by
/// the rendition. This parameter is a comma-separated list of Audio
/// Object Coding Identifiers. It is optional. An Audio Object
/// Coding Identifier is a string containing characters from the set
/// `[A..Z]`, `[0..9]`, and `'-'`.  They are codec-specific. A parameter
/// value of consisting solely of the dash character (`'-'`) indicates
/// that the audio is not object-based.
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
/// # Note
/// Currently there are no example playlists in the documentation,
/// or in popular m3u8 libraries, showing a usage for the second parameter
/// of [`Channels`], so if you have one please open an issue on github!
///
/// [`MediaSegment`]: crate::MediaSegment
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Channels {
    first_parameter: u64,
    second_parameter: Option<Vec<String>>,
}

impl Channels {
    /// Makes a new [`Channels`] struct.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::Channels;
    /// let mut channels = Channels::new(6);
    /// ```
    pub const fn new(value: u64) -> Self {
        Self {
            first_parameter: value,
            second_parameter: None,
        }
    }

    /// Returns the first parameter.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::Channels;
    /// let mut channels = Channels::new(6);
    ///
    /// assert_eq!(channels.first_parameter(), 6);
    /// ```
    pub const fn first_parameter(&self) -> u64 { self.first_parameter }

    /// Sets the first parameter.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::Channels;
    /// let mut channels = Channels::new(3);
    ///
    /// channels.set_first_parameter(6);
    /// assert_eq!(channels.first_parameter(), 6)
    /// ```
    pub fn set_first_parameter(&mut self, value: u64) -> &mut Self {
        self.first_parameter = value;
        self
    }

    /// Returns the second parameter, if there is any!
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::Channels;
    /// let mut channels = Channels::new(3);
    /// # assert_eq!(channels.second_parameter(), &None);
    ///
    /// channels.set_second_parameter(Some(vec!["AAC", "MP3"]));
    /// assert_eq!(
    ///     channels.second_parameter(),
    ///     &Some(vec!["AAC".to_string(), "MP3".to_string()])
    /// )
    /// ```
    ///
    /// # Note
    /// Currently there is no use for this parameter.
    pub const fn second_parameter(&self) -> &Option<Vec<String>> { &self.second_parameter }

    /// Sets the second parameter.
    ///
    /// # Example
    /// ```
    /// # use hls_m3u8::types::Channels;
    /// let mut channels = Channels::new(3);
    /// # assert_eq!(channels.second_parameter(), &None);
    ///
    /// channels.set_second_parameter(Some(vec!["AAC", "MP3"]));
    /// assert_eq!(
    ///     channels.second_parameter(),
    ///     &Some(vec!["AAC".to_string(), "MP3".to_string()])
    /// )
    /// ```
    ///
    /// # Note
    /// Currently there is no use for this parameter.
    pub fn set_second_parameter<T: ToString>(&mut self, value: Option<Vec<T>>) -> &mut Self {
        self.second_parameter = value.map(|v| v.into_iter().map(|s| s.to_string()).collect());
        self
    }
}

impl FromStr for Channels {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parameters = input.split('/').collect::<Vec<_>>();
        let first_parameter = parameters
            .first()
            .ok_or_else(|| Error::missing_attribute("First parameter of channels!"))?
            .parse()?;

        let second_parameter = parameters
            .get(1)
            .map(|v| v.split(',').map(|v| v.to_string()).collect());

        Ok(Self {
            first_parameter,
            second_parameter,
        })
    }
}

impl fmt::Display for Channels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.first_parameter)?;

        if let Some(second) = &self.second_parameter {
            write!(f, "/{}", second.join(","))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let mut channels = Channels::new(6);
        assert_eq!(channels.to_string(), "6".to_string());

        channels.set_first_parameter(7);
        assert_eq!(channels.to_string(), "7".to_string());

        assert_eq!(
            "6/P,K,J".to_string(),
            Channels::new(6)
                .set_second_parameter(Some(vec!["P", "K", "J"]))
                .to_string()
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!("6".parse::<Channels>().unwrap(), Channels::new(6));
        let mut result = Channels::new(6);
        result.set_second_parameter(Some(vec!["P", "K", "J"]));

        assert_eq!("6/P,K,J".parse::<Channels>().unwrap(), result);

        assert!("garbage".parse::<Channels>().is_err());
        assert!("".parse::<Channels>().is_err());
    }
}
