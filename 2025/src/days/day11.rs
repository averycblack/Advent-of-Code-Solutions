use std::collections::HashMap;

use aoclib::solution::{Solution, SolutionPair};

fn p1(machines: &HashMap<&str, Vec<&str>>, key: &str) -> u64 {
    if key == "out" {
        return 1;
    }

    machines[key].iter().map(|&o| p1(machines, o)).sum()
}

const VISIT_DAC: u32 = 1;
const VISIT_FFT: u32 = 2;

fn p2<'a>(
    machines: &'a HashMap<&str, Vec<&str>>,
    memo: &mut HashMap<(&'a str, u32), u64>,
    key: &'a str,
    mut visit: u32,
) -> u64 {
    match key {
        "out" => {
            if visit == (VISIT_DAC | VISIT_FFT) {
                return 1;
            } else {
                return 0;
            }
        }
        "fft" => visit |= VISIT_FFT,
        "dac" => visit |= VISIT_DAC,
        &_ => {}
    }

    if let Some(v) = memo.get(&(key, visit)) {
        return *v;
    }

    let res = machines[key]
        .iter()
        .map(|&o| p2(machines, memo, o, visit))
        .sum();
    memo.insert((key, visit), res);
    res
}

pub fn solve(str: String) -> SolutionPair {
    let machines: HashMap<&str, Vec<&str>> = str
        .lines()
        .map(|l| {
            let (key, vals) = l.split_once(": ").unwrap();
            let vals: Vec<&str> = vals.split_ascii_whitespace().collect();
            (key, vals)
        })
        .collect();

    let mut memo = HashMap::new();
    let sol1 = p1(&machines, "you");
    let sol2 = p2(&machines, &mut memo, "svr", 0);

    (Solution::from(sol1), Solution::from(sol2))
}
