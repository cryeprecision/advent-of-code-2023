#![feature(slice_group_by)]
#![feature(array_windows)]

use std::fmt::Write;

use smallvec::SmallVec;

struct Maze {
    data: Vec<u8>,
    width: usize,
}

impl Maze {
    fn height(&self) -> usize {
        self.data.len() / self.width
    }
    fn row(&self, row_idx: usize) -> &[u8] {
        &self.data[(row_idx * self.width)..((row_idx + 1) * self.width)]
    }
    fn row_mut(&mut self, row_idx: usize) -> &mut [u8] {
        &mut self.data[(row_idx * self.width)..((row_idx + 1) * self.width)]
    }
}

impl std::fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..self.height()).for_each(|row_idx| {
            self.row(row_idx).iter().for_each(|&b| {
                f.write_char(match b {
                    b'|' => '┃', // is a vertical pipe connecting north and south.
                    b'-' => '━', // is a horizontal pipe connecting east and west.
                    b'L' => '┗', // is a 90-degree bend connecting north and east.
                    b'J' => '┛', // is a 90-degree bend connecting north and west.
                    b'7' => '┓', // is a 90-degree bend connecting south and west.
                    b'F' => '┏', // is a 90-degree bend connecting south and east.
                    _ => b as char,
                })
                .unwrap()
            });
            f.write_char('\n').unwrap();
        });
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    // Move in the current direction and set the new direction
    // based on the pipe we just moved through.
    fn next(self, pipe: u8) -> Option<Dir> {
        match (self, pipe) {
            (Dir::Up, b'|') => Some(Dir::Up),
            (Dir::Up, b'7') => Some(Dir::Left),
            (Dir::Up, b'F') => Some(Dir::Right),
            (Dir::Down, b'|') => Some(Dir::Down),
            (Dir::Down, b'L') => Some(Dir::Right),
            (Dir::Down, b'J') => Some(Dir::Left),
            (Dir::Left, b'-') => Some(Dir::Left),
            (Dir::Left, b'L') => Some(Dir::Up),
            (Dir::Left, b'F') => Some(Dir::Down),
            (Dir::Right, b'-') => Some(Dir::Right),
            (Dir::Right, b'J') => Some(Dir::Up),
            (Dir::Right, b'7') => Some(Dir::Down),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    idx: usize,
}

impl Pos {
    fn step(self, dir: Dir, maze: &Maze) -> Option<Pos> {
        match dir {
            Dir::Up if self.idx >= maze.width => Some(Pos {
                idx: self.idx - maze.width,
            }),
            Dir::Down if self.idx < maze.data.len() - maze.width => Some(Pos {
                idx: self.idx + maze.width,
            }),
            Dir::Left if self.idx % maze.width != 0 => Some(Pos { idx: self.idx - 1 }),
            Dir::Right if self.idx % maze.width != maze.width - 1 => {
                Some(Pos { idx: self.idx + 1 })
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    pos: Pos,
    dir: Dir,
}

impl State {
    fn new(idx: usize, dir: Dir) -> State {
        State {
            pos: Pos { idx },
            dir,
        }
    }
    fn step(self, maze: &Maze) -> Option<Self> {
        // Move a step in the current direction
        let pos = self.pos.step(self.dir, maze)?;
        // Change direction according to next pipe
        let dir = self.dir.next(maze.data[pos.idx])?;

        Some(State { pos, dir })
    }
}

fn start_states(maze: &Maze) -> [State; 2] {
    let start = maze
        .data
        .iter()
        .position(|&b| b == b'S')
        .map(|idx| Pos { idx })
        .unwrap();

    [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
        .into_iter()
        .filter_map(|dir| {
            let state = State::new(start.idx, dir);
            let _ = state.step(maze)?;
            Some(state)
        })
        .collect::<SmallVec<[State; 2]>>()
        .into_inner()
        .unwrap()
}

fn fill_inner_fields_row(maze_row: &mut [u8]) {
    // 'F-7' => false
    // 'L-J' => false
    // 'F-J' => true
    // 'L-7' => true
    // '|' => true

    let mut prev = None;
    let mut inside = false;

    for b in maze_row {
        if *b == b'.' && inside {
            // mark the spot as inside
            *b = b'I';
        } else if *b == b'-' {
            // this piece is ignored
            continue;
        } else if *b == b'|'
            || *b == b'S' // TODO: This is wrong and wont work for `F-S-7`
            || prev == Some(b'F') && *b == b'J'
            || prev == Some(b'L') && *b == b'7'
        {
            // we crossed a border
            inside = !inside;
            prev = None;
        }

        if b"7FJL".contains(b) {
            prev = Some(*b)
        }
    }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(10, 2);

    let mut maze = {
        let width = challenge.input_lines().next().unwrap().len();
        let mut data = Vec::new();
        challenge
            .input_lines()
            .for_each(|line| data.extend_from_slice(line.as_bytes()));
        Maze { data, width }
    };
    challenge.finish_parsing();

    let starts = start_states(&maze);
    let finish = starts[1].step(&maze).unwrap();
    let mut current = starts[0].step(&maze).unwrap();

    // record the path we walked
    let mut path = vec![starts[0].pos, current.pos];
    path.sort_unstable();

    // walk the path
    while current.pos != finish.pos {
        current = current.step(&maze).unwrap();
        path.insert(path.binary_search(&current.pos).unwrap_err(), current.pos);
    }

    // replace all pipes that are not part of the main loop with ground
    maze.data
        .iter_mut()
        .enumerate()
        .filter(|&(idx, _)| path.binary_search(&Pos { idx }).is_err())
        .for_each(|(_, b)| *b = b'.');

    // mark all inner fields
    (0..maze.height()).for_each(|row_idx| fill_inner_fields_row(maze.row_mut(row_idx)));

    // count the inner fields
    let solution = maze.data.iter().filter(|&&b| b == b'I').count();

    challenge.finish(solution);
}
