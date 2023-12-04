fn main() {
    let input = advent_of_code_2023::load_input("day-01.txt");
    let start = std::time::Instant::now();

    let result = input
        .lines()
        .map(|line| {
            let first = line.chars().find(char::is_ascii_digit).unwrap() as u8 - b'0';
            let last = line.chars().rev().find(char::is_ascii_digit).unwrap() as u8 - b'0';
            first as u64 * 10 + last as u64
        })
        .sum::<u64>();

    let elapsed = start.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", result, elapsed);
}
