use aoclib::solution::{Solution, SolutionPair};

pub fn solve(str: String) -> SolutionPair {
    let mut line_iter = str.lines();
    let times: Vec<u64> = line_iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let distances: Vec<u64> = line_iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let sol1: u64 = times
        .iter()
        .zip(distances.iter())
        .map(|(time, record)| {
            (1..*time)
                .filter(|speed| {
                    let time_left = time - speed;
                    time_left * speed > *record
                })
                .count() as u64
        })
        .product();

    let total_time: u64 = times.iter().fold(0, |accum, cur_val| {
        let mult = cur_val.checked_ilog10().unwrap() + 1;
        (accum * u64::pow(10, mult)) + cur_val
    });

    let total_distance: u64 = distances.iter().fold(0, |accum, cur_dis| {
        let mult = cur_dis.checked_ilog10().unwrap() + 1;
        (accum * u64::pow(10, mult)) + cur_dis
    });

    let sol2 = (1..total_time)
        .filter(|speed| {
            let time_left = total_time - speed;
            time_left * speed > total_distance
        })
        .count();

    (Solution::from(sol1), Solution::from(sol2))
}
