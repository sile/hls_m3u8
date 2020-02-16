//! Credits go to
//! - https://github.com/globocom/m3u8/blob/master/tests/playlists.py
use std::time::Duration;

use hls_m3u8::tags::{ExtInf, ExtXEndList};
use hls_m3u8::{MediaPlaylist, MediaSegment};
use pretty_assertions::assert_eq;

#[test]
fn test_simple_playlist() {
    let playlist = concat!(
        "#EXTM3U\n",
        "#EXT-X-TARGETDURATION:5220\n",
        "#EXTINF:0,\n",
        "http://media.example.com/entire1.ts\n",
        "#EXTINF:5220,\n",
        "http://media.example.com/entire2.ts\n",
        "#EXT-X-ENDLIST\n"
    );

    assert_eq!(
        MediaPlaylist::builder()
            .target_duration(Duration::from_secs(5220))
            .segments(vec![
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(0)))
                    .uri("http://media.example.com/entire1.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs(5220)))
                    .uri("http://media.example.com/entire2.ts")
                    .build()
                    .unwrap(),
            ])
            .end_list(ExtXEndList)
            .build()
            .unwrap(),
        playlist.parse::<MediaPlaylist>().unwrap(),
    );
}
