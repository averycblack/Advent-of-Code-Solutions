use std::collections::HashMap;

use aoclib::solution::{Solution, SolutionPair};

// Expect .###. or other variant
fn solve_part(cache: &mut HashMap<(String, Vec<usize>), u64>, str: &str, lengths: &[usize]) -> u64 {
    if cache.contains_key(&(str.to_owned(), lengths.to_owned())) {
        return *cache.get(&(str.to_owned(), lengths.to_owned())).unwrap();
    }

    let orig_str = str;

    if lengths.len() == 0 {
        let ret = if str.contains('#') { 0 } else { 1 };
        return ret;
    }

    if str.len() < 2 {
        return 0;
    }

    if str.starts_with('#') {
        return 0;
    }

    // remove first character then finish processing
    let str = &str[1..];

    let len = *lengths.first().unwrap();
    // not long enough to fit number of hot springs
    if len > str.len() {
        return 0;
    }

    for c in str[0..len].chars() {
        if c == '.' {
            return 0;
        }
    }

    // Valid segment!

    let mut sum = 0;
    let next_str = &str[len..];
    if !next_str.contains('#') && (!next_str.contains('?') || lengths.len() == 1) {
        sum += solve_part(cache, ".", &lengths[1..]);
    } else {
        for i in 0..next_str.find('#').unwrap_or(next_str.len()) {
            let new_perms = solve_part(cache, &next_str[i..], &lengths[1..]);
            sum += new_perms;
        }
    }

    cache.insert((orig_str.to_owned(), lengths.to_owned()), sum);

    sum
}

pub fn solve(str: String) -> SolutionPair {
    let sol1: u64 = str
        .lines()
        .map(|line| {
            let (springs, lengths) = line.split_once(' ').unwrap();
            let lengths: Vec<usize> = lengths.split(',').map(|val| val.parse().unwrap()).collect();

            let springs = String::from(".") + springs + ".";

            let mut permutations: u64 = 0;
            for i in 0..springs.find('#').unwrap_or(springs.len()) {
                permutations += solve_part(&mut HashMap::new(), &springs[i..], &lengths[..]);
            }
            permutations
        })
        .sum();

    let sol2: u64 = str
        .lines()
        .map(|line| {
            let (springs, lengths) = line.split_once(' ').unwrap();
            let lengths: Vec<usize> = lengths.split(',').map(|val| val.parse().unwrap()).collect();
            let len = lengths.len();
            let lengths: Vec<usize> = lengths.into_iter().cycle().take(5 * len).collect();

            let mut new_springs = String::from(springs);
            for _ in 0..3 {
                new_springs.push_str(&(String::from("?") + &springs));
            }

            new_springs = String::from(".") + &new_springs + "?" + &springs + ".";

            let mut permutations: u64 = 0;
            for i in 0..new_springs.find('#').unwrap_or(new_springs.len()) {
                permutations += solve_part(&mut HashMap::new(), &new_springs[i..], &lengths[..]);
            }
            // println!("{}", new_springs);
            // println!("{:?}", lengths);
            // println!("{}", permutations);
            permutations
        })
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
