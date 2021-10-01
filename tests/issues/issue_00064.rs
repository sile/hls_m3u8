// The relevant issue:
// https://github.com/sile/hls_m3u8/issues/55
use std::convert::TryFrom;

use hls_m3u8::tags::VariantStream;
use hls_m3u8::types::StreamData;
use hls_m3u8::MasterPlaylist;

use pretty_assertions::assert_eq;

#[test]
fn parse() {
    let file = include_str!("assets/issue_00064.m3u8");

    assert_eq!(
        MasterPlaylist::try_from(file).unwrap(),
        MasterPlaylist::builder()
            .variant_streams(vec![
                VariantStream::ExtXStreamInf {
                    uri: "https://995107575.cloudvdn.com/a.m3u8?cdn=cn-gotcha03&domain=d1--cn-gotcha103.bilivideo.com&expires=1614619920&len=0&oi=1891753406&order=1&player=70YAALwcl0b9RGgW&pt=h5&ptype=0&qn=10000&secondToken=secondToken%3ACZ4ggpPHomuwcnT8XWDjJUp9eh8&sign=325afc8bc3b01ccbadeac084004ece64&sigparams=cdn%2Cexpires%2Clen%2Coi%2Cpt%2Cqn%2Ctrid&sl=1&src=4&streamid=live-qn%3Alive-qn%2Flive_402401719_42665292&trid=20d9f245179b4ef3a7e3635afaaa87ea&v3=1".into(),
                    frame_rate: None,
                    audio: None,
                    subtitles: None,
                    closed_captions: None,
                    stream_data: StreamData::builder()
                        .bandwidth(10000000)
                        .build()
                        .unwrap()
                }
            ])
            .build()
            .unwrap()
    );
}
