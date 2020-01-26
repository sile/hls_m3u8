use core::iter::FusedIterator;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct AttributePairs<'a> {
    string: &'a str,
    index: usize,
}

impl<'a> AttributePairs<'a> {
    pub const fn new(string: &'a str) -> Self { Self { string, index: 0 } }
}

impl<'a> Iterator for AttributePairs<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        // return `None`, if there are no more chars
        self.string.as_bytes().get(self.index + 1)?;

        let key = {
            // the position in the string:
            let start = self.index;
            // the key ends at an `=`:
            let end = self
                .string
                .bytes()
                .skip(self.index)
                .position(|i| i == b'=')?
                + start;

            // advance the index to the 2nd char after the end of the key
            // (this will skip the `=`)
            self.index = end + 1;

            core::str::from_utf8(&self.string.as_bytes()[start..end]).unwrap()
        };

        let value = {
            let start = self.index;
            let mut end = 0;

            // find the end of the value by searching for `,`.
            // it should ignore `,` that are inside double quotes.
            let mut inside_quotes = false;
            while let Some(item) = self.string.as_bytes().get(start + end) {
                end += 1;

                if *item == b'"' {
                    inside_quotes = !inside_quotes;
                } else if *item == b',' && !inside_quotes {
                    self.index += 1;
                    end -= 1;
                    break;
                }
            }

            self.index += end;
            end += start;

            core::str::from_utf8(&self.string.as_bytes()[start..end]).unwrap()
        };

        Some((key.trim(), value.trim()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut remaining = 0;

        // each `=` in the remaining str is an iteration
        // this also ignores `=` inside quotes!
        let mut inside_quotes = false;
        for c in self.string.as_bytes().iter().skip(self.index) {
            if *c == b'=' && !inside_quotes {
                remaining += 1;
            } else if *c == b'"' {
                inside_quotes = !inside_quotes;
            }
        }

        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for AttributePairs<'a> {}
impl<'a> FusedIterator for AttributePairs<'a> {}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_attributes() {
        let mut attributes = AttributePairs::new("KEY=VALUE,PAIR=YES");
        assert_eq!((2, Some(2)), attributes.size_hint());
        assert_eq!(Some(("KEY", "VALUE")), attributes.next());
        assert_eq!((1, Some(1)), attributes.size_hint());
        assert_eq!(Some(("PAIR", "YES")), attributes.next());
        assert_eq!((0, Some(0)), attributes.size_hint());
        assert_eq!(None, attributes.next());

        let mut attributes = AttributePairs::new("garbage");
        assert_eq!((0, Some(0)), attributes.size_hint());
        assert_eq!(None, attributes.next());

        let mut attributes = AttributePairs::new("KEY=,=VALUE,=,");
        assert_eq!((3, Some(3)), attributes.size_hint());
        assert_eq!(Some(("KEY", "")), attributes.next());
        assert_eq!((2, Some(2)), attributes.size_hint());
        assert_eq!(Some(("", "VALUE")), attributes.next());
        assert_eq!((1, Some(1)), attributes.size_hint());
        assert_eq!(Some(("", "")), attributes.next());
        assert_eq!((0, Some(0)), attributes.size_hint());
        assert_eq!(None, attributes.next());

        // test quotes:
        let mut attributes = AttributePairs::new("KEY=\"VALUE,\",");
        assert_eq!((1, Some(1)), attributes.size_hint());
        assert_eq!(Some(("KEY", "\"VALUE,\"")), attributes.next());
        assert_eq!((0, Some(0)), attributes.size_hint());
        assert_eq!(None, attributes.next());

        // test with chars, that are larger, than 1 byte
        let mut attributes = AttributePairs::new(
            "LANGUAGE=\"fre\",\
             NAME=\"Français\",\
             AUTOSELECT=YES",
        );

        assert_eq!(Some(("LANGUAGE", "\"fre\"")), attributes.next());
        assert_eq!(Some(("NAME", "\"Français\"")), attributes.next());
        assert_eq!(Some(("AUTOSELECT", "YES")), attributes.next());
    }

    #[test]
    fn test_parser() {
        let mut pairs = AttributePairs::new("FOO=BAR,BAR=\"baz,qux\",ABC=12.3");

        assert_eq!(pairs.next(), Some(("FOO", "BAR")));
        assert_eq!(pairs.next(), Some(("BAR", "\"baz,qux\"")));
        assert_eq!(pairs.next(), Some(("ABC", "12.3")));
        assert_eq!(pairs.next(), None);
    }
}
