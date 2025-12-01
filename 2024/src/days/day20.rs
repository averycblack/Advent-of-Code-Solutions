use std::collections::{HashMap, VecDeque};

use aoclib::{
    lib2d::{Coordinates, Grid},
    solution::{Solution, SolutionPair},
};

const SAVINGS: i32 = 100;

fn create_map(g: &Grid<char>, start: Coordinates, end: Coordinates) -> HashMap<Coordinates, i32> {
    let mut map = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));

    while !q.is_empty() {
        let (p, s) = q.pop_front().unwrap();

        if map.contains_key(&p) {
            continue;
        }
        map.insert(p, s);

        if p == end {
            return map;
        }

        for n in p.neighbors_cardinal() {
            if g.get_point(n).unwrap() == '#' {
                continue;
            }

            q.push_back((n, s + 1));
        }
    }

    map
}

fn find_cheats(len: i32, scores: &HashMap<Coordinates, i32>) -> i32 {
    let mut cheats = 0;

    for (p, s) in scores {
        for (n, n_s) in scores {
            let dist = (n.0 - p.0).abs() + (n.1 - p.1).abs();
            if dist > len {
                continue;
            }

            let diff = n_s - (s + dist);
            if diff < SAVINGS {
                continue;
            }

            cheats += 1;
        }
    }

    cheats
}

pub fn solve(str: String) -> SolutionPair {
    let grid = Grid::from_string(&str, |c| c);
    let start = grid.find_one('S').unwrap();
    let end = grid.find_one('E').unwrap();

    let scores = &create_map(&grid, start, end);
    let sol1 = find_cheats(2, scores);
    let sol2 = find_cheats(20, scores);

    (Solution::from(sol1), Solution::from(sol2))
}
