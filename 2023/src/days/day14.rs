use std::collections::HashMap;

use aoclib::solution::{Solution, SolutionPair};

fn get_north_load(map: &Vec<Vec<char>>) -> usize {
    let max_x = map[0].len();
    let max_y = map.len();

    (0..max_x)
        .map(|x| {
            let mut score = 0;
            for y in 0..max_y {
                if map[y][x] == 'O' {
                    score += map.len() - y;
                }
            }

            score
        })
        .sum()
}

pub fn solve(str: String) -> SolutionPair {
    let mut map: Vec<Vec<char>> = str.lines().map(|line| line.chars().collect()).collect();
    let row_len = map[0].len();

    let sol1: usize = (0..row_len)
        .map(|x| {
            let mut cur_score = map.len();
            let mut score = 0;

            for y in 0..map.len() {
                match map[y][x] {
                    '#' => {
                        cur_score = map.len() - y - 1;
                    }
                    'O' => {
                        score += cur_score;
                        cur_score -= 1;
                    }
                    _ => {}
                }
            }

            score
        })
        .sum();

    let mut state_to_idx_map: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut cycle_history: Vec<usize> = vec![];
    state_to_idx_map.insert(map.clone(), 1);
    cycle_history.push(sol1);

    let mut first_duplicate = 0;
    let mut cycle_len = 0;

    for c in 1..1000000000 {
        (0..row_len).for_each(|x| {
            let mut cur_cube = 0;

            for y in 0..map.len() {
                match map[y][x] {
                    '#' => {
                        cur_cube = y + 1;
                    }
                    'O' => {
                        map[y][x] = '.';
                        map[cur_cube][x] = 'O';
                        cur_cube += 1;
                    }
                    _ => {}
                }
            }
        });

        for line in &mut map {
            let mut cur_cube = 0;
            for x in 0..line.len() {
                match line[x] {
                    '#' => {
                        cur_cube = x + 1;
                    }
                    'O' => {
                        line[x] = '.';
                        line[cur_cube] = 'O';
                        cur_cube += 1;
                    }
                    _ => {}
                }
            }
        }

        (0..row_len).for_each(|x| {
            let mut cur_cube = map.len() - 1;

            for y in (0..map.len()).rev() {
                match map[y][x] {
                    '#' => {
                        if y != 0 {
                            cur_cube = y - 1;
                        }
                    }
                    'O' => {
                        map[y][x] = '.';
                        map[cur_cube][x] = 'O';
                        if cur_cube != 0 {
                            cur_cube -= 1;
                        }
                    }
                    _ => {}
                }
            }
        });

        for line in &mut map {
            let mut cur_cube = row_len - 1;
            for x in (0..line.len()).rev() {
                match line[x] {
                    '#' => {
                        if x != 0 {
                            cur_cube = x - 1;
                        }
                    }
                    'O' => {
                        line[x] = '.';
                        line[cur_cube] = 'O';
                        if cur_cube != 0 {
                            cur_cube -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        match state_to_idx_map.get(&map) {
            Some(idx) => {
                first_duplicate = *idx;
                cycle_len = c - idx;
                break;
            }
            None => {
                state_to_idx_map.insert(map.clone(), c);
                let score = get_north_load(&map);
                cycle_history.push(score);
            }
        }
    }

    let sol2 = cycle_history[first_duplicate + (1000000000 - first_duplicate) % cycle_len];

    (Solution::from(sol1), Solution::from(sol2))
}
