use aoclib::solution::{Solution, SolutionPair};
use std::ops::Range;

type SeedRange = (Range<u64>, Range<u64>, u64, u64);

fn map_range(str: &str) -> Vec<SeedRange> {
    str.lines()
        .skip(1)
        .map(|map| {
            let res: Vec<u64> = map
                .split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect();
            (
                res[1]..res[1] + res[2],
                res[0]..res[0] + res[2],
                res[1],
                res[0],
            )
        })
        .collect()
}

pub fn solve(str: String) -> SolutionPair {
    let mut maps_iter = str.split("\n\n");

    let seeds: Vec<u64> = maps_iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|val| val.parse().unwrap())
        .collect();

    let maps: Vec<_> = maps_iter.map(|map| map_range(map)).collect();

    let mut temp = seeds.clone();
    for map in &maps {
        for seed in temp.iter_mut() {
            match map.iter().find(|r| r.0.contains(seed)) {
                Some(range) => {
                    *seed = (*seed) - range.2 + range.3;
                }
                None => { /* Nothing happens */ }
            }
        }
    }

    let sol1: u64 = *temp.iter().min().unwrap();

    //
    // Part 2
    //
    let seed_ranges: Vec<(u64, u64)> = seeds.chunks(2).map(|m| (m[0], m[1])).collect();

    let mut sol2 = 0;

    // Brute force backwards from location to seed for part 2
    'outer: for i in 0..u64::MAX {
        let mut seed: u64 = i;
        for map in maps.iter().rev() {
            match map.iter().find(|r| r.1.contains(&seed)) {
                Some(range) => {
                    seed = seed - range.3 + range.2;
                }
                None => { /* Nothing happens */ }
            }
        }

        for range in &seed_ranges {
            if seed >= range.0 && seed < range.0 + range.1 {
                sol2 = i;
                break 'outer;
            }
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
