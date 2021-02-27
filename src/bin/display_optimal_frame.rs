use kyria_oled::Frame;
use std::env::args;
use std::io::{stdin, stdout, Write};

fn main() {
    let frames = args()
        .skip(1)
        .map(|filename| Frame::create_from_file(&filename).unwrap())
        .collect();

    println!("{}", Frame::create_from_multiple_frame(frames).unwrap());
}
