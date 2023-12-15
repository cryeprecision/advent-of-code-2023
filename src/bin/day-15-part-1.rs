fn hash(input: &[u8]) -> u8 {
    input.iter().fold(0u64, |mut acc, &next| {
        acc += next as u64;
        acc *= 17;
        acc %= 256;
        acc
    }) as u8
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(15, 1);

    // parse input lazily
    let init_seq = challenge.input().as_bytes().split(|&b| b == b',');

    let solution = init_seq.map(|step| hash(step) as u64).sum::<u64>();
    challenge.finish(solution);
}
