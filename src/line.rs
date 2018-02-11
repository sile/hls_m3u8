use {ErrorKind, Result};
use tag::Tag;

// [rfc8216#section-4.1]
// > Playlist files MUST be encoded in UTF-8 [RFC3629].  They MUST NOT
// > contain any Byte Order Mark (BOM); clients SHOULD fail to parse
// > Playlists that contain a BOM or do not parse as UTF-8.  Playlist
// > files MUST NOT contain UTF-8 control characters (U+0000 to U+001F and
// > U+007F to U+009F), with the exceptions of CR (U+000D) and LF
// > (U+000A).  All character sequences MUST be normalized according to
// > Unicode normalization form "NFC" [UNICODE].  Note that US-ASCII
// > [US_ASCII] conforms to these rules.
// >
// > Lines in a Playlist file are terminated by either a single line feed
// > character or a carriage return character followed by a line feed
// > character.
#[derive(Debug)]
pub struct Lines<'a> {
    input: &'a str,
}
impl<'a> Lines<'a> {
    pub fn new(input: &'a str) -> Self {
        Lines { input }
    }

    fn read_line(&mut self) -> Result<Line<'a>> {
        let mut end = self.input.len();
        let mut next_start = self.input.len();
        let mut adjust = 0;
        for (i, c) in self.input.char_indices() {
            match c {
                '\n' => {
                    next_start = i + 1;
                    end = i - adjust;
                    break;
                }
                '\r' => {
                    adjust = 1;
                }
                '\u{00}'...'\u{1F}' | '\u{7F}'...'\u{9f}' => {
                    track_panic!(ErrorKind::InvalidInput);
                }
                _ => {
                    adjust = 0;
                }
            }
        }
        let raw_line = &self.input[..end];
        let line = if raw_line.is_empty() {
            Line::Blank
        } else if raw_line.starts_with("#EXT") {
            Line::Tag(track!(raw_line.parse())?)
        } else if raw_line.starts_with("#") {
            Line::Comment(raw_line)
        } else {
            Line::Uri(raw_line)
        };
        self.input = &self.input[next_start..];
        Ok(line)
    }
}
impl<'a> Iterator for Lines<'a> {
    type Item = Result<Line<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }
        match track!(self.read_line()) {
            Err(e) => Some(Err(e)),
            Ok(line) => Some(Ok(line)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Line<'a> {
    Blank,
    Comment(&'a str),
    Tag(Tag),

    // TODO:
    Uri(&'a str),
}

// TODO
// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let mut lines = Lines::new("foo\nbar\r\nbaz");
//         assert_eq!(lines.next().and_then(|x| x.ok()), Some("foo"));
//         assert_eq!(lines.next().and_then(|x| x.ok()), Some("bar"));
//         assert_eq!(lines.next().and_then(|x| x.ok()), Some("baz"));
//         assert_eq!(lines.next().and_then(|x| x.ok()), None);
//     }
// }
