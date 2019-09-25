use std::fmt;
use std::str::FromStr;

use crate::Error;

/// Identifier of a rendition within the segments in a media playlist.
///
/// See: [4.3.4.1. EXT-X-MEDIA]
///
/// [4.3.4.1. EXT-X-MEDIA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.1
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InStreamId {
    Cc1,
    Cc2,
    Cc3,
    Cc4,
    Service1,
    Service2,
    Service3,
    Service4,
    Service5,
    Service6,
    Service7,
    Service8,
    Service9,
    Service10,
    Service11,
    Service12,
    Service13,
    Service14,
    Service15,
    Service16,
    Service17,
    Service18,
    Service19,
    Service20,
    Service21,
    Service22,
    Service23,
    Service24,
    Service25,
    Service26,
    Service27,
    Service28,
    Service29,
    Service30,
    Service31,
    Service32,
    Service33,
    Service34,
    Service35,
    Service36,
    Service37,
    Service38,
    Service39,
    Service40,
    Service41,
    Service42,
    Service43,
    Service44,
    Service45,
    Service46,
    Service47,
    Service48,
    Service49,
    Service50,
    Service51,
    Service52,
    Service53,
    Service54,
    Service55,
    Service56,
    Service57,
    Service58,
    Service59,
    Service60,
    Service61,
    Service62,
    Service63,
}

impl fmt::Display for InStreamId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format!("{:?}", self).to_uppercase().fmt(f)
    }
}

impl FromStr for InStreamId {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "CC1" => Self::Cc1,
            "CC2" => Self::Cc2,
            "CC3" => Self::Cc3,
            "CC4" => Self::Cc4,
            "SERVICE1" => Self::Service1,
            "SERVICE2" => Self::Service2,
            "SERVICE3" => Self::Service3,
            "SERVICE4" => Self::Service4,
            "SERVICE5" => Self::Service5,
            "SERVICE6" => Self::Service6,
            "SERVICE7" => Self::Service7,
            "SERVICE8" => Self::Service8,
            "SERVICE9" => Self::Service9,
            "SERVICE10" => Self::Service10,
            "SERVICE11" => Self::Service11,
            "SERVICE12" => Self::Service12,
            "SERVICE13" => Self::Service13,
            "SERVICE14" => Self::Service14,
            "SERVICE15" => Self::Service15,
            "SERVICE16" => Self::Service16,
            "SERVICE17" => Self::Service17,
            "SERVICE18" => Self::Service18,
            "SERVICE19" => Self::Service19,
            "SERVICE20" => Self::Service20,
            "SERVICE21" => Self::Service21,
            "SERVICE22" => Self::Service22,
            "SERVICE23" => Self::Service23,
            "SERVICE24" => Self::Service24,
            "SERVICE25" => Self::Service25,
            "SERVICE26" => Self::Service26,
            "SERVICE27" => Self::Service27,
            "SERVICE28" => Self::Service28,
            "SERVICE29" => Self::Service29,
            "SERVICE30" => Self::Service30,
            "SERVICE31" => Self::Service31,
            "SERVICE32" => Self::Service32,
            "SERVICE33" => Self::Service33,
            "SERVICE34" => Self::Service34,
            "SERVICE35" => Self::Service35,
            "SERVICE36" => Self::Service36,
            "SERVICE37" => Self::Service37,
            "SERVICE38" => Self::Service38,
            "SERVICE39" => Self::Service39,
            "SERVICE40" => Self::Service40,
            "SERVICE41" => Self::Service41,
            "SERVICE42" => Self::Service42,
            "SERVICE43" => Self::Service43,
            "SERVICE44" => Self::Service44,
            "SERVICE45" => Self::Service45,
            "SERVICE46" => Self::Service46,
            "SERVICE47" => Self::Service47,
            "SERVICE48" => Self::Service48,
            "SERVICE49" => Self::Service49,
            "SERVICE50" => Self::Service50,
            "SERVICE51" => Self::Service51,
            "SERVICE52" => Self::Service52,
            "SERVICE53" => Self::Service53,
            "SERVICE54" => Self::Service54,
            "SERVICE55" => Self::Service55,
            "SERVICE56" => Self::Service56,
            "SERVICE57" => Self::Service57,
            "SERVICE58" => Self::Service58,
            "SERVICE59" => Self::Service59,
            "SERVICE60" => Self::Service60,
            "SERVICE61" => Self::Service61,
            "SERVICE62" => Self::Service62,
            "SERVICE63" => Self::Service63,
            _ => return Err(Error::custom(format!("Unknown instream id: {:?}", input))),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! gen_tests {
        ( $($string:expr => $enum:expr),* ) => {
            #[test]
            fn test_display() {
                $(
                    assert_eq!($enum.to_string(), $string.to_string());
                )*
            }

            #[test]
            fn test_parser() {
                $(
                    assert_eq!($enum, $string.parse::<InStreamId>().unwrap());
                )*
                assert!("invalid_input".parse::<InStreamId>().is_err());
            }
        };
    }

    gen_tests![
        "CC1" => InStreamId::Cc1,
        "CC2" => InStreamId::Cc2,
        "CC3" => InStreamId::Cc3,
        "CC4" => InStreamId::Cc4,
        "SERVICE1" => InStreamId::Service1,
        "SERVICE2" => InStreamId::Service2,
        "SERVICE3" => InStreamId::Service3,
        "SERVICE4" => InStreamId::Service4,
        "SERVICE5" => InStreamId::Service5,
        "SERVICE6" => InStreamId::Service6,
        "SERVICE7" => InStreamId::Service7,
        "SERVICE8" => InStreamId::Service8,
        "SERVICE9" => InStreamId::Service9,
        "SERVICE10" => InStreamId::Service10,
        "SERVICE11" => InStreamId::Service11,
        "SERVICE12" => InStreamId::Service12,
        "SERVICE13" => InStreamId::Service13,
        "SERVICE14" => InStreamId::Service14,
        "SERVICE15" => InStreamId::Service15,
        "SERVICE16" => InStreamId::Service16,
        "SERVICE17" => InStreamId::Service17,
        "SERVICE18" => InStreamId::Service18,
        "SERVICE19" => InStreamId::Service19,
        "SERVICE20" => InStreamId::Service20,
        "SERVICE21" => InStreamId::Service21,
        "SERVICE22" => InStreamId::Service22,
        "SERVICE23" => InStreamId::Service23,
        "SERVICE24" => InStreamId::Service24,
        "SERVICE25" => InStreamId::Service25,
        "SERVICE26" => InStreamId::Service26,
        "SERVICE27" => InStreamId::Service27,
        "SERVICE28" => InStreamId::Service28,
        "SERVICE29" => InStreamId::Service29,
        "SERVICE30" => InStreamId::Service30,
        "SERVICE31" => InStreamId::Service31,
        "SERVICE32" => InStreamId::Service32,
        "SERVICE33" => InStreamId::Service33,
        "SERVICE34" => InStreamId::Service34,
        "SERVICE35" => InStreamId::Service35,
        "SERVICE36" => InStreamId::Service36,
        "SERVICE37" => InStreamId::Service37,
        "SERVICE38" => InStreamId::Service38,
        "SERVICE39" => InStreamId::Service39,
        "SERVICE40" => InStreamId::Service40,
        "SERVICE41" => InStreamId::Service41,
        "SERVICE42" => InStreamId::Service42,
        "SERVICE43" => InStreamId::Service43,
        "SERVICE44" => InStreamId::Service44,
        "SERVICE45" => InStreamId::Service45,
        "SERVICE46" => InStreamId::Service46,
        "SERVICE47" => InStreamId::Service47,
        "SERVICE48" => InStreamId::Service48,
        "SERVICE49" => InStreamId::Service49,
        "SERVICE50" => InStreamId::Service50,
        "SERVICE51" => InStreamId::Service51,
        "SERVICE52" => InStreamId::Service52,
        "SERVICE53" => InStreamId::Service53,
        "SERVICE54" => InStreamId::Service54,
        "SERVICE55" => InStreamId::Service55,
        "SERVICE56" => InStreamId::Service56,
        "SERVICE57" => InStreamId::Service57,
        "SERVICE58" => InStreamId::Service58,
        "SERVICE59" => InStreamId::Service59,
        "SERVICE60" => InStreamId::Service60,
        "SERVICE61" => InStreamId::Service61,
        "SERVICE62" => InStreamId::Service62,
        "SERVICE63" => InStreamId::Service63
    ];
}
