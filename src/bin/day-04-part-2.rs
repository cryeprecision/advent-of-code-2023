#![allow(dead_code)]

struct Card {
    id: u64,
    copies: u64,
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
}

fn main() {
    let start = std::time::Instant::now();
    let mut cards = advent_of_code_2023::load_input!("day-04.txt")
        .lines()
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
                copies: 1,
                winners,
                numbers,
            }
        })
        .collect::<Vec<_>>();

    // determine the number of copies for each card
    for i in 0..cards.len() {
        let matching_numbers = cards[i].matching_numbers();

        for j in 0..matching_numbers {
            cards[i + j + 1].copies += cards[i].copies;
        }
    }

    let result = cards.iter().map(|card| card.copies).sum::<u64>();

    let elapsed = start.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", result, elapsed);
}
