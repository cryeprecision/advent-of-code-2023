#![allow(dead_code)]

struct Card {
    id: u64,
    winners: Vec<u64>,
    numbers: Vec<u64>,
}

impl Card {
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
    let mut challenge = advent_of_code_2023::Challenge::start(4, 1);

    let cards = challenge
        .input_lines()
        .map(|line| {
            let (_, line) = line.split_once("Card").unwrap();
            let (id, line) = line.split_once(':').unwrap();
            let id = id.trim_start().parse::<u64>().unwrap();

            let mut winners = Vec::<u64>::new();
            let mut numbers = Vec::<u64>::new();

            let mut past_separator = false;
            for part in line.split_whitespace() {
                match (part, past_separator) {
                    ("|", _) => past_separator = true,
                    (_, false) => winners.push(part.parse().unwrap()),
                    (_, true) => numbers.push(part.parse().unwrap()),
                }
            }

            // sort for binary search
            winners.sort_unstable();
            numbers.sort_unstable();

            Card {
                id,
                winners,
                numbers,
            }
        })
        .collect::<Vec<_>>();
    challenge.finish_parsing();

    let solution = cards.iter().map(|card| card.points()).sum::<u64>();

    challenge.finish(solution);
}
