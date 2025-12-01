use std::collections::HashSet;

use aoclib::{
    solution::{Solution, SolutionPair},
    lib2d::{CARDINALS, Coordinates, Grid, NORTH_IDX},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Obstacle,
    FreeSpace,
    Start,
}

pub fn solve(str: String) -> SolutionPair {
    let mut grid = Grid::from_string(&str, |c| match c {
        '#' => Tile::Obstacle,
        '.' => Tile::FreeSpace,
        '^' => Tile::Start,
        _ => panic!(),
    });
    let start = grid.find_one(Tile::Start).unwrap();

    let mut visited: HashSet<Coordinates> = HashSet::new();
    let mut dir = NORTH_IDX;
    let mut loc = start;
    loop {
        visited.insert(loc);
        match grid.get_point(loc + CARDINALS[dir]) {
            Some(t) => {
                if t == Tile::Obstacle {
                    dir = (dir + 1) % CARDINALS.len();
                } else {
                    loc = loc + CARDINALS[dir];
                }
            }
            None => break,
        }
    }

    let mut sol2: u64 = 0;
    for &Coordinates(x, y) in &visited {
        if Coordinates(x, y) == start {
            continue;
        }

        grid.grid[y as usize][x as usize] = Tile::Obstacle;

        let mut visited: HashSet<(Coordinates, usize)> = HashSet::new();
        let mut dir = 0;
        let mut loc = start;
        loop {
            if !visited.insert((loc, dir)) {
                sol2 += 1;
                break;
            }

            match grid.get_point(loc + CARDINALS[dir]) {
                Some(t) => {
                    if t == Tile::Obstacle {
                        dir = (dir + 1) % CARDINALS.len();
                    } else {
                        loc = loc + CARDINALS[dir];
                    }
                }
                None => break,
            }
        }

        grid.grid[y as usize][x as usize] = Tile::FreeSpace;
    }

    (Solution::from(visited.len()), Solution::from(sol2))
}
