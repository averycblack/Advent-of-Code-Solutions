use std::collections::{HashMap, HashSet};

use aoclib::{
    lib2d::Coordinates,
    solution::{Solution, SolutionPair},
};

pub fn solve(str: String) -> SolutionPair {
    let mut nodes: HashMap<char, Vec<Coordinates>> = HashMap::new();

    for (y, l) in str.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c.is_alphanumeric() {
                nodes
                    .entry(c)
                    .or_default()
                    .push(Coordinates(x as i32, y as i32));
            }
        }
    }

    let max_size = Coordinates(
        str.lines().next().unwrap().chars().count() as i32,
        str.lines().count() as i32,
    );

    let mut antinodes: HashSet<Coordinates> = HashSet::new();
    nodes.values().for_each(|n| {
        for i in 0..n.len() {
            for j in 0..n.len() {
                if i == j {
                    continue;
                }

                let a = n[i];
                let b = n[j];
                let x_diff = a.0 - b.0;
                let y_diff = a.1 - b.1;
                let new_x = a.0 + x_diff;
                let new_y = a.1 + y_diff;

                if new_x < 0 || new_x >= max_size.0 || new_y < 0 || new_y >= max_size.1 {
                    continue;
                }

                antinodes.insert(Coordinates(new_x, new_y));
            }
        }
    });
    let sol1 = antinodes.len();

    let mut antinodes: HashSet<Coordinates> = HashSet::new();
    nodes.values().for_each(|n| {
        if n.len() > 1 {
            for &ant in n {
                antinodes.insert(ant);
            }
        }

        for i in 0..n.len() {
            for j in 0..n.len() {
                if i == j {
                    continue;
                }

                let a = n[i];
                let b = n[j];
                let mut next = Coordinates(a.0, a.1);
                let diff = Coordinates(a.0 - b.0, a.1 - b.1);

                loop {
                    next += diff;

                    if next.0 < 0 || next.0 >= max_size.0 || next.1 < 0 || next.1 >= max_size.1 {
                        break;
                    }

                    antinodes.insert(next);
                }
            }
        }
    });
    let sol2 = antinodes.len();

    (Solution::from(sol1), Solution::from(sol2))
}
