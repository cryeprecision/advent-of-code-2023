#![feature(slice_group_by)]

use std::cmp::Ordering;

use smallvec::SmallVec;

fn card_weight(c: char) -> u64 {
    // High to low ordering
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2
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

fn hand_type_weight(ordered: &[char; 5]) -> u64 {
    fn max_len(gs: &[&[char]]) -> usize {
        gs.iter().fold(0, |acc, next| acc.max(next.len()))
    }

    let mut groups = SmallVec::<[&[char]; 5]>::new();
    ordered
        .group_by(|lhs, rhs| lhs == rhs)
        .for_each(|group| groups.push(group));

    match (groups.len(), max_len(&groups)) {
        (1, _) => 7, // Five of a kind
        (2, 4) => 6, // Four or a kind
        (2, _) => 5, // Full House
        (3, 3) => 4, // Thee of a kind
        (3, _) => 3, // Two pair
        (4, _) => 2, // One pair
        (5, _) => 1, // High card
        _ => panic!("invalid amound of groups {}", groups.len()),
    }
}

#[derive(Debug)]
struct Hand {
    hand: [char; 5],
    hand_ordered: [char; 5],
    bid: u64,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}
impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let type_cmp =
            hand_type_weight(&self.hand_ordered).cmp(&hand_type_weight(&other.hand_ordered));
        if type_cmp != Ordering::Equal {
            return Some(type_cmp);
        } else {
            let self_weights = self.hand.map(|c| card_weight(c));
            let other_weights = other.hand.map(|c| card_weight(c));
            return Some(self_weights.cmp(&other_weights));
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(7, 1);

    let mut hands = challenge
        .input_lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            let hand: [char; 5] = hand.chars().collect::<Vec<_>>().try_into().unwrap();

            let mut hand_ordered = hand.clone();
            hand_ordered.sort_by_key(|&c| card_weight(c));

            let bid = bid.parse::<u64>().unwrap();

            Hand {
                hand,
                hand_ordered,
                bid,
            }
        })
        .collect::<Vec<_>>();

    hands.sort_unstable();
    let solution = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bid)
        .sum::<u64>();

    challenge.finish(solution);
}
