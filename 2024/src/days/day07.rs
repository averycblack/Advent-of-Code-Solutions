use aoclib::solution::{Solution, SolutionPair};

fn p1(ops: &[u64], res: u64, target: u64) -> bool {
    if ops.is_empty() {
        return res == target;
    }

    let add = ops.first().unwrap() + res;
    let mult = ops.first().unwrap() * res;
    p1(&ops[1..], add, target) || p1(&ops[1..], mult, target)
}

fn p2(ops: &[u64], res: u64, target: u64) -> bool {
    if ops.is_empty() {
        return res == target;
    }

    let add = ops[0] + res;
    let mult = ops[0] * res;
    let concat = (res * cheap_log10(ops[0])) + ops[0];
    p2(&ops[1..], add, target) || p2(&ops[1..], mult, target) || p2(&ops[1..], concat, target)
}

fn cheap_log10(v: u64) -> u64 {
    if v < 10 {
        return 10;
    } else if v < 100 {
        return 100;
    }
    return 1000;
}

pub fn solve(str: String) -> SolutionPair {
    let equations: Vec<(u64, Vec<u64>)> = str
        .lines()
        .map(|l| {
            let (val, operands) = l.split_once(':').unwrap();
            let val = val.parse().unwrap();
            let operands = operands
                .split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            (val, operands)
        })
        .collect();

    let sol1: u64 = equations
        .iter()
        .map(|(target, operands)| {
            if p1(&operands[1..], operands[0], *target) {
                return *target;
            } else {
                return 0;
            }
        })
        .sum();

    let sol2: u64 = equations
        .iter()
        .map(|(target, operands)| {
            if p2(&operands[1..], operands[0], *target) {
                return *target;
            } else {
                return 0;
            }
        })
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
