pub fn generate_base_frame(between: &[Vec<u8>]) -> Vec<u8> {
    let mut base_frame = vec![None; between[0].len()];

    for idx in 0..base_frame.len() {
        if between
            .windows(2)
            .all(|frames| frames[0][idx] == frames[1][idx])
        {
            base_frame[idx] = Some(between[0][idx]);
        } else {
            base_frame[idx] = None;
        }
    }

    dbg!(base_frame);

    let idx = between
        .iter()
        .map(|frame| compute_size_from_base(frame, between))
        .enumerate()
        .min_by_key(|(_i, size)| *size)
        .unwrap()
        .0;
    dbg!(idx);
    between[idx].to_vec()
}

/// compute the total size of all the frame if we use the specified frame as a base
fn compute_size_from_base(base: &[u8], frames: &[Vec<u8>]) -> usize {
    frames
        .iter()
        .map(|frame| crate::generate_from_base(base, frame).len())
        .sum()
}
