fn parse_digit(str: &str) -> Option<u64> {
    // check for single character digit first
    let c = str.chars().next()?;
    if c.is_ascii_digit() {
        return Some((c as u8 - b'0') as u64);
    }

    if str.len() < 3 {
        return None;
    }
    match &str[..3] {
        "one" => return Some(1),
        "two" => return Some(2),
        "six" => return Some(6),
        _ => (),
    }

    if str.len() < 4 {
        return None;
    }
    match &str[..4] {
        "zero" => return Some(0),
        "four" => return Some(4),
        "five" => return Some(5),
        "nine" => return Some(9),
        _ => (),
    }

    if str.len() < 5 {
        return None;
    }
    match &str[..5] {
        "three" => return Some(3),
        "seven" => return Some(7),
        "eight" => return Some(8),
        _ => (),
    }

    None
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(1, 2);

    let solution = challenge
        .input_lines()
        .map(|line| {
            let first = (0..line.len())
                .find_map(|offset| parse_digit(&line[offset..]))
                .unwrap();

            let last = (0..line.len())
                .rev()
                .find_map(|offset| parse_digit(&line[offset..]))
                .unwrap();

            first * 10 + last
        })
        .sum::<u64>();

    challenge.finish(solution);
}
