extern crate hls_m3u8;
#[macro_use]
extern crate trackable;

use std::io::{self, Read};
use trackable::error::Failure;

fn main() {
    let mut m3u8 = String::new();
    track_try_unwrap!(
        io::stdin()
            .read_to_string(&mut m3u8)
            .map_err(Failure::from_error)
    );
    for line in hls_m3u8::line::Lines::new(&m3u8) {
        println!("{:?}", track_try_unwrap!(line));
    }
}
