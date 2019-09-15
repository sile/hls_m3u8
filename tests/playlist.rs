//! Credits go to
//! - https://github.com/globocom/m3u8/blob/master/tests/playlists.py
use hls_m3u8::tags::*;
use hls_m3u8::types::*;
use hls_m3u8::MediaPlaylist;

use std::time::Duration;

#[test]
fn test_simple_playlist() {
    let playlist = r#"
    #EXTM3U
    #EXT-X-TARGETDURATION:5220
    #EXTINF:0,
    http://media.example.com/entire1.ts
    #EXTINF:5220,
    http://media.example.com/entire2.ts
    #EXT-X-ENDLIST"#;

    let media_playlist = playlist.parse::<MediaPlaylist>().unwrap();
    assert_eq!(
        media_playlist.target_duration_tag(),
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
        &SingleLineString::new("http://media.example.com/entire1.ts").unwrap()
    );

    assert_eq!(
        media_playlist.segments()[1].uri(),
        &SingleLineString::new("http://media.example.com/entire2.ts").unwrap()
    );
}
