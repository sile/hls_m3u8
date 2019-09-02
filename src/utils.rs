use trackable::error::ErrorKindExt;

use crate::error::ErrorKind;

pub(crate) fn unquote<T: ToString>(value: T) -> String {
    value
        .to_string()
        // silently remove forbidden characters + quotes
        .replace("\n", "")
        .replace("\r", "")
        .replace("\"", "")
}

pub(crate) fn quote<T: ToString>(value: T) -> String {
    format!("\"{}\"", value.to_string())
}

pub(crate) fn parse_yes_or_no<T: ToString>(s: T) -> crate::Result<bool> {
    match s.to_string().as_str() {
        "YES" => Ok(true),
        "NO" => Ok(false),
        _ => track_panic!(
            ErrorKind::InvalidInput,
            "Unexpected value: {:?}",
            s.to_string()
        ),
    }
}

pub(crate) fn parse_u64<T: ToString>(s: T) -> crate::Result<u64> {
    let n = track!(s
        .to_string()
        .parse()
        .map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
    Ok(n)
}
