fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(9, 2);

    let lists = challenge
        .input_lines()
        .map(|list| {
            list.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    challenge.finish_parsing();

    let solution = lists
        .iter()
        .map(|list| {
            fn recurse(depth: usize, buffer: &mut [i64]) -> i64 {
                // check for end of recursion
                if buffer.len() < 2 || buffer.iter().all(|&num| num == 0) {
                    return 0;
                }

                // calculate differences in place
                for i in 0..(buffer.len() - 1) {
                    buffer[buffer.len() - i - 1] =
                        buffer[buffer.len() - i - 1] - buffer[buffer.len() - i - 2];
                }

                let my_result = buffer[0];
                my_result - recurse(depth + 1, &mut buffer[1..])
            }

            let mut list_buffer = list.clone();
            recurse(0, &mut list_buffer[..])
        })
        .sum::<i64>();

    challenge.finish(solution);
}
