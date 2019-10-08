use hls_m3u8::tags::{ExtXIFrameStreamInf, ExtXStreamInf};
use hls_m3u8::MasterPlaylist;

use pretty_assertions::assert_eq;

#[test]
fn test_master_playlist() {
    let master_playlist = "#EXTM3U\n\
                           #EXT-X-STREAM-INF:BANDWIDTH=1280000,AVERAGE-BANDWIDTH=1000000\n\
                           http://example.com/low.m3u8\n\
                           #EXT-X-STREAM-INF:BANDWIDTH=2560000,AVERAGE-BANDWIDTH=2000000\n\
                           http://example.com/mid.m3u8\n\
                           #EXT-X-STREAM-INF:BANDWIDTH=7680000,AVERAGE-BANDWIDTH=6000000\n\
                           http://example.com/hi.m3u8\n\
                           #EXT-X-STREAM-INF:BANDWIDTH=65000,CODECS=\"mp4a.40.5\"\n\
                           http://example.com/audio-only.m3u8"
        .parse::<MasterPlaylist>()
        .unwrap();

    assert_eq!(
        MasterPlaylist::builder()
            .stream_inf_tags(vec![
                ExtXStreamInf::builder()
                    .bandwidth(1280000)
                    .average_bandwidth(1000000)
                    .uri("http://example.com/low.m3u8")
                    .build()
                    .unwrap(),
                ExtXStreamInf::builder()
                    .bandwidth(2560000)
                    .average_bandwidth(2000000)
                    .uri("http://example.com/mid.m3u8")
                    .build()
                    .unwrap(),
                ExtXStreamInf::builder()
                    .bandwidth(7680000)
                    .average_bandwidth(6000000)
                    .uri("http://example.com/hi.m3u8")
                    .build()
                    .unwrap(),
                ExtXStreamInf::builder()
                    .bandwidth(65000)
                    .codecs("mp4a.40.5")
                    .uri("http://example.com/audio-only.m3u8")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
        master_playlist
    );
}

#[test]
fn test_master_playlist_with_i_frames() {
    let master_playlist = "#EXTM3U\n\
                           #EXT-X-STREAM-INF:BANDWIDTH=1280000\n\
                           low/audio-video.m3u8\n\
                           #EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=86000,URI=\"low/iframe.m3u8\"\n\
                           #EXT-X-STREAM-INF:BANDWIDTH=2560000\n\
                           mid/audio-video.m3u8\n\
                           #EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=150000,URI=\"mid/iframe.m3u8\"\n\
                           #EXT-X-STREAM-INF:BANDWIDTH=7680000\n\
                           hi/audio-video.m3u8\n\
                           #EXT-X-I-FRAME-STREAM-INF:BANDWIDTH=550000,URI=\"hi/iframe.m3u8\"\n\
                           #EXT-X-STREAM-INF:BANDWIDTH=65000,CODECS=\"mp4a.40.5\"\n\
                           audio-only.m3u8"
        .parse::<MasterPlaylist>()
        .unwrap();

    assert_eq!(
        MasterPlaylist::builder()
            .stream_inf_tags(vec![
                ExtXStreamInf::builder()
                    .bandwidth(1280000)
                    .uri("low/audio-video.m3u8")
                    .build()
                    .unwrap(),
                ExtXStreamInf::builder()
                    .bandwidth(2560000)
                    .uri("mid/audio-video.m3u8")
                    .build()
                    .unwrap(),
                ExtXStreamInf::builder()
                    .bandwidth(7680000)
                    .uri("hi/audio-video.m3u8")
                    .build()
                    .unwrap(),
                ExtXStreamInf::builder()
                    .bandwidth(65000)
                    .codecs("mp4a.40.5")
                    .uri("audio-only.m3u8")
                    .build()
                    .unwrap(),
            ])
            .i_frame_stream_inf_tags(vec![
                ExtXIFrameStreamInf::builder()
                    .bandwidth(86000)
                    .uri("low/iframe.m3u8")
                    .build()
                    .unwrap(),
                ExtXIFrameStreamInf::builder()
                    .bandwidth(150000)
                    .uri("mid/iframe.m3u8")
                    .build()
                    .unwrap(),
                ExtXIFrameStreamInf::builder()
                    .bandwidth(550000)
                    .uri("hi/iframe.m3u8")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
        master_playlist
    );
}
