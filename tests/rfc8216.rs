// https://tools.ietf.org/html/rfc8216#section-8
use std::time::Duration;

use hls_m3u8::tags::{
    ExtInf, ExtXEndList, ExtXKey, ExtXMedia, ExtXMediaSequence, ExtXTargetDuration, VariantStream,
};
use hls_m3u8::types::{DecryptionKey, EncryptionMethod, MediaType, StreamData};
use hls_m3u8::{MasterPlaylist, MediaPlaylist, MediaSegment};
use pretty_assertions::assert_eq;

macro_rules! generate_tests {
    ( $( $fnname:ident => { $struct:expr, $str:expr }),+ $(,)* ) => {
        $(
            #[test]
            fn $fnname() {
                assert_eq!($struct, $str.parse().unwrap());

                assert_eq!($struct.to_string(), $str.to_string());
            }
        )+
    }
}

generate_tests! {
    test_simple_playlist => {
        MediaPlaylist::builder()
            .target_duration(ExtXTargetDuration::new(Duration::from_secs(10)))
            .segments(vec![
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(9.009)))
                    .uri("http://media.example.com/first.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(9.009)))
                    .uri("http://media.example.com/second.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(3.003)))
                    .uri("http://media.example.com/third.ts")
                    .build()
                    .unwrap(),
            ])
            .end_list(ExtXEndList)
            .build()
            .unwrap(),
        concat!(
            "#EXTM3U\n",
            "#EXT-X-VERSION:3\n",
            "#EXT-X-TARGETDURATION:10\n",
            "#EXTINF:9.009,\n",
            "http://media.example.com/first.ts\n",
            "#EXTINF:9.009,\n",
            "http://media.example.com/second.ts\n",
            "#EXTINF:3.003,\n",
            "http://media.example.com/third.ts\n",
            "#EXT-X-ENDLIST\n"
        )
    },
    test_live_media_playlist_using_https => {
        MediaPlaylist::builder()
            .target_duration(ExtXTargetDuration::new(Duration::from_secs(8)))
            .media_sequence(ExtXMediaSequence::new(2680))
            .segments(vec![
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(7.975)))
                    .uri("https://priv.example.com/fileSequence2680.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(7.941)))
                    .uri("https://priv.example.com/fileSequence2681.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(7.975)))
                    .uri("https://priv.example.com/fileSequence2682.ts")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
        concat!(
            "#EXTM3U\n",
            "#EXT-X-VERSION:3\n",
            "#EXT-X-TARGETDURATION:8\n",
            "#EXT-X-MEDIA-SEQUENCE:2680\n",
            "#EXTINF:7.975,\n",
            "https://priv.example.com/fileSequence2680.ts\n",
            "#EXTINF:7.941,\n",
            "https://priv.example.com/fileSequence2681.ts\n",
            "#EXTINF:7.975,\n",
            "https://priv.example.com/fileSequence2682.ts\n",
        )
    },
    test_media_playlist_with_encrypted_segments => {
        MediaPlaylist::builder()
            .target_duration(ExtXTargetDuration::new(Duration::from_secs(15)))
            .media_sequence(ExtXMediaSequence::new(7794))
            .segments(vec![
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(2.833)))
                    .keys(vec![
                        ExtXKey::new(DecryptionKey::new(
                            EncryptionMethod::Aes128,
                            "https://priv.example.com/key.php?r=52"
                        ))
                    ])
                    .uri("http://media.example.com/fileSequence52-A.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(15.0)))
                    .keys(vec![
                        ExtXKey::new(DecryptionKey::new(
                            EncryptionMethod::Aes128,
                            "https://priv.example.com/key.php?r=52"
                        ))
                    ])
                    .uri("http://media.example.com/fileSequence52-B.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(13.333)))
                    .keys(vec![
                        ExtXKey::new(DecryptionKey::new(
                            EncryptionMethod::Aes128,
                            "https://priv.example.com/key.php?r=52"
                        ))
                    ])
                    .uri("http://media.example.com/fileSequence52-C.ts")
                    .build()
                    .unwrap(),
                MediaSegment::builder()
                    .inf(ExtInf::new(Duration::from_secs_f64(15.0)))
                    .keys(vec![
                        ExtXKey::new(DecryptionKey::new(
                            EncryptionMethod::Aes128,
                            "https://priv.example.com/key.php?r=53"
                        ))
                    ])
                    .uri("http://media.example.com/fileSequence53-A.ts")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
        concat!(
            "#EXTM3U\n",
            "#EXT-X-VERSION:3\n",
            "#EXT-X-TARGETDURATION:15\n",
            "#EXT-X-MEDIA-SEQUENCE:7794\n",

            "#EXT-X-KEY:METHOD=AES-128,URI=\"https://priv.example.com/key.php?r=52\"\n",

            "#EXTINF:2.833,\n",
            "http://media.example.com/fileSequence52-A.ts\n",
            "#EXTINF:15,\n",
            "http://media.example.com/fileSequence52-B.ts\n",
            "#EXTINF:13.333,\n",
            "http://media.example.com/fileSequence52-C.ts\n",

            "#EXT-X-KEY:METHOD=AES-128,URI=\"https://priv.example.com/key.php?r=53\"\n",

            "#EXTINF:15,\n",
            "http://media.example.com/fileSequence53-A.ts\n"
        )
    },
    test_master_playlist => {
        MasterPlaylist::builder()
            .variant_streams(vec![
                VariantStream::ExtXStreamInf {
                    uri: "http://example.com/low.m3u8".into(),
                    frame_rate: None,
                    audio: None,
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(1280000)
                        .average_bandwidth(1000000)
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "http://example.com/mid.m3u8".into(),
                    frame_rate: None,
                    audio: None,
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(2560000)
                        .average_bandwidth(2000000)
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "http://example.com/hi.m3u8".into(),
                    frame_rate: None,
                    audio: None,
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(7680000)
                        .average_bandwidth(6000000)
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "http://example.com/audio-only.m3u8".into(),
                    frame_rate: None,
                    audio: None,
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(65000)
                        .codecs(&["mp4a.40.5"])
                        .build()
                        .unwrap()
                },
            ])
            .build()
            .unwrap(),
        concat!(
            "#EXTM3U\n",
            "#EXT-X-STREAM-INF:BANDWIDTH=1280000,AVERAGE-BANDWIDTH=1000000\n",
            "http://example.com/low.m3u8\n",
            "#EXT-X-STREAM-INF:BANDWIDTH=2560000,AVERAGE-BANDWIDTH=2000000\n",
            "http://example.com/mid.m3u8\n",
            "#EXT-X-STREAM-INF:BANDWIDTH=7680000,AVERAGE-BANDWIDTH=6000000\n",
            "http://example.com/hi.m3u8\n",
            "#EXT-X-STREAM-INF:BANDWIDTH=65000,CODECS=\"mp4a.40.5\"\n",
            "http://example.com/audio-only.m3u8\n"
        )
    },
    test_master_playlist_with_i_frames => {
        MasterPlaylist::builder()
            .variant_streams(vec![
                VariantStream::ExtXStreamInf {
                    uri: "low/audio-video.m3u8".into(),
                    frame_rate: None,
                    audio: None,
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::new(1280000)
                },
                VariantStream::ExtXIFrame {
                    uri: "low/iframe.m3u8".into(),
                    stream_data: StreamData::new(86000),
                },
                VariantStream::ExtXStreamInf {
                    uri: "mid/audio-video.m3u8".into(),
                    frame_rate: None,
                    audio: None,
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::new(2560000)
                },
                VariantStream::ExtXIFrame {
                    uri: "mid/iframe.m3u8".into(),
                    stream_data: StreamData::new(150000),
                },
                VariantStream::ExtXStreamInf {
                    uri: "hi/audio-video.m3u8".into(),
                    frame_rate: None,
                    audio: None,
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::new(7680000)
                },
                VariantStream::ExtXIFrame {
                    uri: "hi/iframe.m3u8".into(),
                    stream_data: StreamData::new(550000),
                },
                VariantStream::ExtXStreamInf {
                    uri: "audio-only.m3u8".into(),
                    frame_rate: None,
                    audio: None,
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(65000)
                        .codecs(&["mp4a.40.5"])
                        .build()
                        .unwrap()
                },
            ])
            .build()
            .unwrap(),
        concat!(
            "#EXTM3U\n",
            "#EXT-X-STREAM-INF:BANDWIDTH=1280000\n",
            "low/audio-video.m3u8\n",
            "#EXT-X-I-FRAME-STREAM-INF:URI=\"low/iframe.m3u8\",BANDWIDTH=86000\n",
            "#EXT-X-STREAM-INF:BANDWIDTH=2560000\n",
            "mid/audio-video.m3u8\n",
            "#EXT-X-I-FRAME-STREAM-INF:URI=\"mid/iframe.m3u8\",BANDWIDTH=150000\n",
            "#EXT-X-STREAM-INF:BANDWIDTH=7680000\n",
            "hi/audio-video.m3u8\n",
            "#EXT-X-I-FRAME-STREAM-INF:URI=\"hi/iframe.m3u8\",BANDWIDTH=550000\n",
            "#EXT-X-STREAM-INF:BANDWIDTH=65000,CODECS=\"mp4a.40.5\"\n",
            "audio-only.m3u8\n"
        )
    },
    test_master_playlist_with_alternative_audio => {
        MasterPlaylist::builder()
            .media(vec![
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
            .variant_streams(vec![
                VariantStream::ExtXStreamInf {
                    uri: "low/video-only.m3u8".into(),
                    frame_rate: None,
                    audio: Some("aac".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(1280000)
                        .codecs(&["..."])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "mid/video-only.m3u8".into(),
                    frame_rate: None,
                    audio: Some("aac".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(2560000)
                        .codecs(&["..."])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "hi/video-only.m3u8".into(),
                    frame_rate: None,
                    audio: Some("aac".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(7680000)
                        .codecs(&["..."])
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "main/english-audio.m3u8".into(),
                    frame_rate: None,
                    audio: Some("aac".into()),
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(65000)
                        .codecs(&["mp4a.40.5"])
                        .build()
                        .unwrap()
                },
            ])
            .build()
            .unwrap(),
        concat!(
            "#EXTM3U\n",
            "#EXT-X-MEDIA:",
            "TYPE=AUDIO,",
            "URI=\"main/english-audio.m3u8\",",
            "GROUP-ID=\"aac\",",
            "LANGUAGE=\"en\",",
            "NAME=\"English\",",
            "DEFAULT=YES,",
            "AUTOSELECT=YES\n",

            "#EXT-X-MEDIA:",
            "TYPE=AUDIO,",
            "URI=\"main/german-audio.m3u8\",",
            "GROUP-ID=\"aac\",",
            "LANGUAGE=\"de\",",
            "NAME=\"Deutsch\",",
            "AUTOSELECT=YES\n",

            "#EXT-X-MEDIA:",
            "TYPE=AUDIO,",
            "URI=\"commentary/audio-only.m3u8\",",
            "GROUP-ID=\"aac\",",
            "LANGUAGE=\"en\",",
            "NAME=\"Commentary\"\n",

            "#EXT-X-STREAM-INF:BANDWIDTH=1280000,CODECS=\"...\",AUDIO=\"aac\"\n",
            "low/video-only.m3u8\n",

            "#EXT-X-STREAM-INF:BANDWIDTH=2560000,CODECS=\"...\",AUDIO=\"aac\"\n",
            "mid/video-only.m3u8\n",

            "#EXT-X-STREAM-INF:BANDWIDTH=7680000,CODECS=\"...\",AUDIO=\"aac\"\n",
            "hi/video-only.m3u8\n",

            "#EXT-X-STREAM-INF:BANDWIDTH=65000,CODECS=\"mp4a.40.5\",AUDIO=\"aac\"\n",
            "main/english-audio.m3u8\n"
        )
    },
    test_master_playlist_with_alternative_video => {
        MasterPlaylist::builder()
            .media(vec![
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
            .variant_streams(vec![
                VariantStream::ExtXStreamInf {
                    uri: "low/main/audio-video.m3u8".into(),
                    frame_rate: None,
                    audio: None,
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(1280000)
                        .codecs(&["..."])
                        .video("low")
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "mid/main/audio-video.m3u8".into(),
                    frame_rate: None,
                    audio: None,
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(2560000)
                        .codecs(&["..."])
                        .video("mid")
                        .build()
                        .unwrap()
                },
                VariantStream::ExtXStreamInf {
                    uri: "hi/main/audio-video.m3u8".into(),
                    frame_rate: None,
                    audio: None,
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(7680000)
                        .codecs(&["..."])
                        .video("hi")
                        .build()
                        .unwrap()
                },
            ])
            .build()
            .unwrap(),
        concat!(
            "#EXTM3U\n",
            "#EXT-X-MEDIA:",
            "TYPE=VIDEO,",
            "URI=\"low/main/audio-video.m3u8\",",
            "GROUP-ID=\"low\",",
            "NAME=\"Main\",",
            "DEFAULT=YES,",
            "AUTOSELECT=YES",
            "\n",

            "#EXT-X-MEDIA:",
            "TYPE=VIDEO,",
            "URI=\"low/centerfield/audio-video.m3u8\",",
            "GROUP-ID=\"low\",",
            "NAME=\"Centerfield\"",
            "\n",

            "#EXT-X-MEDIA:",
            "TYPE=VIDEO,",
            "URI=\"low/dugout/audio-video.m3u8\",",
            "GROUP-ID=\"low\",",
            "NAME=\"Dugout\"",
            "\n",


            "#EXT-X-MEDIA:",
            "TYPE=VIDEO,",
            "URI=\"mid/main/audio-video.m3u8\",",
            "GROUP-ID=\"mid\",",
            "NAME=\"Main\",",
            "DEFAULT=YES,",
            "AUTOSELECT=YES\n",

            "#EXT-X-MEDIA:",
            "TYPE=VIDEO,",
            "URI=\"mid/centerfield/audio-video.m3u8\",",
            "GROUP-ID=\"mid\",",
            "NAME=\"Centerfield\"\n",

            "#EXT-X-MEDIA:",
            "TYPE=VIDEO,",
            "URI=\"mid/dugout/audio-video.m3u8\",",
            "GROUP-ID=\"mid\",",
            "NAME=\"Dugout\"\n",

            "#EXT-X-MEDIA:",
            "TYPE=VIDEO,",
            "URI=\"hi/main/audio-video.m3u8\",",
            "GROUP-ID=\"hi\",",
            "NAME=\"Main\",",
            "DEFAULT=YES,",
            "AUTOSELECT=YES\n",

            "#EXT-X-MEDIA:",
            "TYPE=VIDEO,",
            "URI=\"hi/centerfield/audio-video.m3u8\",",
            "GROUP-ID=\"hi\",",
            "NAME=\"Centerfield\"\n",

            "#EXT-X-MEDIA:",
            "TYPE=VIDEO,",
            "URI=\"hi/dugout/audio-video.m3u8\",",
            "GROUP-ID=\"hi\",",
            "NAME=\"Dugout\"\n",

            "#EXT-X-STREAM-INF:BANDWIDTH=1280000,CODECS=\"...\",VIDEO=\"low\"\n",
            "low/main/audio-video.m3u8\n",

            "#EXT-X-STREAM-INF:BANDWIDTH=2560000,CODECS=\"...\",VIDEO=\"mid\"\n",
            "mid/main/audio-video.m3u8\n",

            "#EXT-X-STREAM-INF:BANDWIDTH=7680000,CODECS=\"...\",VIDEO=\"hi\"\n",
            "hi/main/audio-video.m3u8\n",
        )
    }
}
