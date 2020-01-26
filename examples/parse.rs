use clap::{App, Arg};
use hls_m3u8::{MasterPlaylist, MediaPlaylist};
use std::io::{self, Read};

fn main() {
    let matches = App::new("parse")
        .arg(
            Arg::with_name("M3U8_TYPE")
                .long("m3u8-type")
                .takes_value(true)
                .default_value("media")
                .possible_values(&["media", "master"]),
        )
        .get_matches();
    let mut m3u8 = String::new();
    io::stdin().read_to_string(&mut m3u8).unwrap();

    match matches.value_of("M3U8_TYPE").unwrap() {
        "media" => {
            let playlist: MediaPlaylist = m3u8.parse().unwrap();
            println!("{}", playlist);
        }
        "master" => {
            let playlist: MasterPlaylist = m3u8.parse().unwrap();
            println!("{}", playlist);
        }
        _ => unreachable!(),
    }
}
