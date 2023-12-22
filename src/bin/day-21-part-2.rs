use std::fmt::Write;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Pos {
    idx: usize,
    count: usize,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    /// Move the point with wrap-around
    fn move_pos(self, pos: Pos, map: &Map) -> Pos {
        let Pos { idx, count } = pos;
        match self {
            Dir::Up if idx >= map.width => Pos {
                idx: idx - map.width,
                count,
            },
            Dir::Up => Pos {
                idx: map.data.len() - map.width + idx,
                count,
            },

            Dir::Down if idx < map.data.len() - map.width => Pos {
                idx: idx + map.width,
                count,
            },
            Dir::Down => Pos {
                idx: idx - map.width * (map.height() - 1),
                count,
            },

            Dir::Left if idx % map.width != 0 => Pos {
                idx: idx - 1,
                count,
            },
            Dir::Left => Pos {
                idx: idx + (map.width - 1),
                count,
            },

            Dir::Right if idx % map.width != map.width - 1 => Pos {
                idx: idx + 1,
                count,
            },
            Dir::Right => Pos {
                idx: idx - (map.width - 1),
                count,
            },
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
    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn step(&self, curr_pos: Pos, new_pos_buf: &mut Vec<Pos>) {
        let reachable = [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
            .into_iter()
            .map(|dir| {
                let new_pos = dir.move_pos(curr_pos, self);
                (self.data[new_pos.idx] != b'#').then_some(new_pos)
            })
            .filter_map(|new_pos| new_pos);

        reachable.for_each(|new_pos| {
            match new_pos_buf.binary_search_by_key(&new_pos.idx, |p| p.idx) {
                Ok(idx) => new_pos_buf[idx].count += curr_pos.count,
                Err(idx) => new_pos_buf.insert(idx, new_pos),
            }
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
    let mut challenge = advent_of_code_2023::Challenge::start(21, 2);

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

    let mut positions = vec![Pos {
        idx: start_pos,
        count: 1,
    }];
    let mut new_positions = vec![];

    for i in 0..50 {
        let prev_len = positions.len();
        for pos in positions.drain(..) {
            map.step(pos, &mut new_positions);
        }
        std::mem::swap(&mut positions, &mut new_positions);

        println!("\x1B[2J\x1B[1;1H");
        println!(
            "[i] {:>3}: positions: {} ({:>4})",
            i,
            positions.len(),
            positions.len() as isize - prev_len as isize
        );

        let mut dbg = map.clone();
        positions.iter().for_each(|p| dbg.data[p.idx] = b'O');
        println!("[i] Map({}):\n{:?}\n", i, dbg);

        std::thread::sleep(std::time::Duration::from_millis(250));
    }

    let solution = positions.len();

    challenge.finish(solution);
}
