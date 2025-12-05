use std::cmp::Ordering;

use aoclib::solution::{Solution, SolutionPair};

fn find_range(ranges: &Vec<(u64, u64)>, id: u64) -> Option<usize> {
    for (idx, (l, r)) in ranges.iter().enumerate() {
        if id >= *l && id <= *r {
            return Some(idx);
        }
    }

    None
}

pub fn solve(str: String) -> SolutionPair {
    let (ranges, ingrediants) = str.split_once("\n\n").unwrap();
    let ranges: Vec<(u64, u64)> = ranges.split_whitespace().map(|s| {
        let (l, r) = s.split_once('-').unwrap();
        let l = l.parse().unwrap();
        let r = r.parse().unwrap();
        (l, r)
    }).collect();

    let sol1 = ingrediants.split_whitespace().filter(|id| {
        let id: u64 = id.parse().unwrap();
        find_range(&ranges, id).is_some()
    }).count();

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    'next_range: for (l, r) in &ranges {
        // First check for if we fully contain any smaller ranges or fully contained by another range
        let mut to_remove = Vec::new();
        for (idx,ra) in merged_ranges.iter().enumerate() {
            if *l <= ra.0 && *r >= ra.1 {
                to_remove.push(idx);
            }

            if *l >= ra.0 && *r <= ra.1 {
                continue 'next_range;
            }
        }

        to_remove.sort();
        to_remove.iter().rev().for_each(|idx| {merged_ranges.remove(*idx);});

        // Now merge with ranges that may contain our start/end
        let l_range = find_range(&merged_ranges, *l);
        let r_range = find_range(&merged_ranges, *r);

        if l_range.is_some() && r_range.is_some() {
            let l_range = l_range.unwrap();
            let r_range = r_range.unwrap();

            if l_range == r_range {
                // Already fully contained within a range
                panic!();
            }

            // Modify first, then remove second range
            // Prevents indices moving around
            // Always merge into LEFT range
            let new_l = merged_ranges.get(l_range).unwrap().0;
            let new_r = merged_ranges.get(r_range).unwrap().1;
            merged_ranges[l_range] = (new_l.min(*l), new_r.max(*r));
            merged_ranges.remove(r_range);
        } else if l_range.is_some() {
            let l_range = l_range.unwrap();
            let new_range = merged_ranges.get_mut(l_range).unwrap();
            new_range.1 = new_range.1.max(*r);
        } else if r_range.is_some() {
            let r_range = r_range.unwrap();
            let new_range = merged_ranges.get_mut(r_range).unwrap();
            new_range.0 = new_range.0.min(*l);
        } else {
            merged_ranges.push((*l, *r));
        }
    }

    let sol2 = merged_ranges.iter().fold(0, |accum, (l, r)| {
        accum + (*r - *l) + 1
    });

    if cfg!(debug_assertions) {
        merged_ranges.sort_by(|a, b| {
            let res = a.0.cmp(&b.0);
            if res == Ordering::Equal {
                return a.1.cmp(&b.1);
            }
            res
        });

        // overlapping ranges
        for i in merged_ranges.windows(2) {
            if i[1].0 <= i[0].1 {
                panic!("!! {}-{} {}-{}", i[0].0, i[0].1, i[1].0, i[1].1);
            }
        }

        // malformed/reversed range
        for (l, r) in &merged_ranges {
            if l > r {
                panic!("!! {}-{}", l,r);
            }
        }
    }


    (Solution::from(sol1), Solution::from(sol2))
}
