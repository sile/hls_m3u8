// The relevant issue:
// https://github.com/sile/hls_m3u8/issues/59
use std::convert::TryFrom;

use hls_m3u8::MediaPlaylist;

use pretty_assertions::assert_eq;

#[test]
fn parse() {
    let playlist = concat!(
        "#EXTM3U\n",
        "#EXT-X-DISCONTINUITY-SEQUENCE:1\n",
        "#EXT-X-TARGETDURATION:10\n",
        "#EXT-X-VERSION:3\n",
        "#EXTINF:9.009,\n",
        "http://media.example.com/first.ts\n",
        "#EXTINF:9.009,\n",
        "http://media.example.com/second.ts\n",
        "#EXTINF:3.003,\n",
        "http://media.example.com/third.ts\n",
        "#EXT-X-ENDLIST"
    );

    let playlist = MediaPlaylist::try_from(playlist).unwrap();
    assert_eq!(playlist.discontinuity_sequence, 1);
}
