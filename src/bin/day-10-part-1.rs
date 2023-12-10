use smallvec::SmallVec;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn step(self, dir: Dir) -> Option<Pos> {
        let offset = match dir {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        };
        Some(Pos {
            row: self.row.checked_add_signed(offset.0)?,
            col: self.col.checked_add_signed(offset.1)?,
        })
    }
    fn is_inside(self, path: &[Pos]) -> bool {
        let intersections = path
            .iter()
            .filter(|p| p.row == self.row && p.col < self.col)
            .count();
        intersections % 2 != 0
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
    fn step(self, maze: &[&[u8]]) -> Option<Self> {
        // Move a step in the current direction
        let pos = self.pos.step(self.dir)?;
        // Change direction according to next pipe
        let dir = self.dir.next(maze[pos.row][pos.col])?;

        Some(State { pos, dir })
    }
}

fn start_states(pos: Pos, maze: &[&[u8]]) -> [State; 2] {
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
    let challenge = advent_of_code_2023::Challenge::start(10, 1);

    // println!("{}", debug_maze(challenge.input()));

    let maze = challenge
        .input_lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let start = maze
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .position(|&b| b == b'S')
                .map(|col| Pos { row, col })
        })
        .unwrap();

    let start_states = start_states(start, &maze);
    println!("start_states: {:?}", start_states);

    let finish = start_states[1].step(&maze).unwrap();
    let mut current = start_states[0].step(&maze).unwrap();

    // record the path we walked
    let mut path = vec![start_states[0].pos, current.pos];

    while current.pos != finish.pos {
        current = current.step(&maze).unwrap();
        path.push(current.pos);
    }

    // :(
    let dots = maze
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &b)| b == b'.')
                .map(|(col, _)| Pos { row, col })
        })
        .flatten()
        .collect::<Vec<_>>();

    challenge.finish(path.len());
}
