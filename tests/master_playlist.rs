use hls_m3u8::tags::{ExtXIFrameStreamInf, ExtXMedia, ExtXStreamInf};
use hls_m3u8::types::MediaType;
use hls_m3u8::MasterPlaylist;

use pretty_assertions::assert_eq;

#[test]
fn test_master_playlist() {
    // https://tools.ietf.org/html/rfc8216#section-8.4
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
    // https://tools.ietf.org/html/rfc8216#section-8.5
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

#[test]
fn test_master_playlist_with_alternative_audio() {
    // https://tools.ietf.org/html/rfc8216#section-8.6
    // TODO: I think the CODECS=\"..." have to be replaced.
    let master_playlist = "#EXTM3U\n\
                           #EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"aac\",NAME=\"English\", \
                           DEFAULT=YES,AUTOSELECT=YES,LANGUAGE=\"en\", \
                           URI=\"main/english-audio.m3u8\"\n\

                           #EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"aac\",NAME=\"Deutsch\", \
                           DEFAULT=NO,AUTOSELECT=YES,LANGUAGE=\"de\", \
                           URI=\"main/german-audio.m3u8\"\n\

                           #EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"aac\",NAME=\"Commentary\", \
                           DEFAULT=NO,AUTOSELECT=NO,LANGUAGE=\"en\", \
                           URI=\"commentary/audio-only.m3u8\"\n\

                           #EXT-X-STREAM-INF:BANDWIDTH=1280000,CODECS=\"...\",AUDIO=\"aac\"\n\
                           low/video-only.m3u8\n\
                           #EXT-X-STREAM-INF:BANDWIDTH=2560000,CODECS=\"...\",AUDIO=\"aac\"\n\
                           mid/video-only.m3u8\n\
                           #EXT-X-STREAM-INF:BANDWIDTH=7680000,CODECS=\"...\",AUDIO=\"aac\"\n\
                           hi/video-only.m3u8\n\
                           #EXT-X-STREAM-INF:BANDWIDTH=65000,CODECS=\"mp4a.40.5\",AUDIO=\"aac\"\n\
                           main/english-audio.m3u8"
        .parse::<MasterPlaylist>()
        .unwrap();

    assert_eq!(
        MasterPlaylist::builder()
            .media_tags(vec![
                ExtXMedia::builder()
                    .media_type(MediaType::Audio)
                    .group_id("aac")
                    .name("English")
                    .is_default(true)
                    .is_autoselect(true)
                    .language("en")
                    .uri("main/english-audio.m3u8")
                    .build()
                    .unwrap(),
                ExtXMedia::builder()
                    .media_type(MediaType::Audio)
                    .group_id("aac")
                    .name("Deutsch")
                    .is_default(false)
                    .is_autoselect(true)
                    .language("de")
                    .uri("main/german-audio.m3u8")
                    .build()
                    .unwrap(),
                ExtXMedia::builder()
                    .media_type(MediaType::Audio)
                    .group_id("aac")
                    .name("Commentary")
                    .is_default(false)
                    .is_autoselect(false)
                    .language("en")
                    .uri("commentary/audio-only.m3u8")
                    .build()
                    .unwrap(),
            ])
            .stream_inf_tags(vec![
                ExtXStreamInf::builder()
                    .bandwidth(1280000)
                    .codecs("...")
                    .audio("aac")
                    .uri("low/video-only.m3u8")
                    .build()
                    .unwrap(),
                ExtXStreamInf::builder()
                    .bandwidth(2560000)
                    .codecs("...")
                    .audio("aac")
                    .uri("mid/video-only.m3u8")
                    .build()
                    .unwrap(),
                ExtXStreamInf::builder()
                    .bandwidth(7680000)
                    .codecs("...")
                    .audio("aac")
                    .uri("hi/video-only.m3u8")
                    .build()
                    .unwrap(),
                ExtXStreamInf::builder()
                    .bandwidth(65000)
                    .codecs("mp4a.40.5")
                    .audio("aac")
                    .uri("main/english-audio.m3u8")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
        master_playlist
    );
}

#[test]
fn test_master_playlist_with_alternative_video() {
    // https://tools.ietf.org/html/rfc8216#section-8.7
    let master_playlist = "#EXTM3U\n\
                           #EXT-X-MEDIA:TYPE=VIDEO,GROUP-ID=\"low\",NAME=\"Main\", \
                           AUTOSELECT=YES,DEFAULT=YES,URI=\"low/main/audio-video.m3u8\"\n\

                           #EXT-X-MEDIA:TYPE=VIDEO,GROUP-ID=\"low\",NAME=\"Centerfield\", \
                           DEFAULT=NO,URI=\"low/centerfield/audio-video.m3u8\"\n\

                           #EXT-X-MEDIA:TYPE=VIDEO,GROUP-ID=\"low\",NAME=\"Dugout\", \
                           DEFAULT=NO,URI=\"low/dugout/audio-video.m3u8\"\n\

                           #EXT-X-STREAM-INF:BANDWIDTH=1280000,CODECS=\"...\",VIDEO=\"low\"\n\
                           low/main/audio-video.m3u8\n\

                           #EXT-X-MEDIA:TYPE=VIDEO,GROUP-ID=\"mid\",NAME=\"Main\", \
                           AUTOSELECT=YES,DEFAULT=YES,URI=\"mid/main/audio-video.m3u8\"\n\

                           #EXT-X-MEDIA:TYPE=VIDEO,GROUP-ID=\"mid\",NAME=\"Centerfield\", \
                           DEFAULT=NO,URI=\"mid/centerfield/audio-video.m3u8\"\n\

                           #EXT-X-MEDIA:TYPE=VIDEO,GROUP-ID=\"mid\",NAME=\"Dugout\", \
                           DEFAULT=NO,URI=\"mid/dugout/audio-video.m3u8\"\n\

                           #EXT-X-STREAM-INF:BANDWIDTH=2560000,CODECS=\"...\",VIDEO=\"mid\"\n\
                           mid/main/audio-video.m3u8\n\

                           #EXT-X-MEDIA:TYPE=VIDEO,GROUP-ID=\"hi\",NAME=\"Main\", \
                           AUTOSELECT=YES,DEFAULT=YES,URI=\"hi/main/audio-video.m3u8\"\n\

                           #EXT-X-MEDIA:TYPE=VIDEO,GROUP-ID=\"hi\",NAME=\"Centerfield\", \
                           DEFAULT=NO,URI=\"hi/centerfield/audio-video.m3u8\"\n\

                           #EXT-X-MEDIA:TYPE=VIDEO,GROUP-ID=\"hi\",NAME=\"Dugout\", \
                           DEFAULT=NO,URI=\"hi/dugout/audio-video.m3u8\"\n\

                           #EXT-X-STREAM-INF:BANDWIDTH=7680000,CODECS=\"...\",VIDEO=\"hi\"
                           hi/main/audio-video.m3u8"
        .parse::<MasterPlaylist>()
        .unwrap();

    assert_eq!(
        MasterPlaylist::builder()
            .media_tags(vec![
                // low
                ExtXMedia::builder()
                    .media_type(MediaType::Video)
                    .group_id("low")
                    .name("Main")
                    .is_default(true)
                    .is_autoselect(true)
                    .uri("low/main/audio-video.m3u8")
                    .build()
                    .unwrap(),
                ExtXMedia::builder()
                    .media_type(MediaType::Video)
                    .group_id("low")
                    .name("Centerfield")
                    .is_default(false)
                    .uri("low/centerfield/audio-video.m3u8")
                    .build()
                    .unwrap(),
                ExtXMedia::builder()
                    .media_type(MediaType::Video)
                    .group_id("low")
                    .name("Dugout")
                    .is_default(false)
                    .uri("low/dugout/audio-video.m3u8")
                    .build()
                    .unwrap(),
                // mid
                ExtXMedia::builder()
                    .media_type(MediaType::Video)
                    .group_id("mid")
                    .name("Main")
                    .is_default(true)
                    .is_autoselect(true)
                    .uri("mid/main/audio-video.m3u8")
                    .build()
                    .unwrap(),
                ExtXMedia::builder()
                    .media_type(MediaType::Video)
                    .group_id("mid")
                    .name("Centerfield")
                    .is_default(false)
                    .uri("mid/centerfield/audio-video.m3u8")
                    .build()
                    .unwrap(),
                ExtXMedia::builder()
                    .media_type(MediaType::Video)
                    .group_id("mid")
                    .name("Dugout")
                    .is_default(false)
                    .uri("mid/dugout/audio-video.m3u8")
                    .build()
                    .unwrap(),
                // hi
                ExtXMedia::builder()
                    .media_type(MediaType::Video)
                    .group_id("hi")
                    .name("Main")
                    .is_default(true)
                    .is_autoselect(true)
                    .uri("hi/main/audio-video.m3u8")
                    .build()
                    .unwrap(),
                ExtXMedia::builder()
                    .media_type(MediaType::Video)
                    .group_id("hi")
                    .name("Centerfield")
                    .is_default(false)
                    .uri("hi/centerfield/audio-video.m3u8")
                    .build()
                    .unwrap(),
                ExtXMedia::builder()
                    .media_type(MediaType::Video)
                    .group_id("hi")
                    .name("Dugout")
                    .is_default(false)
                    .uri("hi/dugout/audio-video.m3u8")
                    .build()
                    .unwrap(),
            ])
            .stream_inf_tags(vec![
                ExtXStreamInf::builder()
                    .bandwidth(1280000)
                    .codecs("...")
                    .video("low")
                    .uri("low/main/audio-video.m3u8")
                    .build()
                    .unwrap(),
                ExtXStreamInf::builder()
                    .bandwidth(2560000)
                    .codecs("...")
                    .video("mid")
                    .uri("mid/main/audio-video.m3u8")
                    .build()
                    .unwrap(),
                ExtXStreamInf::builder()
                    .bandwidth(7680000)
                    .codecs("...")
                    .video("hi")
                    .uri("hi/main/audio-video.m3u8")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
        master_playlist
    );
}
