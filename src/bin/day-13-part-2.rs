#![feature(iter_map_windows)]

use std::fmt::Write;

#[derive(Default)]
struct Image {
    width: usize,
    data: Vec<u8>,
}

impl Image {
    /// Count the number of not equal elements
    fn cols_match_fuzzy(&self, col_lhs: usize, col_rhs: usize) -> usize {
        debug_assert_ne!(col_lhs, col_rhs);

        let height = self.data.len() / self.width;
        (0..height)
            .filter(|row_idx| {
                self.data[row_idx * self.width + col_lhs]
                    != self.data[row_idx * self.width + col_rhs]
            })
            .count()
    }

    /// Count the number of not equal elements
    fn rows_match_fuzzy(&self, row_lhs: usize, row_rhs: usize) -> usize {
        debug_assert_ne!(row_lhs, row_rhs);

        (0..self.width)
            .filter(|col_idx| {
                self.data[row_lhs * self.width + col_idx]
                    != self.data[row_rhs * self.width + col_idx]
            })
            .count()
    }

    fn solve(&self) -> usize {
        let my_height = self.data.len() / self.width;

        // set up iterators for lazily finding pairs of matching rows/cols
        let mut matching_row_pairs =
            (0..(my_height - 1))
                .zip(1..my_height)
                .filter_map(|(lhs, rhs)| match self.rows_match_fuzzy(lhs, rhs) {
                    errs @ 0..=1 => Some((lhs, rhs, errs)),
                    _ => None,
                });
        let mut matching_col_pairs =
            (0..(self.width - 1))
                .zip(1..self.width)
                .filter_map(|(lhs, rhs)| match self.cols_match_fuzzy(lhs, rhs) {
                    errs @ 0..=1 => Some((lhs, rhs, errs)),
                    _ => None,
                });

        // try to find mirror axis for rows
        if let Some((_, row)) = matching_row_pairs.find_map(|(row_lhs, row_rhs, mut errs)| {
            for (row_lhs, row_rhs) in (0..row_lhs).rev().zip((row_rhs..my_height).skip(1)) {
                errs += self.rows_match_fuzzy(row_lhs, row_rhs);
                if errs > 1 {
                    return None;
                }
            }
            match errs {
                1 => Some((row_lhs, row_rhs)),
                _ => None,
            }
        }) {
            return row * 100;
        }

        // if no mirror axis for rows was found, try to find mirror axis for cols
        if let Some((_, col)) = matching_col_pairs.find_map(|(col_lhs, col_rhs, mut errs)| {
            for (col_lhs, col_rhs) in (0..col_lhs).rev().zip((col_rhs..self.width).skip(1)) {
                errs += self.cols_match_fuzzy(col_lhs, col_rhs);
                if errs > 1 {
                    return None;
                }
            }
            match errs {
                1 => Some((col_lhs, col_rhs)),
                _ => None,
            }
        }) {
            return col;
        }

        panic!("image must contain at least one mirror axixs");
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self.data.len() / self.width;
        let mut rows = 0..height;

        if let Some(row_idx) = rows.next() {
            let row = &self.data[(row_idx * self.width)..((row_idx + 1) * self.width)];
            f.write_str(std::str::from_utf8(row).unwrap())?;

            for row_idx in rows {
                let row = &self.data[(row_idx * self.width)..((row_idx + 1) * self.width)];
                f.write_char('\n')?;
                f.write_str(std::str::from_utf8(row).unwrap())?;
            }
        }

        Ok(())
    }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(13, 2);

    let images = {
        let mut images = Vec::<Image>::new();
        let mut buffer = Image::default();

        let mut lines = challenge.input_lines();

        if let Some(line) = lines.next() {
            buffer.width = line.len();
            buffer.data.extend_from_slice(line.as_bytes());

            for line in lines {
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
