use kyria_oled::*;
use std::path::Path;

fn main() {
    let filenames: Vec<String> = std::env::args().skip(1).collect();
    let frames: Vec<Vec<u8>> = filenames
        .iter()
        .map(|filename| Frame::create_from_file(&filename).unwrap())
        .map(|frame| frame.output())
        .collect();

    let base_frame = find_suboptimal_base_frame(&frames);
    // let base_frame = frames[3].clone();

    let mut total_size = base_frame.len();

    print_slice_as_c_array("BASE_FRAME", &base_frame);

    for idx in 0..frames.len() {
        let compressed_frame = compress(&diff(&base_frame, &frames[idx]));
        let array_name = Path::new(&filenames[idx])
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_uppercase();
        print_slice_as_c_array(&array_name, &compressed_frame);

        total_size += compressed_frame.len();
    }

    println!("// total array size is {} bytes", total_size);
}
