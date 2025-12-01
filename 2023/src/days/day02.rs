use std::collections::HashMap;

use aoclib::solution::{Solution, SolutionPair};

pub fn solve(str: String) -> SolutionPair {
    let mut idx = 0;

    let sol1: u64 = str
        .lines()
        .map(|line| {
            let line = &line[(line.find(':').unwrap() + 1)..];
            let pulls: Vec<&str> = line.split(|c| c == ';' || c == ',').collect();

            idx += 1;

            for set in pulls {
                let set = set.trim();
                let num = set[..set.find(' ').unwrap()].parse::<u64>().unwrap();
                let color = &set[set.find(' ').unwrap() + 1..];

                let max = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!("Unknown color {}", color),
                };

                if num > max {
                    return 0;
                }
            }

            idx
        })
        .sum();

    let sol2: u64 = str
        .lines()
        .map(|line| {
            let line = &line[(line.find(':').unwrap() + 1)..];

            let mut map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            let pulls: Vec<&str> = line.split(|c| c == ';' || c == ',').collect();
            for set in pulls {
                let set = set.trim();
                let num = set[..set.find(' ').unwrap()].parse::<u64>().unwrap();
                let color = &set[set.find(' ').unwrap() + 1..];

                if map[color] < num {
                    map.insert(color, num);
                }
            }

            map.values().product::<u64>()
        })
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
