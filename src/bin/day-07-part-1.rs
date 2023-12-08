#![feature(slice_group_by)]

use std::cmp::Ordering;

use smallvec::SmallVec;

fn card_to_weight(c: char) -> u8 {
    match c {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("unknown card {}", c),
    }
}

fn weight_to_card(w: u8) -> char {
    match w {
        1 => 'J',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'T',
        11 => 'Q',
        12 => 'K',
        13 => 'A',
        _ => panic!("unknown weight {}", w),
    }
}

fn debug_cards(cards: [Card; 5]) -> String {
    let mut buffer = String::new();
    cards.map(|c| buffer.push(weight_to_card(c.weight)));
    buffer
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Card {
    weight: u8,
}

#[derive(Debug)]
struct Hand {
    hand: [Card; 5],
    bid: u64,
    type_weight: u8,
}

impl Hand {
    fn type_weight(mut hand: [Card; 5]) -> u8 {
        hand.sort_unstable();

        let groups: SmallVec<[(Card, usize); 5]> = hand
            .group_by(|lhs, rhs| lhs == rhs)
            .map(|group| (group[0], group.len()))
            .collect();

        let groups_count = groups.len();
        let max_len = groups.iter().fold(0, |acc, next| acc.max(next.1));

        match (groups_count, max_len) {
            (1, _) => 7, // Five of a kind
            (2, 4) => 6, // Four or a kind
            (2, _) => 5, // Full House
            (3, 3) => 4, // Thee of a kind
            (3, _) => 3, // Two pair
            (4, _) => 2, // One pair
            (5, _) => 1, // High card
            _ => panic!(
                "invalid amount of groups {}\n\
                    \thand: {}",
                groups.len(),
                debug_cards(hand),
            ),
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.type_weight.cmp(&other.type_weight);

        if cmp.is_ne() {
            cmp
        } else {
            self.hand.cmp(&other.hand)
        }
    }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(7, 1);

    let mut hands = challenge
        .input_lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            let hand = hand
                .chars()
                .map(|c| Card {
                    weight: card_to_weight(c),
                })
                .collect::<SmallVec<[Card; 5]>>()
                .into_inner()
                .unwrap();

            let type_weight = Hand::type_weight(hand);
            let bid = bid.parse::<u64>().unwrap();

            Hand {
                hand,
                bid,
                type_weight,
            }
        })
        .collect::<Vec<_>>();
    challenge.finish_parsing();

    hands.sort_unstable_by(Hand::cmp);

    let solution = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bid)
        .sum::<u64>();

    challenge.finish(solution);
}
