extern crate clap;
extern crate hls_m3u8;
#[macro_use]
extern crate trackable;

use std::io::{self, Read};
use clap::{App, Arg};
use hls_m3u8::{MasterPlaylist, MediaPlaylist};
use trackable::error::Failure;

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
    track_try_unwrap!(
        io::stdin()
            .read_to_string(&mut m3u8)
            .map_err(Failure::from_error)
    );

    match matches.value_of("M3U8_TYPE").unwrap() {
        "media" => {
            let playlist: MediaPlaylist = track_try_unwrap!(m3u8.parse());
            println!("{}", playlist);
        }
        "master" => {
            let playlist: MasterPlaylist = track_try_unwrap!(m3u8.parse());
            println!("{}", playlist);
        }
        _ => unreachable!(),
    }
}
