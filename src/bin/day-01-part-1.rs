fn main() {
    let input = advent_of_code_2023::load_input!("day-01.txt");

    let mut sum = 0u64;
    for line in input.lines() {
        let first = line.chars().find(char::is_ascii_digit).unwrap() as u8 - b'0';
        let last = line.chars().rev().find(char::is_ascii_digit).unwrap() as u8 - b'0';

        sum += first as u64 * 10 + last as u64;
    }

    println!("{}", sum);
}
