use std::collections::{HashMap, HashSet};

use aoclib::{lib2d::{self, Coordinates}, solution::{Solution, SolutionPair}};

fn p1_recurse(tachyon: &lib2d::Grid<char>, pos: Coordinates, visited: &mut HashSet<Coordinates>) -> u64 {
    if visited.contains(&pos) {
        return 0;
    }

    visited.insert(pos);
    if let Some(c) = tachyon.get_point(pos) {
        if c == '.' {
            return p1_recurse(tachyon, pos + lib2d::SOUTH, visited);
        } else if c == '^' {
            return 1 + p1_recurse(tachyon, pos + lib2d::EAST, visited)
                + p1_recurse(tachyon, pos + lib2d::WEST, visited);
        } else {
            panic!();
        }
    }

    0
}

fn p2_recurse(tachyon: &lib2d::Grid<char>, pos: Coordinates, visited: &mut HashMap<Coordinates, u64>) -> u64 {
    if visited.contains_key(&pos) {
        return *visited.get(&pos).unwrap();
    }

    let mut timelines = 0;
    if let Some(c) = tachyon.get_point(pos) {
        if c == '.' {
            timelines += p2_recurse(tachyon, pos + lib2d::SOUTH, visited);
        } else if c == '^' {
            timelines += p2_recurse(tachyon, pos + lib2d::EAST, visited);
            timelines += p2_recurse(tachyon, pos + lib2d::WEST, visited);
        } else {
            panic!();
        }
    } else {
        timelines = 1;
    }

    visited.insert(pos, timelines);
    timelines
}

pub fn solve(str: String) -> SolutionPair {
    let tachyon = lib2d::Grid::from_string(&str, |c| c);
    let start = tachyon.find_one('S').unwrap();

    let mut visited = HashSet::new();
    let mut scores = HashMap::new();
    let sol1 = p1_recurse(&tachyon, start + lib2d::SOUTH, &mut visited);
    let sol2 = p2_recurse(&tachyon, start + lib2d::SOUTH, &mut scores);

    (Solution::from(sol1), Solution::from(sol2))
}
