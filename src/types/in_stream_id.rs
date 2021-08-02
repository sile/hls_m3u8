use strum::{Display, EnumString};

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
#[allow(missing_docs)]
#[derive(Ord, PartialOrd, Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
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
