use std::collections::{HashMap, HashSet};

use aoclib::solution::{Solution, SolutionPair};

pub fn solve(str: String) -> SolutionPair {
    let (rules_str, updates) = str.split_once("\n\n").unwrap();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for l in rules_str.lines() {
        let num = l.split_once('|').unwrap();
        let key: i32 = num.0.parse().unwrap();
        let val: i32 = num.1.parse().unwrap();
        rules.entry(key).or_default().push(val);
    }

    let mut valid_updates = Vec::new();
    let mut invalid_updates = Vec::new();
    'next_update: for l in updates.lines() {
        let mut seen: HashSet<i32> = HashSet::new();
        let pages: Vec<i32> = l.split(',').map(|p| p.parse().unwrap()).collect();

        for p in &pages {
            let rule = rules.get(&p);

            if let None = rule {
                seen.insert(*p);
                continue;
            }

            if rule.unwrap().iter().any(|a| seen.contains(a)) {
                invalid_updates.push(pages);
                continue 'next_update;
            }

            seen.insert(*p);
        }

        valid_updates.push(pages);
    }

    let valid_updates_sum: i32 = valid_updates.iter().map(|u| u[u.len() / 2]).sum();
    let invalid_updates_sum: i32 = invalid_updates
        .iter_mut()
        .map(|update| {
            update.sort_by(|a, b| {
                let rule = rules.get(&a);
                if let None = rule {
                    return std::cmp::Ordering::Equal;
                }

                let rule = rule.unwrap();
                if rule.contains(b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });

            update[update.len() / 2]
        })
        .sum();

    (
        Solution::from(valid_updates_sum),
        Solution::from(invalid_updates_sum),
    )
}
