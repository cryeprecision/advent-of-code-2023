use smallvec::SmallVec;

fn debug_maze(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '|' => '┃', // is a vertical pipe connecting north and south.
            '-' => '━', // is a horizontal pipe connecting east and west.
            'L' => '┗', // is a 90-degree bend connecting north and east.
            'J' => '┛', // is a 90-degree bend connecting north and west.
            '7' => '┓', // is a 90-degree bend connecting south and west.
            'F' => '┏', // is a 90-degree bend connecting south and east.
            '.' => ' ', // is ground; there is no pipe in this tile.
            'S' => 'S', // is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
            _ => c,
        })
        .collect()
}

fn to_offset(kind: u8) -> (isize, isize) {
    match kind {
        '|' => (1, 0) // ┃
        '-' => (0, 1) // ━
        'L' => (1, 1) // ┗
        'J' => (1, 1) // ┛
        '7' => (1, 1) // ┓
        'F' => () // ┏
        _ => panic!("unknown kind {:?}", kind),
    }
}

fn next(current: (usize, usize), maze: &[&[u8]]) -> (usize, usize) {
    unimplemented!()
}

fn next_start(start: (usize, usize), maze: &[&[u8]]) -> [(usize, usize); 2] {
    const OFFSETS: [(isize, isize); 8] = [
        (-1, -1), // top left
        (-1, 0),  // top
        (-1, 1),  // top right
        (0, 1),   // right
        (1, 1),   // bottom right
        (1, 0),   // bottom
        (1, -1),  // bottom left
        (0, -1),  // left
    ];

    let mut buffer_offset = 0usize;
    let mut buffer = SmallVec::<[(usize, usize); 2]>::new();
    OFFSETS.iter().for_each(|&(row, col)| match maze[start.0 + row][start.1 + col] {
        '|' =>
        '-' =>
        'L' =>
        'J' =>
        '7' =>
        'F' =>
        '.' =>
    })

    for (row, col) in OFFSETS {

    }

    buffer
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(10, 1);

    // println!("{}", debug_maze(challenge.input()));

    let lines = challenge
        .input_lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let start = lines
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|&b| b == b'S').map(|col| (row, col)))
        .unwrap();

    println!("start: {:?}", start);

    let mut current = next(start, &lines);
    let mut path_len = 1u64;

    while current != start {
        current = next(current, &lines);
        path_len += 1;
    }

    challenge.finish(path_len);
}
