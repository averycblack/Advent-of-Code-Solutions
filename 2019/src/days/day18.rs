use std::{
    collections::{HashMap, VecDeque},
    u32,
};

use aoclib::lib2d;
use aoclib::{
    lib2d::{Coordinates, Grid},
    solution::{Solution, SolutionPair},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Obstacle,
    Door(u8),
    Key(u8),
    Start,
    Empty,
}

fn p1(map: &Grid<Tile>, start: Coordinates, key_mask: u32) -> u64 {
    let mut queue: VecDeque<(Coordinates, u32, u32)> = VecDeque::new();
    let mut visisted: HashMap<(Coordinates, u32), u32> = HashMap::new();
    queue.push_back((start, 0, 0));

    while !queue.is_empty() {
        let (p, steps, keys) = queue.pop_front().unwrap();
        let last_score = visisted.get(&(p, keys)).unwrap_or(&u32::MAX);

        if steps >= *last_score {
            continue;
        }

        if keys == key_mask {
            return steps as u64;
        }

        visisted.insert((p, keys), steps);

        for n in p.neighbors_cardinal() {
            let tile = map.get_point(n);
            if tile.is_none() {
                continue;
            }

            match tile.unwrap() {
                Tile::Empty => queue.push_back((n, steps + 1, keys)),
                Tile::Key(c) => queue.push_back((n, steps + 1, keys | (1 << (c - b'a')))),
                Tile::Obstacle => continue,
                Tile::Door(c) => {
                    if keys & (1 << (c - b'A')) != 0 {
                        queue.push_back((n, steps + 1, keys))
                    }
                }
                _ => panic!(),
            }
        }
    }

    println!("{:?}", visisted);
    unreachable!()
}

fn p2(map: &Grid<Tile>, start: &[Coordinates], key_mask: u32) -> u64 {
    let mut queue: VecDeque<([Coordinates; 4], u32, u32)> = VecDeque::new();
    let mut visisted: HashMap<(Coordinates, u32), u32> = HashMap::new();
    let start = [start[0], start[1], start[2], start[3]];
    queue.push_back((start, 0, 0));

    while !queue.is_empty() {
        let (p, steps, keys) = queue.pop_front().unwrap();

        // Can only control one robot at a time
        for i in 0..4 {
            let r = p[i];
            let last_score = visisted.get(&(r, keys)).unwrap_or(&u32::MAX);

            if steps >= *last_score {
                continue;
            }

            if keys == key_mask {
                return steps as u64;
            }

            visisted.insert((r, keys), steps);

            for n in r.neighbors_cardinal() {
                let tile = map.get_point(n);
                if tile.is_none() {
                    continue;
                }

                let mut next_state = p.clone();
                next_state[i] = n;

                match tile.unwrap() {
                    Tile::Empty => queue.push_back((next_state, steps + 1, keys)),
                    Tile::Key(c) => {
                        queue.push_back((next_state, steps + 1, keys | (1 << (c - b'a'))))
                    }
                    Tile::Obstacle => continue,
                    Tile::Door(c) => {
                        if keys & (1 << (c - b'A')) != 0 {
                            queue.push_back((next_state, steps + 1, keys))
                        }
                    }
                    _ => panic!(),
                }
            }
        }
    }

    println!("{:?}", visisted);
    unreachable!()
}

pub fn solve(str: String) -> SolutionPair {
    let mut grid = Grid::from_string(&str, |c| match c {
        '#' => Tile::Obstacle,
        'a'..='z' => Tile::Key(c as u8),
        'A'..='Z' => Tile::Door(c as u8),
        '.' => Tile::Empty,
        '@' => Tile::Start,
        _ => panic!(),
    });

    let start_p1 = grid.find_one(Tile::Start).unwrap();
    let mut grid_p1 = grid.clone();
    grid_p1.set_point(start_p1, Tile::Empty);

    let mut keys = 0;
    let mut start = vec![];
    for y in 0..grid.max_size.1 {
        for x in 0..grid.max_size.0 {
            let p = Coordinates(x, y);
            let t = grid.get_point(p).unwrap();
            if let Tile::Key(k) = t {
                keys |= 1 << (k - b'a');
            } else if let Tile::Start = t {
                start.push(p + lib2d::NORTHEAST);
                start.push(p + lib2d::NORTHWEST);
                start.push(p + lib2d::SOUTHEAST);
                start.push(p + lib2d::SOUTHWEST);
                grid.set_point(p, Tile::Obstacle);
                grid.set_point(p + lib2d::WEST, Tile::Obstacle);
                grid.set_point(p + lib2d::EAST, Tile::Obstacle);
                grid.set_point(p + lib2d::NORTH, Tile::Obstacle);
                grid.set_point(p + lib2d::SOUTH, Tile::Obstacle);
            }
        }
    }

    let sol1 = p1(&grid_p1, start_p1, keys);
    let sol2 = p2(&grid, &start[0..4], keys);

    (Solution::from(sol1), Solution::from(sol2))
}
