use aoclib::solution::{Solution, SolutionPair};

#[derive(PartialEq, Eq)]
enum Op {
    Mult,
    Add,
}

fn p1(ops: &Vec<Op>, lines: &[&str]) -> u64 {
    let lines: Vec<Vec<u64>> = lines[0..(lines.len() - 1)]
        .iter()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();

    let mut result = vec![0 as u64; lines[0].len()];
    for (idx, op) in ops.iter().enumerate() {
        if *op == Op::Mult {
            result[idx] = 1;
        }
    }

    for line in lines {
        line.iter().enumerate().for_each(|(idx, v)| {
            if ops[idx] == Op::Mult {
                result[idx] *= v;
            } else {
                result[idx] += v;
            }
        });
    }

    result.iter().sum()
}

fn p2(ops: &Vec<Op>, lines: &[&str]) -> u64 {
    let mut result = vec![0 as u64; lines[0].split_ascii_whitespace().count()];
    for (idx, op) in ops.iter().enumerate() {
        if *op == Op::Mult {
            result[idx] = 1;
        }
    }

    let lines: Vec<&[u8]> = lines[0..(lines.len() - 1)]
        .iter()
        .map(|l| l.as_bytes())
        .collect();

    let mut problem = 0;
    for col in 0..lines[0].len() {
        let mut num = 0;
        for l in &lines {
            let c = l[col];
            if c == ' ' as u8 {
                continue;
            }

            num = (num * 10) as u64 + (c - '0' as u8) as u64
        }

        // Dataset contains no zeros!
        if num == 0 {
            problem += 1;
            continue;
        }

        if ops[problem] == Op::Mult {
            result[problem] *= num;
        } else {
            result[problem] += num;
        }
    }

    result.iter().sum()
}

pub fn solve(str: String) -> SolutionPair {
    let lines: Vec<&str> = str.split('\n').collect();
    let ops: Vec<Op> = lines
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| if c == "*" { Op::Mult } else { Op::Add })
        .collect();

    let sol1 = p1(&ops, &lines);
    let sol2 = p2(&ops, &lines);

    (Solution::from(sol1), Solution::from(sol2))
}
