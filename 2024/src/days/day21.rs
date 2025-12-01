use std::{collections::HashMap, i32};

use aoclib::{
    lib2d::{Coordinates, EAST, NORTH, SOUTH, WEST},
    solution::{Solution, SolutionPair},
};

fn get_coord_from_numpad(c: char) -> Coordinates {
    match c {
        'A' => Coordinates(2, 3),
        '0' => Coordinates(1, 3),
        '1' => Coordinates(0, 2),
        '2' => Coordinates(1, 2),
        '3' => Coordinates(2, 2),
        '4' => Coordinates(0, 1),
        '5' => Coordinates(1, 1),
        '6' => Coordinates(2, 1),
        '7' => Coordinates(0, 0),
        '8' => Coordinates(1, 0),
        '9' => Coordinates(2, 0),
        _ => panic!(),
    }
}

fn get_coord_from_dirpad(dir: Coordinates) -> Option<Coordinates> {
    match dir {
        WEST => Some(Coordinates(0, 1)),
        SOUTH => Some(Coordinates(1, 1)),
        EAST => Some(Coordinates(2, 1)),
        NORTH => Some(Coordinates(1, 0)),
        A => Some(Coordinates(2, 0)),
        _ => None,
    }
}

const A: Coordinates = Coordinates(i32::MAX, i32::MAX);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pad {
    NumPad,
    DirPad,
}

/**
 * Solution derived from AllanTaylor314:
 * https://www.reddit.com/r/adventofcode/comments/1hj2odw/comment/m33qs9d
 *
 *   ^ A
 * < v >
 *
 * 7 8 9
 * 4 5 6
 * 1 2 3
 *   0 A
 */

fn p1_step(s: Coordinates, t: Coordinates, pad: Pad) -> Vec<Coordinates> {
    let diff = t - s;

    let mut vert = vec![SOUTH * diff.1.signum(); diff.1.abs() as usize];
    let mut horiz = vec![EAST * diff.0.signum(); diff.0.abs() as usize];

    // if diff_x > 0 && (target_y, source_x) in pad
    //    vert then horiz
    // if (source_y, target_x) in pad
    //    horiz then vert
    // vert then horiz

    if pad == Pad::DirPad {
        if diff.0 > 0 && s.0 != 0 {
            vert.append(&mut horiz);
            vert.push(A);
            return vert;
        } else if !(s.1 == 0 && t.0 == 0) {
            horiz.append(&mut vert);
            horiz.push(A);
            return horiz;
        } else {
            vert.append(&mut horiz);
            vert.push(A);
            return vert;
        }
    } else {
        if diff.0 > 0 && !(t.1 == 3 && s.0 == 0) {
            vert.append(&mut horiz);
            vert.push(A);
            return vert;
        } else if !(t.0 == 0 && s.1 == 3) {
            horiz.append(&mut vert);
            horiz.push(A);
            return horiz;
        }

        vert.append(&mut horiz);
        vert.push(A);
        return vert;
    }
}

fn p1(r: &Vec<Coordinates>, pad: Pad) -> Vec<Coordinates> {
    let mut res = vec![];
    let mut last = if pad == Pad::DirPad {
        get_coord_from_dirpad(A).unwrap()
    } else {
        get_coord_from_numpad('A')
    };
    for &n in r {
        res.append(&mut p1_step(last, n, pad));
        last = n;
    }

    return res;
}

fn p2(r: &Vec<Coordinates>, pad: Pad) -> HashMap<Vec<Coordinates>, i64> {
    let mut res = HashMap::new();
    let mut last = if pad == Pad::DirPad {
        get_coord_from_dirpad(A).unwrap()
    } else {
        get_coord_from_numpad('A')
    };
    for &n in r {
        let v = p1_step(last, n, pad);
        res.entry(v).and_modify(|v| *v += 1).or_insert(1_i64);
        last = n;
    }

    return res;
}

pub fn solve(str: String) -> SolutionPair {
    let codes: Vec<(i64, Vec<Coordinates>)> = str
        .lines()
        .map(|l| {
            (
                l[0..3].parse().unwrap(),
                l.chars().map(|c| get_coord_from_numpad(c)).collect(),
            )
        })
        .collect();

    let sol1: i64 = codes
        .iter()
        .map(|c| {
            let numpad = p1(&c.1, Pad::NumPad);
            let numpad: Vec<Coordinates> = numpad
                .iter()
                .map(|d| get_coord_from_dirpad(*d).unwrap())
                .collect();
            let one = p1(&numpad, Pad::DirPad);
            let one: Vec<Coordinates> = one
                .iter()
                .map(|d| get_coord_from_dirpad(*d).unwrap())
                .collect();
            let two = p1(&one, Pad::DirPad);

            for &del in &two {
                match del {
                    NORTH => print!("^"),
                    EAST => print!(">"),
                    SOUTH => print!("v"),
                    WEST => print!("<"),
                    A => print!("A"),
                    _ => panic!(),
                }
            }
            println!("");

            println!("{} {}", c.0, two.len());
            c.0 * two.len() as i64
        })
        .sum();

    let sol2: i64 = codes
        .iter()
        .map(|c| {
            let numpad = p2(&c.1, Pad::NumPad);

            let mut dirs = numpad;
            for _ in 0..25 {
                let mut new_dirs = HashMap::new();
                for (subset, count) in dirs {
                    let subset = subset
                        .iter()
                        .map(|d| get_coord_from_dirpad(*d).unwrap())
                        .collect();
                    let dirpad = p2(&subset, Pad::DirPad);
                    for (r, v) in dirpad {
                        new_dirs
                            .entry(r)
                            .and_modify(|e| *e += v * count)
                            .or_insert(v * count);
                    }
                }
                dirs = new_dirs;
            }

            c.0 * dirs
                .iter()
                .map(|(subset, count)| subset.len() as i64 * *count)
                .sum::<i64>()
        })
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}

// pub fn solve(str: String) -> SolutionPair {
//     let sol2: u64 = 0;

//     let start = get_coord_from_num('A');
//     let codes: Vec<(i32, Vec<Coordinates>)> = str.lines().map(|l| {
//         (l[0..3].parse().unwrap(), l.chars().map(|c| get_coord_from_num(c)).collect())
//     }).collect();

//     let mut sol1 = 0;
//     for (v, code) in &codes {
//         let (s_1, d_1) = get_least_turns_path(start, WEST, &code, get_score_for_a(WEST), vec![]);
//         let (s_2, d_2) = get_least_turns_path(start, NORTH, &code, get_score_for_a(NORTH), vec![]);

//         let (_s, d) = if s_1 < s_2 {
//             (s_1, d_1)
//         } else {
//             (s_2, d_2)
//         };

//         for &del in &d {
//             match del {
//                 NORTH => print!("^"),
//                 EAST => print!(">"),
//                 SOUTH => print!("v"),
//                 WEST => print!("<"),
//                 A => print!("A"),
//                 _ => panic!()
//             }
//         }
//         println!("");

//         let d = mutate_paths(d);
//         for &del in &d {
//             match del {
//                 NORTH => print!("^"),
//                 EAST => print!(">"),
//                 SOUTH => print!("v"),
//                 WEST => print!("<"),
//                 A => print!("A"),
//                 _ => panic!()
//             }
//         }
//         println!("");

//         let d = mutate_paths(d);
//         for &del in &d {
//             match del {
//                 NORTH => print!("^"),
//                 EAST => print!(">"),
//                 SOUTH => print!("v"),
//                 WEST => print!("<"),
//                 A => print!("A"),
//                 _ => panic!()
//             }
//         }
//         println!("");
//         println!("{} * {}", d.len(), v);

//         sol1 += d.len() as i32 * v;
//     }

//     (Solution::from(sol1), Solution::from(sol2))
// }

// fn get_score_for_dir(cur: Coordinates, next: Coordinates) -> i32 {
//     if cur == next {
//         return get_score_for_a(next);
//     }

//     if cur == SOUTH {
//         return get_score_for_a(next) + 1;
//     }

//     if next == SOUTH {
//         return get_score_for_a(next) +  1 + (1 * get_score_for_a(next));
//     }

//     2 + (1 * get_score_for_a(next))
// }

// fn get_score_for_a(cur: Coordinates) -> i32 {
//     match cur {
//         NORTH => 1,
//         EAST => 1,
//         SOUTH => 2,
//         WEST => 3,
//         _ => panic!()
//     }
// }

// fn get_least_turns_path(pos: Coordinates, dir: Coordinates, mut next: &[Coordinates], mut score: i32, mut dirs: Vec<Coordinates>) -> (i32, Vec<Coordinates>) {
//     let mut is_end = false;
//     if pos == next[0] {
//         score += get_score_for_a(dir);
//         is_end = true;
//         next = &next[1..];
//         dirs.push(A);
//     }

//     if next.len() == 0 {
//         return (score, dirs);
//     }

//     let diff = next[0] - pos;
//     let mut sol = i32::MAX;
//     let mut next_dirs = dirs.clone();
//     for d in CARDINALS {
//         if (d.0 != 0 && d.0.signum() == diff.0.signum()) ||
//             (d.1 != 0 && d.1.signum() == diff.1.signum())
//         {
//             let mut new_score = score + get_score_for_dir(dir, d);
//             if is_end {
//                 new_score = score + get_score_for_a(d);
//             }

//             let mut new_dirs = dirs.clone();
//             new_dirs.push(d);
//             let (s, d) = get_least_turns_path(
//                 pos + d,
//                 d,
//                 next,
//                 new_score,
//                 new_dirs
//             );

//             if s < sol {
//                 sol = s;
//                 next_dirs = d;
//             }
//         }
//     }

//     (sol, next_dirs)
// }

// fn mutate_paths(dirs: Vec<Coordinates>) -> Vec<Coordinates> {
//     let mut res = vec![];
//     let mut last_dir = A;
//     for d in dirs {
//         // Anatomy of going a direction
//         // Move arm
//         // Press A

//         if last_dir == d {
//             res.push(A);
//             continue;
//         }

//         match d {
//             NORTH => {
//                 match last_dir {
//                     NORTH => {},
//                     EAST => { res.push(WEST); res.push(NORTH); },
//                     WEST => { res.push(EAST); res.push(NORTH); },
//                     SOUTH => { res.push(NORTH); }
//                     A => { res.push(WEST); }
//                     _ => panic!()
//                 }
//             },
//             SOUTH => {
//                 match last_dir {
//                     NORTH => { res.push(SOUTH); },
//                     EAST => { res.push(WEST); },
//                     WEST => { res.push(EAST); },
//                     SOUTH => {},
//                     A => { res.push(WEST); res.push(SOUTH); },
//                     _ => panic!()
//                 }
//             },
//             EAST => {
//                 match last_dir {
//                     NORTH => { res.push(SOUTH); res.push(EAST) },
//                     EAST => { },
//                     WEST => { res.push(EAST); res.push(EAST); },
//                     SOUTH => { res.push(EAST); },
//                     A => { res.push(SOUTH); },
//                     _ => panic!()
//                 }
//             },
//             WEST => {
//                 match last_dir {
//                     NORTH => { res.push(SOUTH); res.push(WEST); },
//                     EAST => { res.push(WEST); res.push(WEST); },
//                     WEST => {},
//                     SOUTH => { res.push(WEST); },
//                     A => { res.push(SOUTH); res.push(WEST); res.push(WEST); },
//                     _ => panic!()
//                 }
//             },
//             A => {
//                 match last_dir {
//                     NORTH => { res.push(EAST); },
//                     EAST => { res.push(NORTH); },
//                     WEST => { res.push(EAST); res.push(EAST); res.push(NORTH); },
//                     SOUTH => { res.push(EAST); res.push(NORTH); },
//                     A => {},
//                     _ => panic!()
//                 }
//             },
//             _ => panic!()
//         }

//         res.push(A);

//         last_dir = d;
//     }

//     res
// }
