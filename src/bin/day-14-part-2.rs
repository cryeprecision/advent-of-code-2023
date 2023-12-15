#![feature(is_sorted)]

use std::{fmt::Write, hash::Hasher};

#[derive(Clone)]
struct Image {
    data: Vec<u8>,
    width: usize,
}

impl Image {
    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn row(&self, row_idx: usize) -> &[u8] {
        &self.data[(row_idx * self.width)..((row_idx + 1) * self.width)]
    }

    fn tilt_up(&mut self) {
        let height = self.height();
        (0..self.width).for_each(|col_idx| {
            let mut free_spot = None;
            (0..height).for_each(|row_idx| {
                let from_idx = row_idx * self.width + col_idx;
                match (self.data[from_idx], free_spot) {
                    (b'.', None) => free_spot = Some(from_idx),
                    (b'#', Some(_)) => free_spot = None,
                    (b'O', Some(to_idx)) => {
                        self.move_rock(from_idx, to_idx);
                        free_spot = Some(to_idx + self.width);
                    }
                    _ => (),
                }
            });
        });
    }
    fn tilt_down(&mut self) {
        let height = self.height();
        (0..self.width).for_each(|col_idx| {
            let mut free_spot = None;
            (0..height).rev().for_each(|row_idx| {
                let from_idx = row_idx * self.width + col_idx;
                match (self.data[from_idx], free_spot) {
                    (b'.', None) => free_spot = Some(from_idx),
                    (b'#', Some(_)) => free_spot = None,
                    (b'O', Some(to_idx)) => {
                        self.move_rock(from_idx, to_idx);
                        free_spot = Some(to_idx - self.width);
                    }
                    _ => (),
                }
            });
        });
    }
    fn tilt_left(&mut self) {
        let height = self.height();
        (0..height).for_each(|row_idx| {
            let mut free_spot = None;
            (0..self.width).for_each(|col_idx| {
                let from_idx = row_idx * self.width + col_idx;
                match (self.data[from_idx], free_spot) {
                    (b'.', None) => free_spot = Some(from_idx),
                    (b'#', Some(_)) => free_spot = None,
                    (b'O', Some(to_idx)) => {
                        self.move_rock(from_idx, to_idx);
                        free_spot = Some(to_idx + 1);
                    }
                    _ => (),
                }
            });
        })
    }
    fn tilt_right(&mut self) {
        let height = self.height();
        (0..height).for_each(|row_idx| {
            let mut free_spot = None;
            (0..self.width).rev().for_each(|col_idx| {
                let from_idx = row_idx * self.width + col_idx;
                match (self.data[from_idx], free_spot) {
                    (b'.', None) => free_spot = Some(from_idx),
                    (b'#', Some(_)) => free_spot = None,
                    (b'O', Some(to_idx)) => {
                        self.move_rock(from_idx, to_idx);
                        free_spot = Some(to_idx - 1);
                    }
                    _ => (),
                }
            });
        })
    }

    fn cycle(&mut self) {
        self.tilt_up();
        self.tilt_left();
        self.tilt_down();
        self.tilt_right();
    }

    fn cycle_n(&mut self, cycles: usize) {
        let mut hashes = vec![(0, self.hashed())];

        // keep cycling until we encounter the same hash twice
        let repeating_hash_idx = loop {
            if hashes.len() - 1 == cycles {
                return;
            }
            self.cycle();

            let hash = self.hashed();
            match hashes.binary_search_by_key(&hash, |&(_, hash)| hash) {
                Ok(idx) => break hashes[idx].0,
                Err(idx) => hashes.insert(idx, (hashes.len(), hash)),
            };
        };

        // after how many cycles we arrive at the same hash again
        let cycle_len = hashes.len() - repeating_hash_idx;

        // how many cycles were done before hitting the cycle start
        let before_cycle = repeating_hash_idx;

        // imagine we looped enough times and only less than cycle_len cycles are left
        let cycles_left = (cycles - before_cycle) % cycle_len;

        // do the remaining cycles
        (0..cycles_left).for_each(|_| self.cycle());
    }

    fn move_rock(&mut self, from: usize, to: usize) {
        self.data[from] = b'.';
        self.data[to] = b'O';
    }

    fn weight(&self) -> u64 {
        let height = self.height();
        (0..height)
            .map(|row_idx| {
                let rocks = self.row(row_idx).iter().filter(|&&b| b == b'O').count();
                (height - row_idx) * rocks
            })
            .sum::<usize>() as u64
    }

    fn hashed(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        hasher.write(&self.data);
        hasher.finish()
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = 0..self.height();
        if let Some(row) = rows.next() {
            let row = std::str::from_utf8(self.row(row)).unwrap();
            f.write_str(row)?;

            for row in rows {
                let row = std::str::from_utf8(self.row(row)).unwrap();
                f.write_char('\n')?;
                f.write_str(row)?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(14, 2);

    let mut image = {
        let width = challenge.input_lines().next().unwrap().len();
        let mut data = Vec::new();
        challenge
            .input_lines()
            .for_each(|line| data.extend_from_slice(line.as_bytes()));
        Image { data, width }
    };
    challenge.finish_parsing();

    image.cycle_n(1_000_000_000);

    challenge.finish(image.weight());
}
