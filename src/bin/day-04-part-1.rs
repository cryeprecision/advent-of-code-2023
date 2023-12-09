struct Card<'a> {
    winners: &'a [u8],
    numbers: &'a [u8],
}

impl<'a> Card<'a> {
    fn matching_numbers(&self) -> usize {
        self.numbers
            .iter()
            .filter(|number| self.winners.binary_search(number).is_ok())
            .count()
    }

    fn points(&self) -> u64 {
        match self.matching_numbers() as u64 {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(4, 1);

    fn scan_input<'a>(state: &'a mut Vec<u8>, line: &'static str) -> Card<'a> {
        let (_, line) = line.split_once(": ").unwrap();
        let (winners, numbers) = line.split_once(" | ").unwrap();

        // reuse buffer
        state.clear();

        // push all winners
        winners
            .split_whitespace()
            .for_each(|num| state.push(num.parse::<u8>().unwrap()));

        // push all numbers
        let first_number = state.len();
        numbers
            .split_whitespace()
            .for_each(|num| state.push(num.parse::<u8>().unwrap()));

        // sort segments for binary search
        state[..first_number].sort_unstable();
        state[first_number..].sort_unstable();

        Card {
            winners: &state[..first_number],
            numbers: &state[first_number..],
        }
    }

    let mut state = Vec::new();
    let solution = challenge
        .input_lines()
        .map(|line| scan_input(&mut state, line).points())
        .sum::<u64>();

    challenge.finish(solution);
}
