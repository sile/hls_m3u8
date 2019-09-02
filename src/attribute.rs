use std::collections::HashMap;
use std::str::FromStr;

use shrinkwraprs::Shrinkwrap;

use crate::error::{Error, ErrorKind};

#[derive(Shrinkwrap, Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct AttributePairs(HashMap<String, String>);

impl IntoIterator for AttributePairs {
    type Item = (String, String);
    type IntoIter = ::std::collections::hash_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[allow(dead_code)]
impl AttributePairs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<String>
    where
        K: ToString,
        V: ToString,
    {
        self.0.insert(key.to_string(), value.to_string())
    }
}

impl FromStr for AttributePairs {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut result = HashMap::new();

        for line in split(value) {
            let t = line
                .trim()
                .split("=")
                .map(|x| x.to_string())
                .collect::<Vec<_>>();

            let (key, value) = {
                if t.len() != 2 {
                    Err(ErrorKind::InvalidInput)?
                } else {
                    (t[0].clone(), t[1].clone())
                }
            };

            result.insert(key.to_string(), value.to_string());
        }

        Ok(Self(result))
    }
}

pub fn split(value: &str) -> Vec<String> {
    let mut result = vec![];

    let mut inside_quotes = false;
    let mut temp_string = String::new();

    for c in value.chars() {
        match c {
            '"' => {
                if inside_quotes {
                    inside_quotes = false;
                } else {
                    inside_quotes = true;
                }
                temp_string.push(c);
            }
            ',' => {
                if !inside_quotes {
                    result.push(temp_string);
                    temp_string = String::new();
                } else {
                    temp_string.push(c);
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
        let pairs = ("FOO=BAR,BAR=\"baz,qux\",ABC=12.3")
            .parse::<AttributePairs>()
            .unwrap();

        let mut iterator = pairs.iter();
        assert!(iterator.any(|(k, v)| (k, v) == (&"FOO".to_string(), &"BAR".to_string())));

        let mut iterator = pairs.iter();
        assert!(iterator.any(|(k, v)| (k, v) == (&"BAR".to_string(), &"\"baz,qux\"".to_string())));

        let mut iterator = pairs.iter();
        assert!(iterator.any(|(k, v)| (k, v) == (&"ABC".to_string(), &"12.3".to_string())));
    }

    #[test]
    fn test_malformed_input() {
        let result = ("FOO=,Bar==,,=12,ABC=12").parse::<AttributePairs>();
        assert!(result.is_err());
    }

    #[test]
    fn test_iterator() {
        let mut attrs = AttributePairs::new();
        attrs.insert("key_01".to_string(), "value_01".to_string());
        attrs.insert("key_02".to_string(), "value_02".to_string());

        let mut iterator = attrs.iter();
        assert!(iterator.any(|(k, v)| (k, v) == (&"key_01".to_string(), &"value_01".to_string())));

        let mut iterator = attrs.iter();
        assert!(iterator.any(|(k, v)| (k, v) == (&"key_02".to_string(), &"value_02".to_string())));
    }
}
