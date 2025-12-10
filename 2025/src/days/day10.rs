use std::collections::VecDeque;

use aoclib::solution::SolutionPair;
use good_lp::{
    Expression, IntoAffineExpression, Solution, SolverModel, microlp, variable, variables,
};

pub fn solve(str: String) -> SolutionPair {
    let machines: Vec<(u32, Vec<u32>, Vec<u32>)> = str
        .lines()
        .map(|l| {
            let mut iter = l.split_ascii_whitespace();
            let v = iter
                .next()
                .unwrap()
                .chars()
                .enumerate()
                .fold(0, |accum, (i, c)| match c {
                    '[' | ']' | '.' => accum,
                    '#' => accum | (1 << (i - 1)),
                    _ => panic!(),
                });

            let mut btns = vec![];
            let mut joltage = vec![];

            for btn in iter {
                if btn.starts_with('{') {
                    btn[1..(btn.len() - 1)].split(',').for_each(|v| {
                        joltage.push(v.parse().unwrap());
                    });
                    break;
                }

                let v = btn[1..(btn.len() - 1)].split(',').fold(0, |accum, v| {
                    let shift: u32 = v.parse().unwrap();
                    accum | (1u32 << shift)
                });

                btns.push(v);
            }

            (v, btns, joltage)
        })
        .collect();

    let sol1: u64 = machines
        .iter()
        .map(|(final_state, btns, _)| {
            let mut q = VecDeque::new();
            q.push_back((0u32, 0u64));
            loop {
                let (state, pushes) = q.pop_front().unwrap();

                if state == *final_state {
                    break pushes;
                }

                for btn in btns {
                    q.push_back((state ^ btn, pushes + 1));
                }
            }
        })
        .sum();

    let sol2: u64 = machines
        .iter()
        .map(|(_, btns, jolts)| {
            let mut vars = variables!();
            // a..(num btns) >= 0
            let press_vars = (0..btns.len())
                .map(|_| vars.add(variable().min(0).integer()))
                .collect::<Vec<_>>();

            // Minimize sum(1..num btns)
            let mut problem = vars
                .minimise(press_vars.iter().sum::<Expression>())
                .using(microlp);

            // a + b + c... = jolts[i]
            let mut exprs = vec![0.into_expression(); jolts.len()];
            for i in 0..btns.len() {
                for j in 0..32 {
                    if btns[i] & (1 << j) != 0 {
                        exprs[j] += press_vars[i];
                    }
                }
            }
            for (e, &j) in exprs.into_iter().zip(jolts) {
                problem.add_constraint(e.eq(j));
            }

            // solve
            let sol = problem.solve().unwrap();
            press_vars.iter().map(|&v| sol.value(v)).sum::<f64>() as u64
        })
        .sum();

    (
        aoclib::solution::Solution::from(sol1),
        aoclib::solution::Solution::from(sol2),
    )
}
