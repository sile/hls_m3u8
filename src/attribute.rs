use crate::{ErrorKind, Result};
use std::collections::HashSet;
use std::str;

#[derive(Debug)]
pub struct AttributePairs<'a> {
    input: &'a str,
    visited_keys: HashSet<&'a str>,
}
impl<'a> AttributePairs<'a> {
    pub fn parse(input: &'a str) -> Self {
        AttributePairs {
            input,
            visited_keys: HashSet::new(),
        }
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
                b'A'..=b'Z' | b'0'..=b'9' | b'-' => {}
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
        let mut in_quote = false;
        let mut value_end = self.input.len();
        let mut next = self.input.len();
        for (i, c) in self.input.bytes().enumerate() {
            match c {
                b'"' => {
                    in_quote = !in_quote;
                }
                b',' if !in_quote => {
                    value_end = i;
                    next = i + 1;
                    break;
                }
                _ => {}
            }
        }
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
            track_assert!(
                self.visited_keys.insert(key),
                ErrorKind::InvalidInput,
                "Duplicate attribute key: {:?}",
                key
            );

            let value = self.parse_raw_value();
            Ok((key, value))
        }();
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut pairs = AttributePairs::parse("FOO=BAR,BAR=\"baz,qux\",ABC=12.3");
        assert_eq!(pairs.next().map(|x| x.ok()), Some(Some(("FOO", "BAR"))));
        assert_eq!(
            pairs.next().map(|x| x.ok()),
            Some(Some(("BAR", "\"baz,qux\"")))
        );
        assert_eq!(pairs.next().map(|x| x.ok()), Some(Some(("ABC", "12.3"))));
        assert_eq!(pairs.next().map(|x| x.ok()), None)
    }
}
