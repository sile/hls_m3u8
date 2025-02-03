use core::iter::FusedIterator;

#[derive(Clone, Debug)]
pub(crate) struct AttributePairs<'a> {
    string: &'a str,
    index: usize,
}

impl<'a> AttributePairs<'a> {
    pub const fn new(string: &'a str) -> Self {
        Self { string, index: 0 }
    }
}

impl<'a> Iterator for AttributePairs<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        // return `None`, if there are no more bytes
        self.string.as_bytes().get(self.index + 1)?;

        let key = {
            // the position in the string:
            let start = self.index;
            // the key ends at an `=`:
            let end = self.string[self.index..]
                .char_indices()
                .find_map(|(i, c)| if c == '=' { Some(i) } else { None })?
                + self.index;

            // advance the index to the char after the end of the key (to skip the `=`)
            // NOTE: it is okay to add 1 to the index, because an `=` is exactly 1 byte.
            self.index = end + 1;

            // NOTE: See https://github.com/sile/hls_m3u8/issues/64
            self.string[start..end].trim()
        };

        let value = {
            let start = self.index;

            // find the end of the value by searching for `,`.
            // it should ignore `,` that are inside double quotes.
            let mut inside_quotes = false;

            let end = {
                let mut result = self.string.len();

                for (i, c) in self.string[self.index..].char_indices() {
                    // if a quote is encountered
                    if c == '"' {
                        // update variable
                        inside_quotes = !inside_quotes;
                    // terminate if a comma is encountered, which is not in a
                    // quote
                    } else if c == ',' && !inside_quotes {
                        // move the index past the comma
                        self.index += 1;
                        // the result is the index of the comma (comma is not included in the
                        // resulting string)
                        result = i + self.index - 1;
                        break;
                    }
                }

                result
            };

            self.index += end;
            self.index -= start;

            // NOTE: See https://github.com/sile/hls_m3u8/issues/64
            self.string[start..end].trim()
        };

        Some((key, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut remaining = 0;

        // each `=` in the remaining str is an iteration
        // this also ignores `=` inside quotes!
        let mut inside_quotes = false;

        for (_, c) in self.string[self.index..].char_indices() {
            if c == '=' && !inside_quotes {
                remaining += 1;
            } else if c == '"' {
                inside_quotes = !inside_quotes;
            }
        }

        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for AttributePairs<'_> {}
impl FusedIterator for AttributePairs<'_> {}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_attributes() {
        let mut attributes = AttributePairs::new("KEY=VALUE,PAIR=YES");

        assert_eq!((2, Some(2)), attributes.size_hint());
        assert_eq!(attributes.next(), Some(("KEY", "VALUE")));

        assert_eq!((1, Some(1)), attributes.size_hint());
        assert_eq!(attributes.next(), Some(("PAIR", "YES")));

        assert_eq!((0, Some(0)), attributes.size_hint());
        assert_eq!(attributes.next(), None);

        let mut attributes = AttributePairs::new("garbage");
        assert_eq!((0, Some(0)), attributes.size_hint());
        assert_eq!(attributes.next(), None);

        let mut attributes = AttributePairs::new("KEY=,=VALUE,=,");

        assert_eq!((3, Some(3)), attributes.size_hint());
        assert_eq!(attributes.next(), Some(("KEY", "")));

        assert_eq!((2, Some(2)), attributes.size_hint());
        assert_eq!(attributes.next(), Some(("", "VALUE")));

        assert_eq!((1, Some(1)), attributes.size_hint());
        assert_eq!(attributes.next(), Some(("", "")));

        assert_eq!((0, Some(0)), attributes.size_hint());
        assert_eq!(attributes.next(), None);

        // test quotes:
        let mut attributes = AttributePairs::new("KEY=\"VALUE,\",");

        assert_eq!((1, Some(1)), attributes.size_hint());
        assert_eq!(attributes.next(), Some(("KEY", "\"VALUE,\"")));

        assert_eq!((0, Some(0)), attributes.size_hint());
        assert_eq!(attributes.next(), None);

        // test with chars, that are larger, than 1 byte
        let mut attributes = AttributePairs::new(concat!(
            "LANGUAGE=\"fre\",",
            "NAME=\"Français\",",
            "AUTOSELECT=YES"
        ));

        assert_eq!(attributes.next(), Some(("LANGUAGE", "\"fre\"")));
        assert_eq!(attributes.next(), Some(("NAME", "\"Français\"")));
        assert_eq!(attributes.next(), Some(("AUTOSELECT", "YES")));
    }

    #[test]
    fn test_parser() {
        let mut pairs = AttributePairs::new("FOO=BAR,BAR=\"baz,qux\",ABC=12.3");

        assert_eq!(pairs.next(), Some(("FOO", "BAR")));
        assert_eq!(pairs.next(), Some(("BAR", "\"baz,qux\"")));
        assert_eq!(pairs.next(), Some(("ABC", "12.3")));
        assert_eq!(pairs.next(), None);

        // stress test with foreign input
        // got it from https://generator.lorem-ipsum.info/_chinese

        let mut pairs = AttributePairs::new(concat!(
            "載抗留囲軽来実基供全必式覧領意度振。=著地内方満職控努作期投綱研本模,",
            "後文図様改表宮能本園半参裁報作神掲索=\"針支年得率新賞現報発援白少動面。矢拉年世掲注索政平定他込\",",
            "ध्वनि स्थिति और्४५० नीचे =देखने लाभो द्वारा करके(विशेष"
        ));

        assert_eq!((3, Some(3)), pairs.size_hint());
        assert_eq!(
            pairs.next(),
            Some((
                "載抗留囲軽来実基供全必式覧領意度振。",
                "著地内方満職控努作期投綱研本模"
            ))
        );

        assert_eq!((2, Some(2)), pairs.size_hint());
        assert_eq!(
            pairs.next(),
            Some((
                "後文図様改表宮能本園半参裁報作神掲索",
                "\"針支年得率新賞現報発援白少動面。矢拉年世掲注索政平定他込\""
            ))
        );

        assert_eq!((1, Some(1)), pairs.size_hint());
        assert_eq!(
            pairs.next(),
            Some(("ध्वनि स्थिति और्४५० नीचे", "देखने लाभो द्वारा करके(विशेष"))
        );

        assert_eq!((0, Some(0)), pairs.size_hint());
        assert_eq!(pairs.next(), None);
    }
}
