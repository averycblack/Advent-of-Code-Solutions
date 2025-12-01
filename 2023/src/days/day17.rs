use std::collections::{HashMap, VecDeque};

use aoclib::{
    lib2d::{CARDINALS, Coordinates, Grid},
    solution::{Solution, SolutionPair},
};

fn p1(grid: &Grid<u32>) -> u32 {
    let mut queue = VecDeque::new();
    let mut scores = HashMap::new();

    queue.push_back((Coordinates(1, 0), 0, 1, 1));
    queue.push_back((Coordinates(0, 1), 0, 2, 1));

    let mut sol1 = u32::max_value();

    while !queue.is_empty() {
        let (loc, heat, dir, dist) = queue.pop_front().unwrap();

        let loc_heat = match grid.get_point(loc) {
            None => continue,
            Some(h) => heat + h,
        };

        if loc == Coordinates(grid.max_size.0 - 1, grid.max_size.1 - 1) {
            sol1 = sol1.min(loc_heat);
            continue;
        }

        if let Some(h) = scores.get(&(loc, dir, dist)) {
            if loc_heat >= *h {
                continue;
            }
        }

        scores.insert((loc, dir, dist), loc_heat);

        let b_dir = (dir + 3) % 4;
        let c_dir = (dir + 1) % 4;
        if dist < 3 {
            queue.push_back((loc + CARDINALS[dir], loc_heat, dir, dist + 1));
        }
        queue.push_back((loc + CARDINALS[b_dir], loc_heat, b_dir, 1));
        queue.push_back((loc + CARDINALS[c_dir], loc_heat, c_dir, 1));
    }

    sol1
}

fn p2(grid: &Grid<u32>) -> u32 {
    let mut queue = VecDeque::new();
    let mut scores = HashMap::new();

    queue.push_back((Coordinates(1, 0), 0, 1, 1));
    queue.push_back((Coordinates(0, 1), 0, 2, 1));

    let mut sol1 = u32::max_value();

    while !queue.is_empty() {
        let (loc, heat, dir, dist) = queue.pop_front().unwrap();

        let loc_heat = match grid.get_point(loc) {
            None => continue,
            Some(h) => heat + h,
        };

        if loc == Coordinates(grid.max_size.0 - 1, grid.max_size.1 - 1) && dist > 3 {
            sol1 = sol1.min(loc_heat);
            continue;
        }

        if let Some(h) = scores.get(&(loc, dir, dist)) {
            if loc_heat >= *h {
                continue;
            }
        }

        scores.insert((loc, dir, dist), loc_heat);

        if dist < 10 {
            queue.push_back((loc + CARDINALS[dir], loc_heat, dir, dist + 1));
        }

        if dist > 3 {
            let b_dir = (dir + 3) % 4;
            let c_dir = (dir + 1) % 4;
            queue.push_back((loc + CARDINALS[b_dir], loc_heat, b_dir, 1));
            queue.push_back((loc + CARDINALS[c_dir], loc_heat, c_dir, 1));
        }
    }

    sol1
}

pub fn solve(str: String) -> SolutionPair {
    let grid = Grid::from_string(&str, |c| c.to_digit(10).unwrap());

    let sol1 = p1(&grid);
    let sol2 = p2(&grid);

    (Solution::from(sol1), Solution::from(sol2))
}
