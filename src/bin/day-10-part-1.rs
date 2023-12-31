#![feature(slice_group_by)]

use smallvec::SmallVec;

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

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(10, 1);

    let maze = challenge
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

    let mut path_len = 2;

    while current.pos != finish.pos {
        current = current.step(&maze).unwrap();
        path_len += 1;
    }

    challenge.finish(path_len / 2);
}
