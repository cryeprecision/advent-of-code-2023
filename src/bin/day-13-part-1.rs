#![feature(iter_map_windows)]

use std::fmt::Write;

#[derive(Default)]
struct Image {
    width: usize,
    data: Vec<u8>,
}

impl Image {
    fn cols_match(&self, col_lhs: usize, col_rhs: usize) -> bool {
        debug_assert_ne!(col_lhs, col_rhs);

        let height = self.data.len() / self.width;
        for row_idx in 0..height {
            if self.data[row_idx * self.width + col_lhs]
                != self.data[row_idx * self.width + col_rhs]
            {
                return false;
            }
        }

        true
    }

    fn rows_match(&self, row_lhs: usize, row_rhs: usize) -> bool {
        debug_assert_ne!(row_lhs, row_rhs);

        let lhs = &self.data[(row_lhs * self.width)..((row_lhs + 1) * self.width)];
        let rhs = &self.data[(row_rhs * self.width)..((row_rhs + 1) * self.width)];

        lhs == rhs
    }

    fn solve(&self) -> usize {
        let height = self.data.len() / self.width;

        let mut matching_row_pairs = (0..(height - 1))
            .zip(1..height)
            .filter(|&(lhs, rhs)| self.rows_match(lhs, rhs));

        let mut matching_col_pairs = (0..(self.width - 1))
            .zip(1..self.width)
            .filter(|&(lhs, rhs)| self.cols_match(lhs, rhs));

        let row_mirror = matching_row_pairs.find(|&(row_lhs, row_rhs)| {
            (0..row_lhs)
                .rev()
                .zip((row_rhs..height).skip(1))
                .all(|(row_rhs, row_lhs)| self.rows_match(row_lhs, row_rhs))
        });

        let col_mirror = matching_col_pairs.find(|&(col_lhs, col_rhs)| {
            (0..col_lhs)
                .rev()
                .zip((col_rhs..self.width).skip(1))
                .all(|(col_rhs, col_lhs)| self.cols_match(col_lhs, col_rhs))
        });

        match (row_mirror, col_mirror) {
            (Some((_, row)), None) => row * 100,
            (None, Some((_, col))) => col,
            _ => panic!("invalid mirrors: {:?}, {:?}", row_mirror, col_mirror),
        }
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self.data.len() / self.width;
        let mut rows = 0..height;

        if let Some(row_idx) = rows.next() {
            let row = &self.data[(row_idx * self.width)..((row_idx + 1) * self.width)];
            f.write_str(std::str::from_utf8(row).unwrap())?;

            while let Some(row_idx) = rows.next() {
                let row = &self.data[(row_idx * self.width)..((row_idx + 1) * self.width)];
                f.write_char('\n')?;
                f.write_str(std::str::from_utf8(row).unwrap())?;
            }
        }

        Ok(())
    }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(13, 1);

    let images = {
        let mut images = Vec::<Image>::new();
        let mut buffer = Image::default();

        let mut lines = challenge.input_lines();

        if let Some(line) = lines.next() {
            buffer.width = line.len();
            buffer.data.extend_from_slice(line.as_bytes());

            while let Some(line) = lines.next() {
                if line.is_empty() {
                    images.push(std::mem::take(&mut buffer));
                    continue;
                }

                buffer.width = line.len();
                buffer.data.extend_from_slice(line.as_bytes());
            }

            // append the last image
            images.push(std::mem::take(&mut buffer));
        }

        images
    };
    challenge.finish_parsing();

    let solution = images.iter().map(|image| image.solve()).sum::<usize>();

    challenge.finish(solution);
}
