fn main() {
    let input = std::fs::read_to_string("./input/day-01-part-1.txt").unwrap();

    let mut sum = 0u64;
    for line in input.trim().lines() {
        let first = line
            .chars()
            .find(|&char| char >= '0' && char <= '9')
            .unwrap() as u8
            - '0' as u8;

        let last = line
            .chars()
            .rev()
            .find(|&char| char >= '0' && char <= '9')
            .unwrap() as u8
            - '0' as u8;

        sum += first as u64 * 10 + last as u64;
    }

    println!("{}", sum);
}
