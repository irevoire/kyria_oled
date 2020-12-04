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
        let nb = iter.clone().take_while(|&b| b == &base).count();
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
            let nb = intermediate.clone().take_while(|b| b[0] == 1).count();

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

/// return a vec representing the diff between two vectors
pub fn diff(base: &[u8], other: &[u8]) -> Vec<u8> {
    base.iter()
        .zip(other)
        .map(|(base, &other)| base.wrapping_sub(other))
        .collect()
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
}
