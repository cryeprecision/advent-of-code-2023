fn main() {
    let challenge = advent_of_code_2023::Challenge::start(1, 1);

    let solution = challenge
        .input_lines()
        .map(|line| {
            let first = line.chars().find(char::is_ascii_digit).unwrap() as u8 - b'0';
            let last = line.chars().rev().find(char::is_ascii_digit).unwrap() as u8 - b'0';
            first as u64 * 10 + last as u64
        })
        .sum::<u64>();

    challenge.finish(solution);
}
