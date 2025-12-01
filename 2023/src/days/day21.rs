use std::collections::{HashSet, VecDeque};

use aoclib::{
    lib2d::{Coordinates, Grid},
    solution::{Solution, SolutionPair},
};

fn p1(g: &Grid<char>, p: Coordinates, s: u64) -> usize {
    let mut locs = HashSet::new();
    locs.insert(p);

    for _ in 0..s {
        let mut temp = HashSet::new();
        for l in locs {
            for n in l.neighbors_cardinal() {
                let t = g.get_point(n);
                if let None = t {
                    continue;
                }

                if t.unwrap() == '#' {
                    continue;
                }

                temp.insert(n);
            }
        }
        locs = temp;
    }

    println!("{}", "=".repeat(g.max_size.0 as usize));
    for y in 0..g.max_size.1 {
        for x in 0..g.max_size.0 {
            print!(
                "{}",
                if locs.contains(&Coordinates(x, y)) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!("");
    }

    locs.len()
}

fn p2(g: &Grid<char>, p: Coordinates) -> usize {
    let mut locs = HashSet::new();

    let mut odd = HashSet::new();
    let mut odd_corners = HashSet::new();
    let mut even = HashSet::new();
    let mut even_corners = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((p, 0));

    while !queue.is_empty() {
        let (p, s) = queue.pop_front().unwrap();
        if locs.contains(&p) {
            continue;
        }

        locs.insert(p);

        if s % 2 == 0 {
            if s > 65 {
                even_corners.insert(p);
            } else {
                even.insert(p);
            }
        } else {
            if s > 65 {
                odd_corners.insert(p);
            } else {
                odd.insert(p);
            }
        }

        for n in p.neighbors_cardinal() {
            let t = g.get_point(n);
            if let None = t {
                continue;
            }

            if t.unwrap() == '#' {
                continue;
            }

            queue.push_back((n, s + 1));
        }
    }

    let n: usize = ((26501365 - (g.max_size.0 / 2)) / g.max_size.0) as usize;
    let total_odd = odd.len() + odd_corners.len();
    let total_even = even.len() + even_corners.len();
    ((n + 1).pow(2) * total_odd) + (n * n * total_even) - ((n + 1) * odd_corners.len())
        + (n * even_corners.len())
}

pub fn solve(str: String) -> SolutionPair {
    let grid = Grid::from_string(&str, |c| c);
    let start = grid.find_one('S').unwrap();

    let sol1 = p1(&grid, start, 65);
    let sol2 = p2(&grid, start);

    (Solution::from(sol1), Solution::from(sol2))
}
