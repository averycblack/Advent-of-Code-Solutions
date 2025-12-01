use aoclib::solution::{Solution, SolutionPair};

pub fn solve(str: String) -> SolutionPair {
    let mut keys: Vec<[i32; 5]> = Vec::new();
    let mut locks: Vec<[i32; 5]> = Vec::new();
    for s in str.split("\n\n") {
        let mut heights = [0; 5];
        let lines: Vec<&str> = s.lines().collect();
        if lines[0] == "#####" {
            for i in 1..6 {
                let chars: Vec<char> = lines[i].chars().collect();
                for j in 0..5 {
                    if chars[j] == '#' {
                        heights[j] = i as i32;
                    }
                }
            }
            // lock
            locks.push(heights);
        } else {
            //key
            for i in (1..6).rev() {
                let chars: Vec<char> = lines[i].chars().collect();
                for j in 0..5 {
                    if chars[j] == '#' {
                        heights[j] = 6 - i as i32;
                    }
                }
            }

            keys.push(heights);
        }
    }

    let mut sol1 = 0;
    for key in &keys {
        'next_lock: for lock in &locks {
            for i in 0..5 {
                if key[i] + lock[i] > 5 {
                    continue 'next_lock;
                }
            }

            sol1 += 1;
        }
    }

    let sol2: u64 = 2024;
    (Solution::from(sol1), Solution::from(sol2))
}
