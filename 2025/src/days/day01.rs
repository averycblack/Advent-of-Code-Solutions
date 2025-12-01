use aoclib::solution::{Solution, SolutionPair};

pub fn solve(str: String) -> SolutionPair {
    let mut ptr: i32 = 50;
    let mut sol1 = 0;
    let mut sol2 = 0;

    for line in str.lines() {
        let dir = line.chars().next().unwrap();
        let count: i32 = line[1..].parse().unwrap();
        let count = count * (if dir == 'L' { -1 } else { 1 });

        if (ptr + count) <= 0 && ptr != 0 {
            sol2 += 1;
        }
        sol2 += (ptr + count).abs() / 100;
        ptr = (ptr + count).rem_euclid(100);

        if ptr == 0 {
            sol1 += 1;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
