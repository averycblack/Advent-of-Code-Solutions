use std::collections::HashSet;

use aoclib::{
    lib2d::Coordinates,
    solution::{Solution, SolutionPair},
};

const ROOM_WIDTH: i32 = 101;
const ROOM_HEIGHT: i32 = 103;

fn parse_coordinate(str: &str) -> Coordinates {
    let (x, y) = str[2..].split_once(',').unwrap();
    Coordinates(x.parse().unwrap(), y.parse().unwrap())
}

fn simulate_p1(robot: (Coordinates, Coordinates), loops: i32) -> Coordinates {
    let (p, v) = robot;
    let end = (v * loops) + p;
    Coordinates(end.0.rem_euclid(ROOM_WIDTH), end.1.rem_euclid(ROOM_HEIGHT))
}

pub fn solve(str: String) -> SolutionPair {
    let robots: Vec<(Coordinates, Coordinates)> = str
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(' ').unwrap();
            (parse_coordinate(p), parse_coordinate(v))
        })
        .collect();

    let mut quads: [u64; 4] = [0; 4];
    for r in &robots {
        let end = simulate_p1(*r, 100);
        let mut idx = if end.0 > ROOM_WIDTH / 2 {
            1
        } else if end.0 < ROOM_WIDTH / 2 {
            0
        } else {
            continue;
        };

        if end.1 > ROOM_HEIGHT / 2 {
            idx += 2;
        } else if end.1 < ROOM_HEIGHT / 2 {
            idx += 0;
        } else {
            continue;
        }

        quads[idx] += 1;
    }

    let sol1 = quads[0] * quads[1] * quads[2] * quads[3];
    let mut sol2 = 0;
    loop {
        sol2 += 1;
        let ends: HashSet<Coordinates> = robots.iter().map(|r| simulate_p1(*r, sol2)).collect();
        if ends.len() == robots.len() {
            println!("{}", "=".repeat(ROOM_WIDTH as usize));
            println!("{} Seconds", sol2);

            for y in 0..ROOM_HEIGHT {
                for x in 0..ROOM_WIDTH {
                    if ends.contains(&Coordinates(x, y)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }

            break;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
