use aoclib::solution::{Solution, SolutionPair};

fn get_digit(ch: char) -> u8 {
    ch as u8 - b'0'
}

// Old Part 1
#[allow(dead_code)]
fn p1(chars: &Vec<u8>) -> u64 {
    let mut left = 0;
    let mut right = 0;
    let mut i = 0;
    while i < chars.len() {
        if chars[i] > left && i < (chars.len() - 1) {
            left = chars[i];
            right = chars[i + 1];
        } else if chars[i] > right {
            right = chars[i];
        }
        i += 1;
    }

    (left * 10 + right) as u64
}

// Old part 2
#[allow(dead_code)]
fn p2(chars: &Vec<u8>) -> u64 {
    // [0..12] = most to least significant
    let mut vals = [0; 12];
    let mut last_repl = 0;
    for i in 0..12 {
        vals[i] = chars[i];
    }

    'next_bat: for i in 1..chars.len() {
        for val_idx in 0..(i - last_repl).min(12) {
            let num_dig = 12 - val_idx;

            if chars[i] <= vals[val_idx] || (i + num_dig) > chars.len() {
                continue;
            }

            for k in 0..num_dig {
                vals[val_idx + k] = chars[i + k];
            }

            last_repl = i - val_idx;
            continue 'next_bat;
        }
    }

    vals.iter().fold(0, |accum, v| accum * 10 + *v as u64)
}

// First time (I think?) using Rust generics
// Adapted p2
fn p3<const MAX: usize>(chars: &Vec<u8>) -> u64 {
    // [0..MAX] = most to least significant
    let mut vals = [0; MAX];
    let mut last_repl = 0;
    for i in 0..MAX {
        vals[i] = chars[i];
    }

    'next_bat: for i in 1..chars.len() {
        for val_idx in 0..(i - last_repl).min(MAX) {
            let num_dig = MAX - val_idx;

            if chars[i] <= vals[val_idx] || (i + num_dig) > chars.len() {
                continue;
            }

            for k in 0..num_dig {
                vals[val_idx + k] = chars[i + k];
            }

            last_repl = i - val_idx;
            continue 'next_bat;
        }
    }

    vals.iter().fold(0, |accum, v| accum * 10 + *v as u64)
}

pub fn solve(str: String) -> SolutionPair {
    let (sol1, sol2) = str
        .split_whitespace()
        .enumerate()
        .fold((0, 0), |(sol1, sol2), (_j, line)| {
            let chars: Vec<u8> = line.chars().map(get_digit).collect();
            (sol1 + p3::<2>(&chars), sol2 + p3::<12>(&chars))
        });

    (Solution::from(sol1), Solution::from(sol2))
}
