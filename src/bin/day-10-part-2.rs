#![feature(slice_group_by)]

use smallvec::{smallvec, SmallVec};

#[allow(dead_code)]
fn debug_maze(maze: &[Vec<u8>]) -> String {
    maze.iter()
        .map(|line| {
            line.iter()
                .map(|&c| match c {
                    b'|' => '┃', // is a vertical pipe connecting north and south.
                    b'-' => '━', // is a horizontal pipe connecting east and west.
                    b'L' => '┗', // is a 90-degree bend connecting north and east.
                    b'J' => '┛', // is a 90-degree bend connecting north and west.
                    b'7' => '┓', // is a 90-degree bend connecting south and west.
                    b'F' => '┏', // is a 90-degree bend connecting south and east.
                    b'.' => '.', // is ground; there is no pipe in this tile.
                    b'S' => 'S', // is the starting position of the animal;
                    // there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
                    _ => c as char,
                })
                .collect::<String>()
        })
        .reduce(|mut acc, next| {
            acc.push('\n');
            acc.push_str(&next);
            acc
        })
        .unwrap()
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
    row: usize,
    col: usize,
}

impl Pos {
    fn step(self, dir: Dir, maze: &[Vec<u8>]) -> Option<Pos> {
        let offset = match dir {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        };

        // check for integer over-/underflow
        let pos = Pos {
            row: self.row.checked_add_signed(offset.0)?,
            col: self.col.checked_add_signed(offset.1)?,
        };

        // check that we don't move out of the maze bounds
        if pos.row >= maze.len() || pos.col >= maze[pos.row].len() {
            None
        } else {
            Some(pos)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    pos: Pos,
    dir: Dir,
}

impl State {
    fn new(row: usize, col: usize, dir: Dir) -> State {
        State {
            pos: Pos { row, col },
            dir,
        }
    }
    fn step(self, maze: &[Vec<u8>]) -> Option<Self> {
        // Move a step in the current direction
        let pos = self.pos.step(self.dir, maze)?;
        // Change direction according to next pipe
        let dir = self.dir.next(maze[pos.row][pos.col])?;

        Some(State { pos, dir })
    }
    fn mark_step(self, maze: &[Vec<u8>], inner_fields: &mut Vec<Pos>) -> Option<Self> {
        // dir is pointing in the direction of the exit of the current pipe
        let insides: SmallVec<[Dir; 2]> = match (maze[self.pos.row][self.pos.col], self.dir) {
            (b'-', Dir::Left) => smallvec![Dir::Up],
            (b'-', Dir::Right) => smallvec![Dir::Down],
            (b'|', Dir::Up) => smallvec![Dir::Right],
            (b'|', Dir::Down) => smallvec![Dir::Left],
            (b'7', Dir::Left) => smallvec![Dir::Up, Dir::Right],
            (b'7', Dir::Down) => smallvec![],
            (b'F', Dir::Right) => smallvec![],
            (b'F', Dir::Down) => smallvec![Dir::Up, Dir::Left],
            (b'J', Dir::Left) => smallvec![],
            (b'J', Dir::Up) => smallvec![Dir::Right, Dir::Down],
            (b'L', Dir::Right) => smallvec![Dir::Left, Dir::Down],
            (b'L', Dir::Up) => smallvec![],
            _ => smallvec![],
        };

        insides.iter().for_each(|&dir| {
            let Some(pos) = self.pos.step(dir, maze) else {
                return;
            };
            // add to the list of inner fields
            if maze[pos.row][pos.col] == b'.' {
                if let Err(idx) = inner_fields.binary_search(&pos) {
                    inner_fields.insert(idx, pos);
                }
            }
        });

        self.step(maze)
    }
}

fn start_states(pos: Pos, maze: &[Vec<u8>]) -> [State; 2] {
    [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
        .into_iter()
        .filter_map(|dir| {
            let state = State::new(pos.row, pos.col, dir);
            let _ = state.step(maze)?;
            Some(state)
        })
        .collect::<SmallVec<[State; 2]>>()
        .into_inner()
        .unwrap()
}

fn horizontal_fill(pos: Pos, maze: &mut [Vec<u8>]) {
    let line = maze[pos.row].as_mut_slice();

    for col in (pos.col..line.len()).skip(1) {
        if line[col] == b'.' {
            line[col] = b'I';
        } else {
            break;
        }
    }

    for col in (0..pos.col).rev().skip(1) {
        if line[col] == b'.' {
            line[col] = b'I';
        } else {
            break;
        }
    }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(10, 2);

    let mut maze = challenge
        .input_lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    challenge.finish_parsing();

    let start = maze
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .position(|&b| b == b'S')
                .map(|col| Pos { row, col })
        })
        .unwrap();

    let starts = start_states(start, &maze);

    let finish = starts[1].step(&maze).unwrap();
    let mut current = starts[0].step(&maze).unwrap();

    // record the path we walked
    let mut path = vec![starts[0].pos, current.pos];

    while current.pos != finish.pos {
        current = current.step(&maze).unwrap();
        path.push(current.pos);
    }

    // replace all pipes that are not part of the main loop with ground
    maze.iter_mut().enumerate().for_each(|(row, line)| {
        line.iter_mut().enumerate().for_each(|(col, c)| {
            if !path.contains(&Pos { row, col }) {
                *c = b'.';
            }
        })
    });

    let starts = start_states(start, &maze);
    let finish = starts[1].step(&maze).unwrap();

    let mut inner_fields = Vec::new();
    let mut current = starts[0].mark_step(&maze, &mut inner_fields).unwrap();

    while current.pos != finish.pos {
        current = current.mark_step(&maze, &mut inner_fields).unwrap();
    }

    inner_fields.iter().for_each(|&pos| {
        maze[pos.row][pos.col] = b'I';
    });
    inner_fields.iter().for_each(|&pos| {
        horizontal_fill(pos, &mut maze);
    });

    let solution = maze
        .iter()
        .map(|line| line.iter().filter(|&&c| c == b'I').count())
        .sum::<usize>();

    challenge.finish(solution);
}
