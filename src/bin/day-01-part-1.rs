fn main() {
    let input = std::fs::read_to_string("./input/day-01-part-1.txt").unwrap();

    let mut sum = 0u64;
    for line in input.trim().lines() {
        let first = line.chars().find(char::is_ascii_digit).unwrap() as u8 - b'0';
        let last = line.chars().rev().find(char::is_ascii_digit).unwrap() as u8 - b'0';

        sum += first as u64 * 10 + last as u64;
    }

    println!("{}", sum);
}
