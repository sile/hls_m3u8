use std::fmt;
use std::str::{self, FromStr};

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
