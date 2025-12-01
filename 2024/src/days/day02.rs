use aoclib::solution::{Solution, SolutionPair};

fn is_safe(r: &Vec<i32>) -> bool {
    r.is_sorted_by(|&a, &b| i32::abs_diff(a, b) <= 3 && a < b)
        || r.is_sorted_by(|&a, &b| i32::abs_diff(a, b) <= 3 && b < a)
}

pub fn solve(str: String) -> SolutionPair {
    let reports: Vec<Vec<_>> = str
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let sol1 = reports.iter().filter(|&r| is_safe(r)).count();

    let sol2 = reports
        .iter()
        .filter(|&r| {
            for i in 0..r.len() {
                let r_p = [&r[0..i], &r[(i + 1)..]].concat();
                if is_safe(&r_p) {
                    return true;
                }
            }

            false
        })
        .count();

    (Solution::from(sol1), Solution::from(sol2))
}
