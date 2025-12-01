use aoclib::solution::{Solution, SolutionPair};

use std::{cmp, collections::HashSet};

fn part1(str: &str) -> u64 {
    let mut sol1: u64 = 0;

    let lines: Vec<Vec<char>> = str.lines().map(|l| l.chars().collect()).collect();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if !c.is_ascii_alphanumeric() {
                continue;
            }

            if x != 0 && line[x - 1].is_numeric() {
                continue;
            }

            let mut end_idx = x + 1;
            let mut num = c.to_digit(10).unwrap();
            while end_idx < line.len() && line[end_idx].is_numeric() {
                num = (num * 10) + line[end_idx].to_digit(10).unwrap();
                end_idx += 1;
            }

            let min_x: usize = cmp::max(0, x as i64 - 1) as usize;
            let min_y: usize = cmp::max(0, y as i64 - 1) as usize;
            let max_x: usize = cmp::min(end_idx as i64, line.len() as i64 - 1) as usize;
            let max_y: usize = cmp::min(y as i64 + 1, lines.len() as i64 - 1) as usize;

            let big_slice = &lines[min_y..=max_y];
            let big_slice: Vec<&[char]> = big_slice.iter().map(|l| &l[min_x..=max_x]).collect();

            let res = big_slice
                .iter()
                .any(|p| p.iter().any(|p| !p.is_numeric() && *p != '.'));
            if res {
                sol1 += num as u64;
            }
        }
    }

    sol1
}

fn part2(str: &str) -> u64 {
    let lines: Vec<Vec<char>> = str.lines().map(|l| l.chars().collect()).collect();

    let mut nums = vec![vec![-1; lines[0].len()]; lines.len()];

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if !c.is_ascii_alphanumeric() {
                continue;
            }

            if x != 0 && line[x - 1].is_numeric() {
                continue;
            }

            let mut end_idx = x + 1;
            let mut num: i64 = c.to_digit(10).unwrap() as i64;
            while end_idx < line.len() && line[end_idx].is_numeric() {
                num = (num * 10) + line[end_idx].to_digit(10).unwrap() as i64;
                end_idx += 1;
            }

            for i in x..end_idx {
                nums[y][i] = num;
            }
        }
    }

    let mut sum: u64 = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != '*' {
                continue;
            }

            let min_x: usize = cmp::max(0, x as i64 - 1) as usize;
            let min_y: usize = cmp::max(0, y as i64 - 1) as usize;
            let max_x: usize = cmp::min(x as i64 + 1, line.len() as i64 - 1) as usize;
            let max_y: usize = cmp::min(y as i64 + 1, lines.len() as i64 - 1) as usize;

            let mut set: HashSet<i64> = HashSet::new();

            for in_x in min_x..=max_x {
                for in_y in min_y..=max_y {
                    let num = nums[in_y][in_x];
                    if num == -1 {
                        continue;
                    }
                    set.insert(num);
                }
            }

            if set.len() != 2 {
                continue;
            }

            let mut iter = set.iter();
            sum += (iter.next().unwrap() * iter.next().unwrap()) as u64;
        }
    }

    sum
}

pub fn solve(str: String) -> SolutionPair {
    let sol1: u64 = part1(&str);
    let sol2: u64 = part2(&str);

    (Solution::from(sol1), Solution::from(sol2))
}
