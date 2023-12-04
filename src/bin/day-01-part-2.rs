fn main() {
    let input = advent_of_code_2023::load_input("day-01.txt");
    let start = std::time::Instant::now();

    let map: [(&'static str, u64); 20] = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut sum = 0u64;
    for line in input.lines() {
        let first = (0..line.len())
            .find_map(|offset| {
                for (k, v) in map {
                    if line[offset..].starts_with(k) {
                        return Some(v);
                    } else {
                        continue;
                    }
                }
                None
            })
            .unwrap();

        let last = (0..line.len())
            .rev()
            .find_map(|offset| {
                for (k, v) in map {
                    if line[offset..].starts_with(k) {
                        return Some(v);
                    } else {
                        continue;
                    }
                }
                None
            })
            .unwrap();

        sum += first * 10 + last;
    }

    let elapsed = start.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", sum, elapsed);
}
