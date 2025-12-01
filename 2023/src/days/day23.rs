use std::collections::HashSet;

use aoclib::{
    lib2d::{Coordinates, EAST, Grid, NORTH, SOUTH, WEST},
    solution::{Solution, SolutionPair},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    OpenSpace,
    Obstacle,
    Slope(Coordinates),
}

fn bfs(g: &Grid<Tile>, p: Coordinates, v: &mut HashSet<Coordinates>, s: u64) -> u64 {
    let t = g.get_point(p).unwrap();
    if !v.insert(p) {
        return 0;
    }

    if p == Coordinates(g.max_size.0 - 2, g.max_size.1 - 1) {
        v.remove(&p);
        return s;
    }

    let res = match t {
        Tile::Obstacle => 0,
        Tile::Slope(d) => bfs(g, p + d, v, s + 1),
        Tile::OpenSpace => p
            .neighbors_cardinal()
            .iter()
            .map(|n| {
                let t = g.get_point(*n);
                if let None = t {
                    return 0;
                }
                let t = t.unwrap();
                if t == Tile::Obstacle {
                    return 0;
                }

                bfs(g, *n, v, s + 1)
            })
            .max()
            .unwrap(),
    };

    v.remove(&p);

    res
}

pub fn solve(str: String) -> SolutionPair {
    let grid = Grid::from_string(&str, |c| match c {
        '#' => Tile::Obstacle,
        '.' => Tile::OpenSpace,
        '>' => Tile::Slope(EAST),
        '<' => Tile::Slope(WEST),
        '^' => Tile::Slope(NORTH),
        'v' => Tile::Slope(SOUTH),
        _ => panic!(),
    });

    let mut visited = HashSet::new();
    let sol1 = bfs(&grid, Coordinates(1, 0), &mut visited, 0);

    let grid = Grid::from_string(&str, |c| match c {
        '#' => Tile::Obstacle,
        '.' => Tile::OpenSpace,
        '>' => Tile::OpenSpace,
        '<' => Tile::OpenSpace,
        '^' => Tile::OpenSpace,
        'v' => Tile::OpenSpace,
        _ => panic!(),
    });
    let mut visited = HashSet::new();
    let sol2 = bfs(&grid, Coordinates(1, 0), &mut visited, 0);
    // let sol2 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
