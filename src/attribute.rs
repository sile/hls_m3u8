use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use crate::Error;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AttributePairs(HashMap<String, String>);

impl AttributePairs {
    pub fn new() -> Self { Self::default() }
}

impl Deref for AttributePairs {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for AttributePairs {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl IntoIterator for AttributePairs {
    type IntoIter = ::std::collections::hash_map::IntoIter<String, String>;
    type Item = (String, String);

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a> IntoIterator for &'a AttributePairs {
    type IntoIter = ::std::collections::hash_map::Iter<'a, String, String>;
    type Item = (&'a String, &'a String);

    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

impl FromStr for AttributePairs {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut result = Self::new();

        for line in split(input, ',') {
            let pair = split(line.trim(), '=');

            if pair.len() < 2 {
                continue;
            }

            let key = pair[0].trim().to_uppercase();
            let value = pair[1].trim().to_string();
            if value.is_empty() {
                continue;
            }

            result.insert(key.trim().to_string(), value.trim().to_string());
        }

        #[cfg(test)] // this is very useful, when a test fails!
        dbg!(&result);
        Ok(result)
    }
}

fn split(value: &str, terminator: char) -> Vec<String> {
    let mut result = vec![];

    let mut inside_quotes = false;
    let mut temp_string = String::with_capacity(1024);

    for c in value.chars() {
        match c {
            '"' => {
                inside_quotes = !inside_quotes;
                temp_string.push(c);
            }
            k if (k == terminator) => {
                if inside_quotes {
                    temp_string.push(c);
                } else {
                    result.push(temp_string);
                    temp_string = String::with_capacity(1024);
                }
            }
            _ => {
                temp_string.push(c);
            }
        }
    }
    result.push(temp_string);

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let pairs = "FOO=BAR,BAR=\"baz,qux\",ABC=12.3"
            .parse::<AttributePairs>()
            .unwrap();

        let mut iterator = pairs.iter();
        assert!(iterator.any(|(k, v)| k == "FOO" && "BAR" == v));

        let mut iterator = pairs.iter();
        assert!(iterator.any(|(k, v)| k == "BAR" && v == "\"baz,qux\""));

        let mut iterator = pairs.iter();
        assert!(iterator.any(|(k, v)| k == "ABC" && v == "12.3"));

        let mut pairs = AttributePairs::new();
        pairs.insert("FOO".to_string(), "BAR".to_string());

        assert_eq!("FOO=BAR,VAL".parse::<AttributePairs>().unwrap(), pairs);
    }

    #[test]
    fn test_iterator() {
        let mut attrs = AttributePairs::new();
        attrs.insert("key_01".to_string(), "value_01".to_string());
        attrs.insert("key_02".to_string(), "value_02".to_string());

        let mut iterator = attrs.iter();
        assert!(iterator.any(|(k, v)| k == "key_01" && v == "value_01"));

        let mut iterator = attrs.iter();
        assert!(iterator.any(|(k, v)| k == "key_02" && v == "value_02"));
    }

    #[test]
    fn test_into_iter() {
        let mut map = HashMap::new();
        map.insert("k".to_string(), "v".to_string());

        let mut attrs = AttributePairs::new();
        attrs.insert("k".to_string(), "v".to_string());

        assert_eq!(
            attrs.into_iter().collect::<Vec<_>>(),
            map.into_iter().collect::<Vec<_>>()
        );
    }
}
