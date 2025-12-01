use std::collections::HashMap;

use aoclib::solution::{Solution, SolutionPair};

pub fn solve(str: String) -> SolutionPair {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    for line in str.lines() {
        let (a, b) = line.split_once("   ").unwrap();
        let av: i32 = a.parse().unwrap();
        let bv: i32 = b.parse().unwrap();
        left.push(av);
        right.push(bv);
        *right_map.entry(bv).or_default() += 1;
    }

    left.sort();
    right.sort();

    let sol1: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(&a, &b)| i32::abs_diff(a, b))
        .sum();

    let sol2: i32 = left
        .iter()
        .map(|&v| v * right_map.get(&v).cloned().unwrap_or_default())
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
