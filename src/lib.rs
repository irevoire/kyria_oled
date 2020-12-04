pub mod frame;
pub use frame::Frame;

/// compress a slice of `u8`.
///
/// The compression will insert control bytes in the data.
/// Here is how control bytes works:
///
/// ----------------------------------
///
/// 7 6 5 4 3 2 1 0
/// | | | | | | | |
/// | ------------------> This is `n`
/// v
/// This is the `mode bit`
///
/// ----------------------------------
///
/// While the mode bit **is not** set the next 7 bits indicate how many times the following byte
/// repeats.
///
/// If the mode bit **is** set, the next 7 bits indicate the number of bytes you should keep as is.
///
///
/// For example if you want to compress [1, 1, 1, 1] the best mode will be the first and it will
/// give us: [4, 1].
///
/// Though, if you want to compresse [1, 2, 3, 4], the second mode is the best and will be give us:
/// [0
pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut iter = data.iter();
    let mut intermediate = Vec::new();

    // here we are only doing the first mode
    while let Some(&base) = iter.next() {
        let nb = iter
            .clone()
            .enumerate()
            .take_while(|(i, &b)| i < &0b0111_1111 && b == base)
            .count();
        (0..nb).for_each(|_| {
            iter.next();
        });

        intermediate.push(nb as u8 + 1);
        intermediate.push(base);
    }

    let mut res = Vec::new();
    let mut intermediate = intermediate.chunks(2);

    while let Some(base) = intermediate.next() {
        let (control, value) = (base[0], base[1]);

        if control == 1 {
            // how much control byte are also worth 1
            let nb = intermediate
                .clone()
                .enumerate()
                .take_while(|(i, b)| i < &0b0111_1111 && b[0] == 1)
                .count();

            // we set the mode bit
            res.push(((nb + 1) as u8) | 0b1000_0000);
            res.push(base[1]);

            (0..nb).for_each(|_| {
                let v = intermediate.next().unwrap();
                // we can throw the control byte now
                res.push(v[1]);
            });
        } else {
            res.push(base[0]);
            res.push(base[1]);
        }
    }

    res
}

pub fn uncompress(data: &[u8]) -> Vec<u8> {
    let mut iter = data.iter().copied();
    let mut res = Vec::new();

    while let Some(byte) = iter.next() {
        let (mode, n) = ((byte >> 7) == 1, byte & 0b0111_1111);
        if mode {
            (0..n).for_each(|_| res.push(iter.next().unwrap()));
        } else {
            let next = iter.next().unwrap();
            (0..n).for_each(|_| res.push(next));
        }
    }

    res
}

/// return a vec representing the diff between two vectors
pub fn diff(base: &[u8], other: &[u8]) -> Vec<u8> {
    base.iter()
        .zip(other)
        .map(|(base, &other)| base.wrapping_sub(other))
        .collect()
}

pub fn print_slice_as_c_array(varname: &str, v: &[u8]) {
    println!("static char PROGMEM {}[{}] = {{", varname, v.len());
    let mut col = 0;
    for index in 0..v.len() - 1 {
        let tmp = format!("{}, ", v[index]);
        col += tmp.len();
        if col < 80 {
            print!("{}", tmp);
        } else {
            col = tmp.len();
            print!("\n{}", tmp);
        }
    }

    println!("{}\n}};", v.last().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        assert_eq!(compress(&[0, 0, 0, 0, 0]), &[5, 0]);
        assert_eq!(
            compress(&[0, 0, 0, 1, 0]),
            &[3, 0, 0b1000_0010 /* 2 */, 1, 0]
        );
        assert_eq!(compress(&[0, 0, 0, 1, 1, 1, 2, 2, 2]), &[3, 0, 3, 1, 3, 2]);
    }

    #[test]
    fn test_uncompress() {
        assert_eq!(uncompress(&[5, 0]), &[0, 0, 0, 0, 0]);
        assert_eq!(
            uncompress(&[3, 0, 0b1000_0010 /* 2 */, 1, 0]),
            &[0, 0, 0, 1, 0]
        );
        assert_eq!(
            uncompress(&[3, 0, 3, 1, 3, 2]),
            &[0, 0, 0, 1, 1, 1, 2, 2, 2]
        );
    }

    const TEST_FRAME: [u8; 636] = [
        0, 0, 126, 126, 24, 60, 102, 66, 0, 12, 28, 112, 112, 28, 12, 0, 116, 116, 20, 20, 124,
        104, 0, 124, 124, 0, 112, 120, 44, 36, 124, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 128, 128, 0, 0, 0, 0, 0, 128, 64, 64, 32, 32, 32, 32, 16, 16, 16, 16, 8, 4,
        2, 1, 1, 2, 12, 48, 64, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 128,
        0, 0, 0, 0, 192, 96, 48, 24, 12, 132, 198, 98, 35, 51, 17, 145, 113, 241, 113, 145, 17, 51,
        35, 98, 198, 132, 12, 24, 48, 96, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 30, 225, 0, 0, 1, 1, 2, 2, 1, 0, 0, 0, 0, 128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0,
        48, 48, 0, 0, 1, 225, 26, 6, 9, 49, 53, 1, 138, 124, 0, 0, 128, 128, 128, 128, 64, 64, 64,
        64, 32, 32, 32, 32, 16, 16, 16, 16, 8, 8, 8, 8, 8, 196, 4, 196, 4, 196, 2, 194, 2, 194, 1,
        1, 1, 1, 0, 0, 0, 0, 0, 252, 15, 1, 0, 248, 14, 31, 109, 140, 148, 148, 164, 166, 249, 224,
        255, 224, 249, 166, 164, 148, 148, 140, 109, 31, 14, 248, 0, 1, 15, 252, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 112, 12, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 64, 160,
        33, 34, 18, 17, 17, 17, 9, 8, 8, 8, 8, 4, 4, 4, 4, 4, 4, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 170, 255, 255, 195, 191, 127, 3, 127, 191,
        195, 255, 255, 170, 170, 0, 0, 0, 0, 0, 0, 31, 120, 192, 0, 15, 56, 124, 219, 152, 20, 20,
        18, 50, 207, 3, 255, 3, 207, 50, 18, 20, 20, 152, 219, 124, 56, 15, 0, 192, 120, 31, 16,
        16, 16, 16, 8, 8, 8, 8, 8, 4, 4, 4, 4, 4, 2, 3, 122, 122, 121, 121, 121, 121, 57, 49, 2, 2,
        4, 4, 8, 8, 8, 136, 136, 135, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 130, 135, 31, 7, 159,
        7, 28, 7, 159, 7, 159, 7, 2, 130, 0, 0, 0, 0, 32, 16, 16, 16, 17, 11, 14, 12, 24, 16, 49,
        35, 98, 102, 68, 68, 71, 71, 71, 68, 68, 102, 98, 35, 49, 16, 24, 12, 6, 3, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 120, 124, 254, 255, 63, 7, 0, 0,
        0, 0, 255, 255, 127, 127, 63, 62, 28, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 8, 23, 0, 15, 1, 2, 1,
        15, 0, 15, 2, 5, 8,
    ];

    #[test]
    fn test_compress_uncompress() {
        assert_eq!(uncompress(&compress(&TEST_FRAME)), &TEST_FRAME);
    }
}
