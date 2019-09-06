use crate::{ErrorKind, Result};
use trackable::error::ErrorKindExt;

pub fn parse_yes_or_no(s: &str) -> Result<bool> {
    match s {
        "YES" => Ok(true),
        "NO" => Ok(false),
        _ => track_panic!(ErrorKind::InvalidInput, "Unexpected value: {:?}", s),
    }
}

pub fn parse_u64(s: &str) -> Result<u64> {
    let n = track!(s.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
    Ok(n)
}
