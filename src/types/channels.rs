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

    /// Dolby Atmos mix type
    ///
    /// # Example
    ///
    mix_type: Option<MixType>,
    // /// Flag for JOC (Dolby Atmos).
    // ///
    // /// # Example
    // ///
    // /// ```
    // /// # use hls_m3u8::types::Channels;
    // /// let mut channels = Channels::new(6);
    // /// assert_eq!(channels.has_joc_content(), false);
    // ///
    // /// channels.set_has_joc_content(true);
    // /// assert_eq!(channels.has_joc_content(), true);
    // /// ```
    // has_joc_content: bool,
}

/// Dolby atmos mix type for a [`Channels`] configuration
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MixType {
    /// JOC mix
    JOC,
    /// Binaural mix
    Binaural,
    /// Downmixed content
    Downmix,
}

impl fmt::Display for MixType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MixType::JOC => write!(f, "JOC"),
            MixType::Binaural => write!(f, "-/BINAURAL"),
            MixType::Downmix => write!(f, "-/DOWNMIX"),
        }
    }
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
    #[must_use]
    pub const fn new(number: u64) -> Self {
        Self {
            number,
            mix_type: None,
        }
    }
}

impl FromStr for Channels {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.split_once('/') {
            None => {
                let channels = input.parse().map_err(|e| Error::parse_int(input, e))?;
                Ok(Self::new(channels))
            }
            Some((channels, mix_type)) => {
                let channels = channels
                    .parse()
                    .map_err(|e| Error::parse_int(channels, e))?;

                let mix_type = if let Some((no_joc, ty)) = mix_type.split_once("/") {
                    if no_joc != "-" {
                        return Err(Error::invalid_input());
                    }

                    match ty {
                        "BINAURAL" => MixType::Binaural,
                        "DOWNMIX" => MixType::Downmix,
                        _ => return Err(Error::invalid_input()),
                    }
                } else if mix_type == "JOC" {
                    MixType::JOC
                } else {
                    return Err(Error::invalid_input());
                };

                Ok(Self {
                    number: channels,
                    mix_type: Some(mix_type),
                })
            }
        }
    }
}

impl fmt::Display for Channels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mix_type {
            Some(ty) => write!(f, "{}/{}", self.number, ty)?,
            None => write!(f, "{}", self.number)?,
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
        test_channels.set_mix_type(Some(MixType::JOC));

        assert_eq!(test_channels, Channels::from_str("16/JOC").unwrap());

        test_channels.set_mix_type(Some(MixType::Binaural));
        assert_eq!(test_channels, Channels::from_str("16/-/BINAURAL").unwrap());
        test_channels.set_mix_type(Some(MixType::Downmix));
        assert_eq!(test_channels, Channels::from_str("16/-/DOWNMIX").unwrap());

        assert!(Channels::from_str("16/JOKE").is_err());
        assert!(Channels::from_str("16/JOC/4").is_err());
    }
}
