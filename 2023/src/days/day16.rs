use std::collections::{HashSet, VecDeque};

use aoclib::solution::{Solution, SolutionPair};

type XY = (usize, usize);
type DELTA = (i64, i64);

const NORTH: DELTA = (0, -1);
const WEST: DELTA = (-1, 0);
const SOUTH: DELTA = (0, 1);
const EAST: DELTA = (1, 0);

fn go_dir<'a>(
    queue: &mut VecDeque<(&'a DELTA, XY)>,
    map: &Vec<Vec<char>>,
    dir: &'a DELTA,
    pos: XY,
) {
    let (x, y) = pos;
    let new_x = ((x as i64) + dir.0) as usize;
    let new_y = ((y as i64) + dir.1) as usize;

    if new_x < map[0].len() && new_y < map.len() {
        queue.push_back((dir, (new_x, new_y)));
    }
}

fn get_energy(map: &Vec<Vec<char>>, start_pos: XY, start_dir: &DELTA) -> usize {
    let mut visited_map = vec![vec!['.'; map[0].len()]; map.len()];
    let mut queue: VecDeque<(&DELTA, XY)> = VecDeque::new();
    let mut visited: HashSet<(&DELTA, XY)> = HashSet::new();

    queue.push_back((start_dir, start_pos));

    while !queue.is_empty() {
        let (dir, (x, y)) = queue.pop_front().unwrap();

        if visited.contains(&(dir, (x, y))) {
            continue;
        }

        visited.insert((dir, (x, y)));
        visited_map[y][x] = '#';
        let char = map[y][x];

        match char {
            '.' => {
                go_dir(&mut queue, &map, dir, (x, y));
            }
            '|' => match dir {
                &NORTH | &SOUTH => go_dir(&mut queue, &map, dir, (x, y)),
                _ => {
                    go_dir(&mut queue, &map, &NORTH, (x, y));
                    go_dir(&mut queue, &map, &SOUTH, (x, y));
                }
            },
            '-' => match dir {
                &EAST | &WEST => go_dir(&mut queue, &map, dir, (x, y)),
                _ => {
                    go_dir(&mut queue, &map, &EAST, (x, y));
                    go_dir(&mut queue, &map, &WEST, (x, y));
                }
            },
            '/' => match dir {
                &EAST => go_dir(&mut queue, &map, &NORTH, (x, y)),
                &WEST => go_dir(&mut queue, &map, &SOUTH, (x, y)),
                &NORTH => go_dir(&mut queue, &map, &EAST, (x, y)),
                &SOUTH => go_dir(&mut queue, &map, &WEST, (x, y)),
                _ => panic!("Invalid direction!"),
            },
            '\\' => match dir {
                &EAST => go_dir(&mut queue, &map, &SOUTH, (x, y)),
                &WEST => go_dir(&mut queue, &map, &NORTH, (x, y)),
                &NORTH => go_dir(&mut queue, &map, &WEST, (x, y)),
                &SOUTH => go_dir(&mut queue, &map, &EAST, (x, y)),
                _ => panic!("Invalid direction!"),
            },
            _ => {}
        }
    }

    let ret: usize = visited_map
        .iter()
        .map(|line| line.iter().filter(|c| **c == '#').count())
        .sum();
    for line in &visited_map {
        println!("{}", line.iter().collect::<String>());
    }

    println!("{}\n", ret);

    ret
}

pub fn solve(str: String) -> SolutionPair {
    let map: Vec<Vec<char>> = str.lines().map(|line| line.chars().collect()).collect();

    let sol1 = get_energy(&map, (0, 0), &EAST);

    let mut sol2 = sol1;

    let max_x = map[0].len();
    let max_y = map.len();

    // top row
    for i in 0..max_x {
        sol2 = sol2.max(get_energy(&map, (i, 0), &SOUTH));
        sol2 = sol2.max(get_energy(&map, (i, max_y - 1), &NORTH));
    }

    for i in 0..max_y {
        sol2 = sol2.max(get_energy(&map, (0, i), &EAST));
        sol2 = sol2.max(get_energy(&map, (max_x - 1, i), &WEST));
    }

    (Solution::from(sol1), Solution::from(sol2))
}
