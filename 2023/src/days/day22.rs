use std::collections::{HashMap, HashSet};

use aoclib::solution::{Solution, SolutionPair};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coordinate3D(i64, i64, i64);

impl PartialOrd for Coordinate3D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coordinate3D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.cmp(&other.2)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cube(Coordinate3D, Coordinate3D);

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cube {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

fn parse_3d(s: &str) -> Coordinate3D {
    let p: Vec<i64> = s.split(',').map(|s| s.parse().unwrap()).collect();
    Coordinate3D(p[0], p[1], p[2])
}

fn num_cubes_supporting(m: &HashMap<Coordinate3D, usize>, cubes: &Vec<Cube>, i: usize) -> usize {
    let c = cubes[i];
    let z = c.0.2 - 1;
    let mut res = HashSet::new();
    for x in c.0.0..=c.1.0 {
        for y in c.0.1..=c.1.1 {
            if m.contains_key(&Coordinate3D(x, y, z)) {
                res.insert(m.get(&Coordinate3D(x, y, z)).unwrap());
            }
        }
    }

    res.len()
}

fn make_cubes_fall(cubes: &mut Vec<Cube>) -> (HashMap<Coordinate3D, usize>, usize) {
    let mut filled = HashMap::new();
    let mut moved = 0;

    for (i, c) in cubes.iter_mut().enumerate() {
        let mut new_c = *c;
        'outer: loop {
            if new_c.0.2 == 1 {
                break 'outer;
            }

            for x in new_c.0.0..=new_c.1.0 {
                for y in new_c.0.1..=new_c.1.1 {
                    for z in new_c.0.2..=new_c.1.2 {
                        if filled.contains_key(&Coordinate3D(x, y, z - 1)) {
                            // c.0.2 += 1;
                            // c.1.2 += 1;
                            break 'outer;
                        }
                    }
                }
            }

            new_c.0.2 -= 1;
            new_c.1.2 -= 1;
        }

        if new_c != *c {
            moved += 1;
        }

        *c = new_c;

        for x in c.0.0..=c.1.0 {
            for y in c.0.1..=c.1.1 {
                for z in c.0.2..=c.1.2 {
                    filled.insert(Coordinate3D(x, y, z), i);
                }
            }
        }
    }

    (filled, moved)
}

pub fn solve(str: String) -> SolutionPair {
    let mut cubes: Vec<Cube> = str
        .lines()
        .map(|l| {
            let (one, two) = l.split_once('~').unwrap();
            Cube(parse_3d(one), parse_3d(two))
        })
        .collect();

    cubes.sort();
    let (filled, _) = make_cubes_fall(&mut cubes);

    let mut sol1 = 0;
    'next_cube: for c in &cubes {
        let z = c.1.2 + 1;
        for x in c.0.0..=c.1.0 {
            for y in c.0.1..=c.1.1 {
                if filled.contains_key(&Coordinate3D(x, y, z))
                    && num_cubes_supporting(
                        &filled,
                        &cubes,
                        *filled.get(&Coordinate3D(x, y, z)).unwrap(),
                    ) == 1
                {
                    continue 'next_cube;
                }
            }
        }

        sol1 += 1;
    }

    let mut sol2 = 0;
    for i in 0..cubes.len() {
        let mut cubes = [&cubes[0..i], &cubes[(i + 1)..]].concat();
        let (_, moved) = make_cubes_fall(&mut cubes);
        sol2 += moved;
    }

    (Solution::from(sol1), Solution::from(sol2))
}
