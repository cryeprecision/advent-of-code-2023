use std::fmt::Write;

use smallvec::{smallvec, SmallVec};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Field {
    data: Vec<u8>,
    width: usize,
}

impl Field {
    fn height(&self) -> usize {
        self.data.len() / self.width
    }
    fn row(&self, row_idx: usize) -> &[u8] {
        &self.data[(row_idx * self.width)..((row_idx + 1) * self.width)]
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..self.height()).for_each(|row_idx| {
            f.write_str(std::str::from_utf8(self.row(row_idx)).unwrap())
                .unwrap();
            f.write_char('\n').unwrap();
        });
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Beam {
    pos: usize,
    dir: Dir,
}

impl Beam {
    fn new(pos: usize, dir: Dir) -> Beam {
        Beam { pos, dir }
    }

    fn step(self, field: &Field) -> SmallVec<[Self; 2]> {
        let next_pos = match self.dir {
            Dir::Up if self.pos >= field.width => self.pos - field.width,
            Dir::Down if self.pos < field.data.len() - field.width => self.pos + field.width,
            Dir::Left if self.pos % field.width != 0 => self.pos - 1,
            Dir::Right if self.pos % field.width != field.width - 1 => self.pos + 1,
            _ => return smallvec![],
        };

        match (self.dir, field.data[next_pos]) {
            (Dir::Up | Dir::Down, b'|') => smallvec![Beam::new(next_pos, self.dir)],
            (Dir::Left | Dir::Right, b'|') => {
                smallvec![Beam::new(next_pos, Dir::Up), Beam::new(next_pos, Dir::Down)]
            }

            (Dir::Up | Dir::Down, b'-') => smallvec![
                Beam::new(next_pos, Dir::Left),
                Beam::new(next_pos, Dir::Right)
            ],
            (Dir::Left | Dir::Right, b'-') => smallvec![Beam::new(next_pos, self.dir)],

            (Dir::Up, b'/') => smallvec![Beam::new(next_pos, Dir::Right)],
            (Dir::Right, b'/') => smallvec![Beam::new(next_pos, Dir::Up)],
            (Dir::Down, b'/') => smallvec![Beam::new(next_pos, Dir::Left)],
            (Dir::Left, b'/') => smallvec![Beam::new(next_pos, Dir::Down)],

            (Dir::Up, b'\\') => smallvec![Beam::new(next_pos, Dir::Left)],
            (Dir::Left, b'\\') => smallvec![Beam::new(next_pos, Dir::Up)],
            (Dir::Down, b'\\') => smallvec![Beam::new(next_pos, Dir::Right)],
            (Dir::Right, b'\\') => smallvec![Beam::new(next_pos, Dir::Down)],

            (_, b'.') => smallvec![Beam::new(next_pos, self.dir)],
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(16, 1);

    let field = {
        let width = challenge.input_lines().next().unwrap().len();
        let mut data = Vec::new();
        challenge
            .input_lines()
            .for_each(|line| data.extend_from_slice(line.as_bytes()));
        Field { data, width }
    };

    challenge.finish_parsing();

    let initial_state = match field.data[0] {
        b'-' => Beam::new(0, Dir::Right),
        b'|' => Beam::new(0, Dir::Down),
        b'/' => Beam::new(0, Dir::Up),
        b'\\' => Beam::new(0, Dir::Down),
        _ => panic!(),
    };

    let mut states = vec![initial_state];

    // similar to cycle detection
    let mut states_seen = states.clone();

    while let Some(state) = states.pop() {
        let new_states = state.step(&field);

        // do the cycle detection
        new_states.iter().for_each(|new_state| {
            if let Err(idx) = states_seen.binary_search(new_state) {
                states_seen.insert(idx, *new_state);
                states.push(*new_state);
            }
        });
    }

    let solution = {
        let mut iter = states_seen.iter();

        let mut count = 1usize;
        let mut last_pos = iter.next().unwrap().pos;

        iter.for_each(|state| {
            if state.pos != last_pos {
                last_pos = state.pos;
                count += 1;
            }
        });

        count
    };

    challenge.finish(solution);
}
