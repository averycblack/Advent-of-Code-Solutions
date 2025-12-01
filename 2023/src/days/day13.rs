use aoclib::solution::{Solution, SolutionPair};

fn is_vertical_reflection(lines: &Vec<Vec<char>>, left_idx: usize) -> bool {
    let mut left_idx = left_idx;
    let mut right_idx = left_idx + 1;

    while right_idx < lines[0].len() {
        for line in lines {
            if line[left_idx] != line[right_idx] {
                return false;
            }
        }

        // Account for overflow (ew)
        if left_idx == 0 {
            break;
        }

        left_idx -= 1;
        right_idx += 1;
    }

    true
}

fn is_horizontal_reflection(lines: &Vec<Vec<char>>, top_idx: usize) -> bool {
    let mut top_idx = top_idx;
    let mut bot_idx = top_idx + 1;

    while bot_idx < lines.len() {
        let top_line = &lines[top_idx];
        let bot_line = &lines[bot_idx];

        if top_line.iter().zip(bot_line).any(|(a, b)| a != b) {
            return false;
        }

        // Account for overflow (ew)
        if top_idx == 0 {
            break;
        }

        top_idx -= 1;
        bot_idx += 1;
    }

    true
}

fn get_score(lines: &Vec<Vec<char>>, orig_score: usize) -> Option<usize> {
    for left_idx in 0..lines[0].len() - 1 {
        if is_vertical_reflection(&lines, left_idx) {
            let score = left_idx + 1;
            if score != orig_score {
                return Some(score);
            }
        }
    }

    for top_idx in 0..lines.len() - 1 {
        if is_horizontal_reflection(&lines, top_idx) {
            let score = 100 * (top_idx + 1);
            if score != orig_score {
                return Some(score);
            }
        }
    }

    None
}

pub fn solve(str: String) -> SolutionPair {
    let original_scores: Vec<usize> = str
        .split("\n\n")
        .map(|map| {
            let lines: Vec<Vec<char>> = map.lines().map(|line| line.chars().collect()).collect();
            let score = get_score(&lines, usize::MAX);

            if score == None {
                panic!("No reflection found!");
            }

            score.unwrap()
        })
        .collect();

    let sol2: usize = str
        .split("\n\n")
        .enumerate()
        .map(|(score_idx, map)| {
            let mut lines: Vec<Vec<char>> =
                map.lines().map(|line| line.chars().collect()).collect();

            let original_score = original_scores[score_idx];

            for y in 0..lines.len() {
                let line_len = lines[y].len();
                for x in 0..line_len {
                    if lines[y][x] != '#' {
                        continue;
                    }

                    lines[y][x] = '.';
                    let score = get_score(&lines, original_score);
                    lines[y][x] = '#';

                    if score != None {
                        return score.unwrap();
                    }
                }
            }

            panic!("No alternate reflection found!");
        })
        .sum();

    let sol1: usize = original_scores.iter().sum();
    (Solution::from(sol1), Solution::from(sol2))
}
