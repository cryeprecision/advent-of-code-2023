#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn parse_hex(input: &str) -> u32 {
    let red = u8::from_str_radix(&input[1..3], 16).unwrap();
    let green = u8::from_str_radix(&input[3..5], 16).unwrap();
    let blue = u8::from_str_radix(&input[5..7], 16).unwrap();
    u32::from_le_bytes([0, red, green, blue])
}

#[derive(Debug, Clone, Copy)]
struct Op {
    dir: Dir,
    num: i32,
    col: u32,
}

struct Field {
    data: Vec<u8>,
    width: usize,
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(18, 1);

    let mut ops = challenge
        .input_lines()
        .map(|line| {
            let (dir, line) = line.split_once(' ').unwrap();
            let (num, col) = line.split_once(' ').unwrap();

            let dir = match dir.as_bytes()[0] {
                b'U' => Dir::Up,
                b'D' => Dir::Down,
                b'L' => Dir::Left,
                b'R' => Dir::Right,
                _ => panic!(),
            };

            let num = num.parse::<i32>().unwrap();
            let col = parse_hex(&col[1..(col.len() - 1)]);

            Op { dir, num, col }
        })
        .collect::<Vec<_>>();

    challenge.finish_parsing();

    let [_, mins, maxs] = ops.iter().fold([(0i32, 0i32); 3], |acc, next| {
        let [pos, mut mins, mut maxs] = acc;

        let new_pos = match next.dir {
            Dir::Up => (pos.0, pos.1 + next.num),
            Dir::Down => (pos.0, pos.1 - next.num),
            Dir::Left => (pos.0 - next.num, pos.1),
            Dir::Right => (pos.0 + next.num, pos.1),
        };

        mins.0 = mins.0.min(new_pos.0);
        mins.1 = mins.1.min(new_pos.1);

        maxs.0 = maxs.0.max(new_pos.0);
        maxs.1 = maxs.1.max(new_pos.1);

        [new_pos, mins, maxs]
    });

    let translation = (-mins.0, -mins.1);
    let _translated_poitnts = ops.iter().fold(vec![translation], |mut acc, next| {
        let pos = acc[acc.len() - 1];
        let new_pos = match next.dir {
            Dir::Up => (pos.0, pos.1 + next.num),
            Dir::Down => (pos.0, pos.1 - next.num),
            Dir::Left => (pos.0 - next.num, pos.1),
            Dir::Right => (pos.0 + next.num, pos.1),
        };

        acc.push(new_pos);
        acc
    });

    let width = usize::try_from(maxs.0 - mins.0 + 1).unwrap();
    let height = usize::try_from(maxs.1 - mins.1 + 1).unwrap();
    let mut grid = vec![b'.'; width * height];

    println!("translation: {:?}", translation);
    println!("points: {:?}", _translated_poitnts);

    let mut pos =
        usize::try_from(translation.1).unwrap() * width + usize::try_from(translation.0).unwrap();
    grid[pos] = b'#';

    for op in &ops {
        match op.dir {
            Dir::Up => {
                for row_offset in 1..=(op.num as usize) {
                    grid[pos + (row_offset * width)] = b'#';
                }
                pos += op.num as usize * width;
            }
            Dir::Down => {
                for row_offset in 1..=(op.num as usize) {
                    grid[pos - (row_offset * width)] = b'#';
                }
                pos -= op.num as usize * width;
            }
            Dir::Left => {
                for col_offset in 1..=(op.num as usize) {
                    grid[pos - col_offset] = b'#';
                }
                pos -= op.num as usize;
            }
            Dir::Right => {
                for col_offset in 1..=(op.num as usize) {
                    grid[pos + col_offset] = b'#';
                }
                pos += op.num as usize;
            }
        };
    }

    let mut buf = String::new();
    for row in 0..height {
        let line = &grid[(row * width)..((row + 1) * width)];
        let line = std::str::from_utf8(line).unwrap();
        buf.push_str(line);
        buf.push('\n');
    }
    std::fs::write("./map.txt", &buf).unwrap();

    challenge.finish(0);
}
