pub struct Frame {
    frame: Vec<Vec<u8>>,
}

impl Frame {
    pub fn new(
        width: usize,
        height: usize,
        frame: &[u8],
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if false && frame.len() * 8 != width * height {
            return Err(format!(
                "frame length ({} pixels) not matching with the width and height ({} × {} = {} pixels).",
                frame.len() * 8,
                width,
                height,
                width * height
            )
            .into());
        }
        let mut internal = vec![vec![0_u8; width]; height];

        for (height, line) in frame.chunks(width).enumerate() {
            for (width, b) in line.iter().enumerate() {
                let height = height * 8;
                internal[height][width] = (b >> 0) & 1;
                internal[height + 1][width] = (b >> 1) & 1;
                internal[height + 2][width] = (b >> 2) & 1;
                internal[height + 3][width] = (b >> 3) & 1;
                internal[height + 4][width] = (b >> 4) & 1;
                internal[height + 5][width] = (b >> 5) & 1;
                internal[height + 6][width] = (b >> 6) & 1;
                internal[height + 7][width] = (b >> 7) & 1;
            }
        }

        Ok(Self { frame: internal })
    }

    /// return the height of the frame
    pub fn height(&self) -> usize {
        self.frame.len()
    }

    /// return the width of the frame
    pub fn width(&self) -> usize {
        self.frame[0].len()
    }

    /// return the dimensions of the frame in this order: (width, height)
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    pub fn create_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::read(filename)?;
        let mut frame: Vec<Vec<u8>> = file[..file.len() - 1]
            .split(|&b| b == b'\n')
            .map(|line| {
                line.iter()
                    .map(|b| match b {
                        b'.' => 0,
                        b'#' => 1,
                        _ => panic!("lala"),
                    })
                    .collect::<Vec<u8>>()
            })
            .collect();

        // just ensure every line is the same width
        if frame.windows(2).any(|v| v[0].len() != v[1].len()) {
            return Err("All the frame needs to be the same width".into());
        } else {
            Ok(Self { frame })
        }
    }

    // generate an optimal frame from other frame
    // This frame is basically the average of all the other frames
    pub fn create_from_multiple_frame(frames: &[Self]) -> Result<Self, Box<dyn std::error::Error>> {
        if frames
            .windows(2)
            .any(|v| v[0].dimensions() != v[1].dimensions())
        {
            return Err("All the frames need to have the same dimensions!".into());
        }
        let width = frames[0].width();
        let height = frames[0].height();

        let mut v = vec![vec![0; width]; height];
        for frame in frames {
            for y in 0..height {
                for x in 0..width {
                    v[y][x] += frame.frame[y][x];
                }
            }
        }

        for y in 0..height {
            for x in 0..width {
                if v[y][x] > frames.len() as u8 / 2 {
                    v[y][x] = 1;
                } else {
                    v[y][x] = 0;
                }
            }
        }

        Ok(Self { frame: v })
    }

    pub fn print(&self) {
        for line in self.frame.iter() {
            for c in line {
                match c {
                    0 => print!("  "),
                    1 => print!("██"),
                    _ => print!("NO"),
                }
            }
            println!();
        }
    }

    pub fn output(&self) -> Vec<u8> {
        let width = self.frame[0].len();
        let height = self.frame.len();

        let mut res = (0..height)
            .step_by(8)
            .map(move |height| {
                (0..width).map(move |width| {
                    self.frame[height][width]
                        | (self.frame[height + 1][width] << 1)
                        | (self.frame[height + 2][width] << 2)
                        | (self.frame[height + 3][width] << 3)
                        | (self.frame[height + 4][width] << 4)
                        | (self.frame[height + 5][width] << 5)
                        | (self.frame[height + 6][width] << 6)
                        | (self.frame[height + 7][width] << 7)
                })
            })
            .flatten()
            .collect::<Vec<u8>>();

        // we remove all the trailings zeros
        if let Some(i) = res.iter().rposition(|x| *x != 0) {
            let new_len = i + 1;
            res.truncate(new_len);
        }
        res
    }

    pub fn compress(&self) -> Vec<u8> {
        crate::compress(&self.output())
    }
}

impl std::fmt::Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.frame.iter() {
            for c in line {
                match c {
                    0 => write!(f, "  ")?,
                    1 => write!(f, "██")?,
                    _ => write!(f, "NO")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FRAME: [u8; 636] = [
        0, 0, 126, 126, 24, 60, 102, 66, 0, 12, 28, 112, 112, 28, 12, 0, 116, 116, 20, 20, 124,
        104, 0, 124, 124, 0, 112, 120, 44, 36, 124, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 64, 64, 32, 32, 32, 32, 16, 16, 16, 16, 16, 8, 8,
        4, 4, 4, 8, 48, 64, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 128, 0,
        0, 0, 0, 192, 96, 48, 24, 12, 132, 198, 98, 35, 51, 17, 145, 113, 241, 113, 145, 17, 51,
        35, 98, 198, 132, 12, 24, 48, 96, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 24, 100, 130, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0, 128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0,
        48, 48, 0, 192, 193, 193, 194, 4, 8, 16, 32, 64, 128, 0, 0, 0, 128, 128, 128, 128, 64, 64,
        64, 64, 32, 32, 32, 32, 16, 16, 16, 16, 8, 8, 8, 8, 8, 196, 4, 196, 4, 196, 2, 194, 2, 194,
        1, 1, 1, 1, 0, 0, 0, 0, 0, 252, 15, 1, 0, 248, 14, 31, 109, 140, 148, 148, 164, 166, 249,
        224, 255, 224, 249, 166, 164, 148, 148, 140, 109, 31, 14, 248, 0, 1, 15, 252, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 56, 4, 3, 0, 0, 0, 0, 0, 0, 0, 12, 12, 12, 13, 1, 0,
        64, 160, 33, 34, 18, 17, 17, 17, 9, 8, 8, 8, 8, 4, 4, 8, 8, 16, 16, 16, 16, 16, 17, 15, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 170, 255, 255, 195, 191, 127,
        3, 127, 191, 195, 255, 255, 170, 170, 0, 0, 0, 0, 0, 0, 31, 120, 192, 0, 15, 56, 124, 219,
        152, 20, 20, 18, 50, 207, 3, 255, 3, 207, 50, 18, 20, 20, 152, 219, 124, 56, 15, 0, 192,
        120, 31, 16, 16, 16, 16, 8, 8, 8, 8, 8, 4, 4, 4, 4, 4, 2, 3, 2, 2, 1, 1, 1, 1, 1, 1, 2, 2,
        4, 4, 8, 8, 8, 8, 8, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 130, 135, 31, 7, 159, 7, 28,
        7, 159, 7, 159, 7, 2, 130, 0, 0, 0, 0, 32, 16, 16, 16, 17, 11, 14, 12, 24, 16, 49, 35, 98,
        102, 68, 68, 71, 71, 71, 68, 68, 102, 98, 35, 49, 16, 24, 12, 6, 3, 1, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 8, 23, 0, 15, 1, 2, 1, 15, 0, 15, 2, 5, 8,
    ];

    #[test]
    fn test_output() {
        let frame = Frame::new(128, 40, &FRAME).unwrap();
        assert_eq!(&frame.output(), &FRAME);
    }
}
