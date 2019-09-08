/// Session data.
///
/// See: [4.3.4.4. EXT-X-SESSION-DATA]
///
/// [4.3.4.4. EXT-X-SESSION-DATA]: https://tools.ietf.org/html/rfc8216#section-4.3.4.4
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SessionData {
    Value(String),
    Uri(String),
}
