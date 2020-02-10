use strum::{Display, EnumString};

/// HDCP level.
///
/// See: [4.3.4.2. EXT-X-STREAM-INF]
///
/// [4.3.4.2. EXT-X-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.2
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Ord, PartialOrd, Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[strum(serialize_all = "SCREAMING-KEBAB-CASE")]
pub enum HdcpLevel {
    #[strum(serialize = "TYPE-0")]
    Type0,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        let level = HdcpLevel::Type0;
        assert_eq!(level.to_string(), "TYPE-0".to_string());

        let level = HdcpLevel::None;
        assert_eq!(level.to_string(), "NONE".to_string());
    }

    #[test]
    fn test_parser() {
        let level = HdcpLevel::Type0;
        assert_eq!(level, "TYPE-0".parse::<HdcpLevel>().unwrap());

        let level = HdcpLevel::None;
        assert_eq!(level, "NONE".parse::<HdcpLevel>().unwrap());

        assert!("unk".parse::<HdcpLevel>().is_err());
    }
}
