use aoclib::solution::{Solution, SolutionPair};

pub fn solve(str: String) -> SolutionPair {
    // I'm too lazy to find the number of lines or do this in a smarter way
    // I hope this is big enough!
    let mut cards: Vec<u64> = vec![1; 204];

    let winnings: Vec<u64> = str
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let line = &line[line.find(": ").unwrap() + 1..];

            let mut iter = line.split(" | ");
            let winning_nums: Vec<u64> = iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();
            let nums: Vec<u64> = iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();

            let matches = nums
                .iter()
                .filter(|num| winning_nums.contains(&num))
                .count() as u32;

            for i in 1..=matches {
                cards[idx + i as usize] += cards[idx];
            }

            if matches == 0 {
                return 0;
            }

            u64::pow(2, matches - 1)
        })
        .collect();

    let sol1: u64 = winnings.iter().sum();

    // This slice is a consequence of using way too big of a vec
    // for the smaller examples
    let sol2: u64 = cards[..winnings.len()].iter().sum();

    (Solution::from(sol1), Solution::from(sol2))
}
