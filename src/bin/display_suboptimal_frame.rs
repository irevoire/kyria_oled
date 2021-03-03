use kyria_oled::*;
use std::env::args;

fn main() {
    let frames: Vec<Frame> = args()
        .skip(1)
        .map(|filename| Frame::create_from_file(&filename).unwrap())
        .collect();

    let (width, height) = (frames[0].width(), frames[0].height());

    let frames: Vec<Vec<u8>> = frames.iter().map(|frame| frame.output()).collect();

    println!(
        "{}",
        Frame::new(width, height, &find_suboptimal_base_frame(&frames)).unwrap()
    );
}
