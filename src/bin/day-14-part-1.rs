use std::fmt::Write;

use num_integer::Integer;

#[derive(Hash)]
struct Image {
    data: Vec<u8>,
    width: usize,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Pos {
        Pos { row, col }
    }
    fn checked_add_signed(self, row: isize, col: isize, image: Pos) -> Option<Pos> {
        let row = self.row.checked_add_signed(row)?;
        let col = self.col.checked_add_signed(col)?;
        if row >= image.row || col >= image.col {
            return None;
        }
        Some(Pos::new(row, col))
    }
}

impl Dir {
    fn add_to(self, pos: Pos, image: Pos) -> Option<Pos> {
        match self {
            Dir::Up => pos.checked_add_signed(-1, 0, image),
        }
    }
}

impl Image {
    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn row(&self, row_idx: usize) -> &[u8] {
        &self.data[(row_idx * self.width)..((row_idx + 1) * self.width)]
    }

    fn at_pos(&self, pos: Pos) -> u8 {
        self.data[pos.row * self.width + pos.col]
    }
    fn dimensions(&self) -> Pos {
        Pos::new(self.height(), self.width)
    }

    fn best_free_spot(&self, rock: Pos, dir: Dir) -> Option<Pos> {
        let dims = self.dimensions();

        let mut best_pos = dir.add_to(rock, dims)?;
        if self.at_pos(best_pos) != b'.' {
            return None;
        }

        while let Some(next_pos) = dir.add_to(best_pos, dims) {
            if self.at_pos(next_pos) == b'.' {
                best_pos = next_pos;
            } else {
                break;
            }
        }

        Some(best_pos)
    }

    fn rocks(&self, rock_buf: &mut Vec<Pos>) {
        debug_assert_eq!(rock_buf.len(), 0);
        let rocks = self
            .data
            .iter()
            .enumerate()
            .filter(|(_, &b)| b == b'O')
            .map(|(idx, _)| {
                let (row, col) = idx.div_rem(&self.width);
                Pos { row, col }
            });
        rock_buf.extend(rocks);
    }

    fn tilt_up(&mut self, rocks: &[Pos]) {
        (0..self.width).for_each(|col_idx| {
            rocks
                .iter()
                .filter(|&rock_pos| rock_pos.col == col_idx)
                .for_each(|&rock_pos| {
                    if let Some(new_rock_pos) = self.best_free_spot(rock_pos, Dir::Up) {
                        self.move_rock(rock_pos, new_rock_pos);
                    }
                });
        });
    }

    fn move_rock(&mut self, from: Pos, to: Pos) {
        self.data[from.row * self.width + from.col] = b'.';
        self.data[to.row * self.width + to.col] = b'O';
    }

    fn weight(&self) -> u64 {
        let height = self.height();
        self.data
            .iter()
            .enumerate()
            .filter(|(_, &b)| b == b'O')
            .map(|(idx, _)| (height - (idx / self.width)) as u64)
            .sum::<u64>()
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
    let mut challenge = advent_of_code_2023::Challenge::start(14, 1);

    let mut image = {
        let width = challenge.input_lines().next().unwrap().len();
        let mut data = Vec::new();
        challenge
            .input_lines()
            .for_each(|line| data.extend_from_slice(line.as_bytes()));
        Image { data, width }
    };
    challenge.finish_parsing();

    let mut rock_buf = Vec::new();

    image.rocks(&mut rock_buf);
    image.tilt_up(&rock_buf);
    let solution = image.weight();

    challenge.finish(solution);
}
