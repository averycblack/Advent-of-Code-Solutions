use std::collections::HashMap;

use aoclib::solution::{Solution, SolutionPair};

fn blink(map: &HashMap<u64, u64>, blinks: u64) -> u64 {
    let mut p1 = map.clone();
    for _ in 0..blinks {
        let mut t: HashMap<u64, u64> = HashMap::new();
        t.insert(1, p1.remove(&0).unwrap_or_default());

        for (v, c) in p1 {
            let digits = v.ilog10() + 1;
            let mask = 10_u64.pow(digits / 2);
            if (digits % 2) == 0 {
                *t.entry(v % mask).or_default() += c;
                *t.entry(v / mask).or_default() += c;
            } else {
                *t.entry(v * 2024).or_default() += c;
            }
        }
        p1 = t;
    }

    p1.values().sum()
}

pub fn solve(str: String) -> SolutionPair {
    let arr: Vec<u64> = str
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut map: HashMap<u64, u64> = HashMap::new();
    for val in arr {
        *map.entry(val).or_default() += 1;
    }

    let sol1 = blink(&map, 25);
    let sol2 = blink(&map, 75);

    (Solution::from(sol1), Solution::from(sol2))
}
