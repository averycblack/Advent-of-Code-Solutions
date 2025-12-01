use aoclib::solution::{Solution, SolutionPair};

const OPTIONS: [&'static str; 19] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3",
    "4", "5", "6", "7", "8", "9",
];
const MAP: [u32; 19] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

pub fn solve(str: String) -> SolutionPair {
    let sol1: u32 = str
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            if digits.len() == 0 {
                return 0;
            }
            (digits.first().unwrap() * 10) + digits.last().unwrap()
        })
        .sum();

    let sol2: u32 = str
        .lines()
        .map(|line| {
            let mut last: u32 = 0;
            let mut first: u32 = 0;

            'outer: for i in 0..line.len() {
                for j in 0..OPTIONS.len() {
                    let opt = OPTIONS[j];
                    if i + opt.len() > line.len() {
                        continue;
                    }

                    if &line[i..(i + opt.len())] == opt {
                        first = MAP[j];
                        break 'outer;
                    }
                }
            }

            'outer: for i in (1..=line.len()).rev() {
                for o in 0..OPTIONS.len() {
                    let opt = OPTIONS[o];
                    if i < opt.len() {
                        continue;
                    }

                    if &line[(i - opt.len())..(i)] == opt {
                        last = MAP[o];
                        break 'outer;
                    }
                }
            }

            (first * 10) + last
        })
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
