#![feature(iter_advance_by)]

mod days;

use crate::days::day18;
use aoclib::solution::Solution;
use std::env;
use std::fs;
use std::time::Instant;

pub type SolutionPair = (Solution, Solution);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day(s) to run as a command-line argument.");
    }

    let days: Vec<u8> = if args[1] == "all" {
        (1..=25).collect()
    } else {
        args[1..]
            .iter()
            .map(|x| {
                x.parse()
                    .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
            })
            .collect()
    };

    let mut runtime = 0.0;

    for day in days {
        let func = get_day_solver(day);

        // Read in input file
        let input = fs::read_to_string(format!("./input/day{:02}.txt", day))
            .unwrap_or_else(|v| panic!("Input file not found for day {:02}\n\t{}", day, v));

        let time = Instant::now();
        let (p1, p2) = func(input);
        let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;

        println!("\n=== Day {:02} ===", day);
        println!("  · Part 1: {}", p1);
        println!("  · Part 2: {}", p2);
        println!("  · Elapsed: {:.4} ms", elapsed_ms);

        runtime += elapsed_ms;
    }

    println!("Total runtime: {:.4} ms", runtime);
}

fn get_day_solver(day: u8) -> fn(String) -> SolutionPair {
    match day {
        18 => day18::solve,
        _ => unimplemented!(),
    }
}
