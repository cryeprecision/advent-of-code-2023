#![feature(slice_group_by)]

use std::{cmp::Ordering, fmt::Debug};

use smallvec::SmallVec;

fn card_to_weight(c: char) -> u8 {
    match c {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
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

impl Card {
    fn is_joker(self) -> bool {
        self.weight == 1
    }
}

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

        let mut groups_count = groups.len();
        let mut max_len = groups.iter().fold(0, |acc, next| {
            if !next.0.is_joker() {
                acc.max(next.1)
            } else {
                acc
            }
        });

        if groups.len() > 1 {
            if let Some(jokers) = groups.iter().position(|group| group.0.is_joker()) {
                // remove jokers group
                groups_count -= 1;
                // add jokers to largest group
                max_len += groups[jokers].1;
            }
        }

        match (groups_count, max_len) {
            (1, _) => 7, // Five of a kind
            (2, 4) => 6, // Four of a kind
            (2, _) => 5, // Full House
            (3, 3) => 4, // Thee of a kind
            (3, _) => 3, // Two pair
            (4, _) => 2, // One pair
            (5, _) => 1, // High card
            _ => panic!(
                "invalid amount of groups {}\n\
                    \thand: {}",
                groups_count,
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
    let challenge = advent_of_code_2023::Challenge::start(7, 2);

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

    hands.sort_unstable_by(Hand::cmp);

    let solution = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bid)
        .sum::<u64>();

    challenge.finish(solution);
}
