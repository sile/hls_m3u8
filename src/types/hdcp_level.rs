use std::fmt;
use std::str::FromStr;

use crate::Error;

/// HDCP ([`High-bandwidth Digital Content Protection`]) level.
///
/// [`High-bandwidth Digital Content Protection`]:
/// https://www.digital-cp.com/sites/default/files/specifications/HDCP%20on%20HDMI%20Specification%20Rev2_2_Final1.pdf
#[non_exhaustive]
#[derive(Ord, PartialOrd, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HdcpLevel {
    /// The associated [`VariantStream`] could fail to play unless the output is
    /// protected by High-bandwidth Digital Content Protection ([`HDCP`]) Type 0
    /// or equivalent.
    ///
    /// [`VariantStream`]: crate::tags::VariantStream
    /// [`HDCP`]: https://www.digital-cp.com/sites/default/files/specifications/HDCP%20on%20HDMI%20Specification%20Rev2_2_Final1.pdf
    Type0,
    /// The content does not require output copy protection.
    None,
}

impl fmt::Display for HdcpLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Type0 => "TYPE-0",
            Self::None => "NONE",
        })
    }
}

impl FromStr for HdcpLevel {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "TYPE-0" => Ok(Self::Type0),
            "NONE" => Ok(Self::None),
            _ => Err(Error::custom(format!("invalid HDCP level: {input:?}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(HdcpLevel::Type0.to_string(), "TYPE-0".to_string());
        assert_eq!(HdcpLevel::None.to_string(), "NONE".to_string());
    }

    #[test]
    fn test_parser() {
        assert_eq!(HdcpLevel::Type0, "TYPE-0".parse().unwrap());
        assert_eq!(HdcpLevel::None, "NONE".parse().unwrap());

        assert!("unk".parse::<HdcpLevel>().is_err());
    }
}
