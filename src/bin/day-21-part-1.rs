use std::fmt::Write;

use smallvec::SmallVec;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn move_point(self, point: usize, map: &Map) -> Option<usize> {
        match self {
            Dir::Up if point >= map.width => Some(point - map.width),
            Dir::Down if point < map.data.len() - map.width => Some(point + map.width),
            Dir::Left if point % map.width != 0 => Some(point - 1),
            Dir::Right if point % map.width != map.width - 1 => Some(point + 1),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Map {
    data: Vec<u8>,
    width: usize,
}

impl Map {
    fn row(&self, row_idx: usize) -> &[u8] {
        &self.data[(row_idx * self.width)..((row_idx + 1) * self.width)]
    }
    fn row_of<'a, T>(&self, other: &'a [T], row_idx: usize) -> &'a [T] {
        &other[(row_idx * self.width)..((row_idx + 1) * self.width)]
    }
    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn step(&self, curr_pos: usize, new_pos_buf: &mut Vec<usize>) {
        let reachable = [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
            .into_iter()
            .map(|dir| {
                dir.move_point(curr_pos, self)
                    .and_then(|new_pos| (self.data[new_pos] == b'.').then_some(new_pos))
            })
            .filter_map(|new_pos| new_pos);

        reachable.for_each(|new_pos| match new_pos_buf.binary_search(&new_pos) {
            Ok(_) => (/* already in the list */),
            Err(idx) => new_pos_buf.insert(idx, new_pos),
        });
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = 0..self.height();
        if let Some(row_idx) = rows.next() {
            f.write_str(std::str::from_utf8(self.row(row_idx)).unwrap())?;
            for row_idx in rows {
                f.write_char('\n')?;
                f.write_str(std::str::from_utf8(self.row(row_idx)).unwrap())?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(21, 1);

    let map = {
        let width = challenge.input_lines().next().unwrap().len();
        let data = challenge.input_lines().fold(Vec::new(), |mut acc, next| {
            acc.extend_from_slice(next.as_bytes());
            acc
        });
        Map { data, width }
    };

    challenge.finish_parsing();

    let start_pos = map.data.iter().position(|&b| b == b'S').unwrap();

    let mut positions = vec![start_pos];
    let mut new_positions = vec![];

    for _ in 0..64 {
        for pos in positions.drain(..) {
            map.step(pos, &mut new_positions);
        }
        std::mem::swap(&mut positions, &mut new_positions);
    }

    let solution = positions.len() + 1;

    challenge.finish(solution);
}
