use std::collections::HashSet;

use aoclib::solution::{Solution, SolutionPair};

fn p1(p: &str, towels: &HashSet<&str>, max_size: usize) -> bool {
    let mut dp = vec![false; p.len() + 1];
    dp[0] = true;

    for i in 0..p.len() {
        for j in (i + 1)..=(i + max_size).min(p.len()) {
            if towels.contains(&p[i..j]) {
                dp[j] = dp[j] || dp[i];
            }
        }
    }

    dp[p.len()]
}

fn p2<'a>(p: &'a str, towels: &HashSet<&str>, max_size: usize) -> usize {
    let mut dp = vec![0; p.len() + 1];
    dp[0] = 1;

    for i in 0..p.len() {
        for j in (i + 1)..=(i + max_size).min(p.len()) {
            if towels.contains(&p[i..j]) {
                dp[j] += dp[i];
            }
        }
    }

    dp[p.len()]
}

pub fn solve(str: String) -> SolutionPair {
    let (towels, patterns) = str.split_once("\n\n").unwrap();
    let towels: HashSet<&str> = towels.split(',').map(|t| t.trim()).collect();

    let max_size = towels.iter().map(|t| t.len()).max().unwrap();

    let sol1 = patterns
        .lines()
        .filter(|p| p1(p, &towels, max_size))
        .count();

    let sol2: usize = patterns.lines().map(|p| p2(p, &towels, max_size)).sum();

    (Solution::from(sol1), Solution::from(sol2))
}
