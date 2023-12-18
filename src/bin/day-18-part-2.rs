#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Op {
    dir: Dir,
    num: i64,
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(18, 2);

    let ops = challenge.input_lines().map(|line| {
        let (_, line) = line.split_once(' ').unwrap();
        let (_, num_dir) = line.split_once(' ').unwrap(); // (#7a21e3)

        let num = i64::from_str_radix(&num_dir[2..7], 16).unwrap();
        let dir = match num_dir.as_bytes()[7] {
            b'3' => Dir::Up,
            b'1' => Dir::Down,
            b'2' => Dir::Left,
            b'0' => Dir::Right,
            _ => panic!(),
        };

        Op { dir, num }
    });

    let mut translated_points = {
        let mut points = vec![(0i64, 0i64)];
        let mut mins = (0i64, 0i64);
        let mut pos = (0i64, 0i64);

        ops.for_each(|op| {
            let new_pos = match op.dir {
                Dir::Up => (pos.0, pos.1 + op.num),
                Dir::Down => (pos.0, pos.1 - op.num),
                Dir::Left => (pos.0 - op.num, pos.1),
                Dir::Right => (pos.0 + op.num, pos.1),
            };

            mins.0 = mins.0.min(new_pos.0);
            mins.1 = mins.1.min(new_pos.1);

            points.push(new_pos);
            pos = new_pos;
        });

        // translate points such that all coordinates are >=0
        points.iter_mut().for_each(|point| {
            point.0 -= mins.0;
            point.1 -= mins.1;
        });

        points
    };

    // https://stackoverflow.com/a/1165943
    let sum_of_edges = translated_points
        .windows(2)
        .map(|ps| (ps[1].0 - ps[0].0) * (ps[1].1 + ps[0].1)) // (x_2 - x_1)(y_2 + y_1)
        .sum::<i64>();

    if sum_of_edges > 0 {
        // if the polygon is oriented cw, reverse the points to make it ccw
        translated_points.reverse();
    }

    // https://en.wikipedia.org/wiki/Shoelace_formula
    let double_area = translated_points
        .windows(2)
        .map(|ps| (ps[0].1 + ps[1].1) * (ps[0].0 - ps[1].0)) // (y_1 + y_2)(x_1 - x_2)
        .sum::<i64>();

    fn point_dist(lhs: (i64, i64), rhs: (i64, i64)) -> i64 {
        (lhs.0.abs_diff(rhs.0) + lhs.1.abs_diff(rhs.1)) as i64
    }

    let path_len_rem = point_dist(
        translated_points[0],
        translated_points[translated_points.len() - 1],
    );
    let path_len = translated_points
        .windows(2)
        .fold(path_len_rem, |acc, next| acc + point_dist(next[0], next[1]));

    challenge.finish((double_area + path_len) / 2 + 1);
}
