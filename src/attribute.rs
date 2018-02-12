use std::fmt;
use std::str::{self, FromStr};
use std::time::Duration;
use std::u8;
use trackable::error::ErrorKindExt;

use {Error, ErrorKind, Result};

#[derive(Debug)]
pub struct AttributePairs<'a> {
    input: &'a str,
}
impl<'a> AttributePairs<'a> {
    pub fn parse(input: &'a str) -> Self {
        AttributePairs { input }
    }

    fn parse_name(&mut self) -> Result<&'a str> {
        for i in 0..self.input.len() {
            match self.input.as_bytes()[i] {
                b'=' => {
                    let (key, _) = self.input.split_at(i);
                    let (_, rest) = self.input.split_at(i + 1);
                    self.input = rest;
                    return Ok(key);
                }
                b'A'...b'Z' | b'0'...b'9' | b'-' => {}
                _ => track_panic!(
                    ErrorKind::InvalidInput,
                    "Malformed attribute name: {:?}",
                    self.input
                ),
            }
        }
        track_panic!(
            ErrorKind::InvalidInput,
            "No attribute value: {:?}",
            self.input
        );
    }

    fn parse_raw_value(&mut self) -> &'a str {
        let (value_end, next) = if let Some(i) = self.input.bytes().position(|c| c == b',') {
            (i, i + 1)
        } else {
            (self.input.len(), self.input.len())
        };
        let (value, _) = self.input.split_at(value_end);
        let (_, rest) = self.input.split_at(next);
        self.input = rest;
        value
    }
}
impl<'a> Iterator for AttributePairs<'a> {
    type Item = Result<(&'a str, &'a str)>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        let result = || -> Result<(&'a str, &'a str)> {
            // TODO: check key duplications
            let key = track!(self.parse_name())?;
            let value = self.parse_raw_value();
            Ok((key, value))
        }();
        Some(result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuotedString(String);
impl fmt::Display for QuotedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl FromStr for QuotedString {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let len = s.len();
        let bytes = s.as_bytes();
        track_assert!(len >= 2, ErrorKind::InvalidInput);
        track_assert_eq!(bytes[0], b'"', ErrorKind::InvalidInput);
        track_assert_eq!(bytes[len - 1], b'"', ErrorKind::InvalidInput);

        let s = unsafe { str::from_utf8_unchecked(&bytes[1..len - 1]) };
        track_assert!(
            s.chars().all(|c| c != '\r' && c != '\n' && c != '"'),
            ErrorKind::InvalidInput,
            "{:?}",
            s
        );
        Ok(QuotedString(s.to_owned()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HexadecimalSequence(Vec<u8>);
impl fmt::Display for HexadecimalSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x")?;
        for b in &self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}
impl FromStr for HexadecimalSequence {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(
            s.starts_with("0x") || s.starts_with("0X"),
            ErrorKind::InvalidInput
        );
        track_assert!(s.len() % 2 == 0, ErrorKind::InvalidInput);

        let mut v = Vec::with_capacity(s.len() / 2 - 1);
        for c in s.as_bytes().chunks(2).skip(1) {
            let d = track!(str::from_utf8(c).map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
            let b =
                track!(u8::from_str_radix(d, 16).map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
            v.push(b);
        }
        Ok(HexadecimalSequence(v))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecimalInteger(u64);
impl fmt::Display for DecimalInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FromStr for DecimalInteger {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let n = track!(s.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
        Ok(DecimalInteger(n))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DecimalFloatingPoint(f64);
impl DecimalFloatingPoint {
    pub fn to_duration(&self) -> Duration {
        let secs = self.0 as u64;
        let nanos = (self.0.fract() * 1_000_000_000.0) as u32;
        Duration::new(secs, nanos)
    }
    pub fn from_duration(duration: Duration) -> Self {
        let n = (duration.as_secs() as f64) + (duration.subsec_nanos() as f64 / 1_000_000_000.0);
        DecimalFloatingPoint(n)
    }
}
impl Eq for DecimalFloatingPoint {}
impl fmt::Display for DecimalFloatingPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl FromStr for DecimalFloatingPoint {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(
            s.chars().all(|c| match c {
                '0'...'9' | '.' => true,
                _ => false,
            }),
            ErrorKind::InvalidInput
        );
        let n = track!(s.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
        Ok(DecimalFloatingPoint(n))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SignedDecimalFloatingPoint(f64);
impl Eq for SignedDecimalFloatingPoint {}
impl fmt::Display for SignedDecimalFloatingPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl FromStr for SignedDecimalFloatingPoint {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        track_assert!(
            s.chars().all(|c| match c {
                '0'...'9' | '.' | '-' => true,
                _ => false,
            }),
            ErrorKind::InvalidInput
        );
        let n = track!(s.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?;
        Ok(SignedDecimalFloatingPoint(n))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimalResolution {
    pub width: usize,
    pub height: usize,
}
impl fmt::Display for DecimalResolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}
impl FromStr for DecimalResolution {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut tokens = s.splitn(2, 'x');
        let width = tokens.next().expect("Never fails");
        let height = track_assert_some!(tokens.next(), ErrorKind::InvalidInput);
        Ok(DecimalResolution {
            width: track!(width.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?,
            height: track!(height.parse().map_err(|e| ErrorKind::InvalidInput.cause(e)))?,
        })
    }
}
