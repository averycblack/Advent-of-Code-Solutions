use std::collections::{HashSet, VecDeque};

use aoclib::{
    lib2d::Coordinates,
    solution::{Solution, SolutionPair},
};

const MAX_WIDTH: i32 = 70;
const MAX_HEIGHT: i32 = 70;
const BYTES: i32 = 1024;
// const MAX_WIDTH: i32 = 6;
// const MAX_HEIGHT: i32 = 6;
// const BYTES: i32 = 12;

fn bfs(corrupted: &HashSet<Coordinates>) -> Option<i32> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((Coordinates(0, 0), 0));

    while !queue.is_empty() {
        let (p, s) = queue.pop_front().unwrap();
        if p == Coordinates(MAX_WIDTH, MAX_HEIGHT) {
            return Some(s);
        }

        if visited.contains(&p) {
            continue;
        }

        visited.insert(p);

        let neighbors = p.neighbors_cardinal();

        for n in neighbors {
            if n.0 < 0 || n.0 > MAX_WIDTH || n.1 < 0 || n.1 > MAX_HEIGHT {
                continue;
            }

            if corrupted.contains(&n) {
                continue;
            }

            queue.push_back((n, s + 1));
        }
    }

    None
}

pub fn solve(str: String) -> SolutionPair {
    let bytes: Vec<Coordinates> = str
        .lines()
        .map(|v| {
            let (x, y) = v.split_once(',').unwrap();
            Coordinates(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut corrupted = HashSet::new();
    for i in 0..BYTES {
        corrupted.insert(bytes[i as usize]);
    }

    let sol1 = bfs(&corrupted).unwrap();
    let mut sol2 = "".to_string();

    for i in BYTES..bytes.len() as i32 {
        corrupted.insert(bytes[i as usize]);
        if let None = bfs(&corrupted) {
            let c = bytes[i as usize];
            sol2 = format!("{},{}", c.0, c.1);
            break;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
