use aoclib::solution::{Solution, SolutionPair};

pub fn solve(str: String) -> SolutionPair {
    let (sol1, sol2) = str.lines().fold((0 as i64, 0 as i64), |score_accum, line| {
        let mut last_history: Vec<i64> = line
            .split_whitespace()
            .map(|val| val.parse().unwrap())
            .collect();

        let mut last_values: Vec<i64> = vec![*last_history.last().unwrap()];
        let mut first_values: Vec<i64> = vec![*last_history.first().unwrap()];

        loop {
            let history: Vec<i64> = last_history
                .windows(2)
                .map(|vals| vals[1] - vals[0])
                .collect();

            if !history.iter().any(|val| *val != 0) {
                break;
            }

            last_values.push(*history.last().unwrap());
            first_values.push(*history.first().unwrap());
            last_history = history;
        }

        let last_pred = last_values
            .iter()
            .rev()
            .fold(0, |delta, last_value| last_value + delta);

        let first_pred = first_values
            .iter()
            .rev()
            .fold(0, |delta, last_value| last_value - delta);

        (score_accum.0 + last_pred, score_accum.1 + first_pred)
    });

    (Solution::from(sol1), Solution::from(sol2))
}
