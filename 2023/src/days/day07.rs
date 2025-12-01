use std::collections::HashMap;

use aoclib::solution::{Solution, SolutionPair};

#[derive(Clone, Copy)]
#[repr(u64)]
#[derive(Debug)]
enum HandType {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

pub fn solve(str: String) -> SolutionPair {
    let cards_map: HashMap<char, u64> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);

    let mut hands: Vec<_> = str
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let hand: Vec<char> = iter.next().unwrap().chars().collect();
            let bid: u64 = iter.next().unwrap().parse().unwrap();
            let mut counts_of_char: HashMap<u64, u64> = HashMap::new();
            let mut counts: Vec<u64> = vec![0; 6];
            let mut counts_no_j: Vec<u64> = vec![0; 6];

            for card in hand.iter() {
                *counts_of_char.entry(cards_map[card]).or_insert(0) += 1;
            }

            let count_jokers = *counts_of_char.entry(cards_map[&'J']).or_default() as usize;

            for char in counts_of_char.values() {
                counts[*char as usize] += 1;
            }

            let hand_type = if counts[5] != 0 {
                HandType::FiveOfKind
            } else if counts[4] == 1 {
                HandType::FourOfKind
            } else if counts[3] == 1 && counts[2] == 1 {
                HandType::FullHouse
            } else if counts[3] == 1 {
                HandType::ThreeOfKind
            } else if counts[2] == 2 {
                HandType::TwoPair
            } else if counts[2] == 1 {
                HandType::OnePair
            } else {
                HandType::HighCard
            };

            for (key, count) in counts_of_char.iter() {
                if *key != 11 {
                    counts_no_j[*count as usize] += 1;
                }
            }

            let hand_type_w_wild = if count_jokers == 5 || counts_no_j[5 - count_jokers] != 0 {
                HandType::FiveOfKind
            } else if counts_no_j[4 - count_jokers] >= 1 {
                HandType::FourOfKind
            } else if counts_no_j[3] == 1 && counts_no_j[2] == 1 {
                HandType::FullHouse
            } else if count_jokers == 1
                && (counts_no_j[2] == 2 || (counts_no_j[3] == 1 && counts_no_j[1] >= 1))
            {
                HandType::FullHouse
            } else if count_jokers == 2 && (counts_no_j[1] == 1 && counts_no_j[2] == 1) {
                HandType::FullHouse
            } else if counts_no_j[3 - count_jokers] >= 1 {
                HandType::ThreeOfKind
            } else if counts_no_j[2 - count_jokers] == 2 {
                HandType::TwoPair
            } else if count_jokers == 1 || counts_no_j[2] >= 1 {
                HandType::OnePair
            } else {
                HandType::HighCard
            };

            (hand, hand_type, bid, hand_type_w_wild)
        })
        .collect();

    // Part 1 sort

    hands.sort_by(|(a_chars, a_type, _, _), (b_char, b_type, _, _)| {
        if *a_type as u64 > *b_type as u64 {
            return std::cmp::Ordering::Greater;
        } else if (*a_type as u64) < *b_type as u64 {
            return std::cmp::Ordering::Less;
        }

        for (a, b) in a_chars.iter().zip(b_char.iter()) {
            let a = cards_map[a];
            let b = cards_map[b];
            if a > b {
                return std::cmp::Ordering::Greater;
            } else if a < b {
                return std::cmp::Ordering::Less;
            }
        }

        println!("Equal!");
        std::cmp::Ordering::Equal
    });

    let sol1: u64 = hands
        .iter()
        .enumerate()
        .map(|(idx, (_, _, bid, _))| (idx + 1) as u64 * bid)
        .sum();

    // Part 2 sort

    hands.sort_by(|(a_chars, _, _, a_type), (b_char, _, _, b_type)| {
        if *a_type as u64 > *b_type as u64 {
            return std::cmp::Ordering::Greater;
        } else if (*a_type as u64) < *b_type as u64 {
            return std::cmp::Ordering::Less;
        }

        for (a, b) in a_chars.iter().zip(b_char.iter()) {
            let a = if *a == 'J' { 1 } else { cards_map[a] };
            let b = if *b == 'J' { 1 } else { cards_map[b] };

            if a > b {
                return std::cmp::Ordering::Greater;
            } else if a < b {
                return std::cmp::Ordering::Less;
            }
        }

        std::cmp::Ordering::Equal
    });

    let sol2: u64 = hands
        .iter()
        .enumerate()
        .map(|(idx, (_, _, bid, _))| (idx + 1) as u64 * bid)
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
