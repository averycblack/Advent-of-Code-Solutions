use std::collections::{HashSet, VecDeque};

use aoclib::{
    lib2d::{Coordinates, Grid},
    solution::{Solution, SolutionPair},
};

fn bfs(grid: &Grid<u32>, start: Coordinates) -> (u32, u32) {
    let mut queue = VecDeque::new();
    let mut summits = 0;
    let mut unique_trails = 0;
    let mut visited = HashSet::new();

    queue.push_back((start, 0));

    while !queue.is_empty() {
        let (pos, height) = queue.pop_front().unwrap();

        if height == 9 {
            unique_trails += 1;

            if !visited.contains(&pos) {
                summits += 1;
            }

            visited.insert(pos);
            continue;
        }

        for n in pos.neighbors_cardinal() {
            if let Some(next_h) = grid.get_point(n) {
                if next_h == height + 1 {
                    queue.push_back((n, next_h));
                }
            }
        }
    }

    (summits, unique_trails)
}

pub fn solve(str: String) -> SolutionPair {
    let grid = Grid::from_string(&str, |c| c.to_digit(10).unwrap());

    let Coordinates(max_x, max_y) = grid.get_max_size();
    let mut sol1 = 0;
    let mut sol2 = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            if let Some(h) = grid.get_point(Coordinates(x, y)) {
                if h == 0 {
                    let (summits, unique_trails) = bfs(&grid, Coordinates(x, y));
                    sol1 += summits;
                    sol2 += unique_trails;
                }
            }
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
