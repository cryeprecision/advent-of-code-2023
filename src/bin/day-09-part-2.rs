fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(9, 2);

    // put all lines into a single vector to benefit from amortized insertion
    let mut numbers = Vec::new();
    let mut line_ends = vec![0];

    challenge.input_lines().for_each(|line| {
        line.split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .for_each(|num| numbers.push(num));
        line_ends.push(numbers.len());
    });
    challenge.finish_parsing();

    let solution = line_ends
        .windows(2)
        .map(|limits| {
            fn recurse(buffer: &mut [i64]) -> i64 {
                // check for end of recursion
                if buffer.len() < 2 || buffer.iter().all(|&num| num == 0) {
                    return 0;
                }

                // calculate differences in place
                for i in 0..(buffer.len() - 1) {
                    buffer[buffer.len() - i - 1] =
                        buffer[buffer.len() - i - 1] - buffer[buffer.len() - i - 2];
                }

                buffer[0] - recurse(&mut buffer[1..])
            }

            let list = &mut numbers[limits[0]..limits[1]];
            recurse(&mut list[..])
        })
        .sum::<i64>();

    challenge.finish(solution);
}
