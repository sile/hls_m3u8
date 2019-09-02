use hls_m3u8::MediaPlaylist;
// https://developer.apple.com/documentation/http_live_streaming/example_playlists_for_http_live_streaming
#[test]
fn playlist_1() {
    let playlist_1 = r#"
    #EXTM3U
    #EXT-X-PLAYLIST-TYPE:VOD
    #EXT-X-TARGETDURATION:10
    #EXT-X-VERSION:4
    #EXT-X-MEDIA-SEQUENCE:0
    #EXTINF:10.0,
    http://example.com/movie1/fileSequenceA.ts
    #EXTINF:10.0,
    http://example.com/movie1/fileSequenceB.ts
    #EXTINF:10.0,
    http://example.com/movie1/fileSequenceC.ts
    #EXTINF:9.0,
    http://example.com/movie1/fileSequenceD.ts
    #EXT-X-ENDLIST
    "#;

    //dbg!(playlist_1.parse::<MediaPlaylist>());
    assert!(playlist_1.parse::<MediaPlaylist>().is_ok());
}

#[test]
fn playlist_2() {
    let playlist_2 = r#"
    #EXTM3U
    #EXT-X-PLAYLIST-TYPE:VOD
    #EXT-X-TARGETDURATION:10
    #EXT-X-VERSION:4
    #EXT-X-MEDIA-SEQUENCE:0
    #EXTINF:10.0,
    fileSequenceA.ts
    #EXTINF:10.0,
    fileSequenceB.ts
    #EXTINF:10.0,
    fileSequenceC.ts
    #EXTINF:9.0,
    fileSequenceD.ts
    #EXT-X-ENDLIST
    "#;

    assert!(playlist_2.parse::<MediaPlaylist>().is_ok());
}

/*
Error(
    TrackableError {
        kind: InvalidInput,
        cause: Some(Cause("assertion failed: `self.inf_tag.is_some()`")),
        history: History(
            [
                Location {
                    module_path: "hls_m3u8::media_segment",
                    file: "src/media_segment.rs", line: 62,
                    message: ""
                },
                Location {
                    module_path: "hls_m3u8::media_playlist",
                    file: "src/media_playlist.rs",
                    line: 444, message: ""
                },
                Location {
                    module_path: "hls_m3u8::media_playlist",
                    file: "src/media_playlist.rs",
                    line: 292,
                    message: ""
                }
            ]
        )
    }
)

*/
