//! Credits go to
//! - https://github.com/globocom/m3u8/blob/master/tests/playlists.py
use hls_m3u8::tags::*;
use hls_m3u8::MediaPlaylist;

use std::time::Duration;

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

    let media_playlist = playlist.parse::<MediaPlaylist>().unwrap();
    assert_eq!(
        media_playlist.target_duration(),
        ExtXTargetDuration::new(Duration::from_secs(5220))
    );

    assert_eq!(media_playlist.segments().len(), 2);

    assert_eq!(
        media_playlist.segments()[0].inf_tag(),
        &ExtInf::new(Duration::from_secs(0))
    );

    assert_eq!(
        media_playlist.segments()[1].inf_tag(),
        &ExtInf::new(Duration::from_secs(5220))
    );

    assert_eq!(
        media_playlist.segments()[0].uri(),
        &"http://media.example.com/entire1.ts".to_string()
    );

    assert_eq!(
        media_playlist.segments()[1].uri(),
        &"http://media.example.com/entire2.ts".to_string()
    );
}
