use std::fmt;
use std::str::FromStr;

use crate::Error;
use crate::traits::RequiredVersion;
use crate::types::ProtocolVersion;

/// Identifier of a rendition within the [`MediaSegment`]s in a
/// [`MediaPlaylist`].
///
/// The variants [`InStreamId::Cc1`], [`InStreamId::Cc2`], [`InStreamId::Cc3`],
/// and [`InStreamId::Cc4`] identify a Line 21 Data Services channel ([CEA608]).
///
/// The `Service` variants identify a Digital Television Closed Captioning
/// ([CEA708]) service block number. The `Service` variants range from
/// [`InStreamId::Service1`] to [`InStreamId::Service63`].
///
/// [CEA608]: https://tools.ietf.org/html/rfc8216#ref-CEA608
/// [CEA708]: https://tools.ietf.org/html/rfc8216#ref-CEA708
/// [`MediaSegment`]: crate::MediaSegment
/// [`MediaPlaylist`]: crate::MediaPlaylist
#[non_exhaustive]
#[expect(missing_docs)]
#[derive(Ord, PartialOrd, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

macro_rules! in_stream_id_string_table {
    ($($variant:ident => $name:literal),* $(,)?) => {
        impl fmt::Display for InStreamId {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(match self {
                    $(Self::$variant => $name,)*
                })
            }
        }

        impl FromStr for InStreamId {
            type Err = Error;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                match input {
                    $($name => Ok(Self::$variant),)*
                    _ => Err(Error::custom(format!("invalid in-stream id: {input:?}"))),
                }
            }
        }
    };
}

in_stream_id_string_table! {
    Cc1 => "CC1",
    Cc2 => "CC2",
    Cc3 => "CC3",
    Cc4 => "CC4",
    Service1 => "SERVICE1",
    Service2 => "SERVICE2",
    Service3 => "SERVICE3",
    Service4 => "SERVICE4",
    Service5 => "SERVICE5",
    Service6 => "SERVICE6",
    Service7 => "SERVICE7",
    Service8 => "SERVICE8",
    Service9 => "SERVICE9",
    Service10 => "SERVICE10",
    Service11 => "SERVICE11",
    Service12 => "SERVICE12",
    Service13 => "SERVICE13",
    Service14 => "SERVICE14",
    Service15 => "SERVICE15",
    Service16 => "SERVICE16",
    Service17 => "SERVICE17",
    Service18 => "SERVICE18",
    Service19 => "SERVICE19",
    Service20 => "SERVICE20",
    Service21 => "SERVICE21",
    Service22 => "SERVICE22",
    Service23 => "SERVICE23",
    Service24 => "SERVICE24",
    Service25 => "SERVICE25",
    Service26 => "SERVICE26",
    Service27 => "SERVICE27",
    Service28 => "SERVICE28",
    Service29 => "SERVICE29",
    Service30 => "SERVICE30",
    Service31 => "SERVICE31",
    Service32 => "SERVICE32",
    Service33 => "SERVICE33",
    Service34 => "SERVICE34",
    Service35 => "SERVICE35",
    Service36 => "SERVICE36",
    Service37 => "SERVICE37",
    Service38 => "SERVICE38",
    Service39 => "SERVICE39",
    Service40 => "SERVICE40",
    Service41 => "SERVICE41",
    Service42 => "SERVICE42",
    Service43 => "SERVICE43",
    Service44 => "SERVICE44",
    Service45 => "SERVICE45",
    Service46 => "SERVICE46",
    Service47 => "SERVICE47",
    Service48 => "SERVICE48",
    Service49 => "SERVICE49",
    Service50 => "SERVICE50",
    Service51 => "SERVICE51",
    Service52 => "SERVICE52",
    Service53 => "SERVICE53",
    Service54 => "SERVICE54",
    Service55 => "SERVICE55",
    Service56 => "SERVICE56",
    Service57 => "SERVICE57",
    Service58 => "SERVICE58",
    Service59 => "SERVICE59",
    Service60 => "SERVICE60",
    Service61 => "SERVICE61",
    Service62 => "SERVICE62",
    Service63 => "SERVICE63",
}

/// The variants [`InStreamId::Cc1`], [`InStreamId::Cc2`], [`InStreamId::Cc3`]
/// and [`InStreamId::Cc4`] require [`ProtocolVersion::V1`], the other
/// [`ProtocolVersion::V7`].
impl RequiredVersion for InStreamId {
    fn required_version(&self) -> ProtocolVersion {
        match &self {
            Self::Cc1 | Self::Cc2 | Self::Cc3 | Self::Cc4 => ProtocolVersion::V1,
            _ => ProtocolVersion::V7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
