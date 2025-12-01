use std::collections::HashSet;

use aoclib::solution::{Solution, SolutionPair};

type XY = (i64, i64);

fn solve_distances(
    galaxies: &Vec<XY>,
    empty_rows: &HashSet<i64>,
    empty_columns: &HashSet<i64>,
    mut stretch: i64,
) -> i64 {
    stretch -= 1;

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (x_1, y_1): XY = galaxies[i];
            let (x_2, y_2): XY = galaxies[j];

            let x_min = x_1.min(x_2);
            let x_max = x_1.max(x_2);
            let y_min = y_1.min(y_2);
            let y_max = y_1.max(y_2);

            let mut x_stretch = 0;
            for x in x_min + 1..x_max {
                if empty_columns.contains(&x) {
                    x_stretch += stretch;
                }
            }

            let mut y_stretch = 0;
            for y in y_min + 1..y_max {
                if empty_rows.contains(&y) {
                    y_stretch += stretch;
                }
            }

            let abs_dx = x_max - x_min + x_stretch;
            let abs_dy = y_max - y_min + y_stretch;

            sum += abs_dx + abs_dy;
        }
    }

    sum
}

pub fn solve(str: String) -> SolutionPair {
    let map: Vec<Vec<char>> = str.lines().map(|line| line.chars().collect()).collect();

    let galaxies: Vec<XY> = map
        .iter()
        .enumerate()
        .fold(vec![], |mut galaxies, (y, row)| {
            let mut galaxies_in_row: Vec<XY> = row
                .iter()
                .enumerate()
                .filter_map(|(x, char)| {
                    if *char == '#' {
                        Some((x as i64, y as i64))
                    } else {
                        None
                    }
                })
                .collect();
            galaxies.append(&mut galaxies_in_row);
            galaxies
        });

    let rows: HashSet<i64> = map
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            if row.iter().any(|c| *c == '#') {
                None
            } else {
                Some(y as i64)
            }
        })
        .collect();

    let mut columns: HashSet<i64> = HashSet::new();
    for i in 0..map[0].len() {
        let mut is_empty = true;
        for row in &map {
            if row[i] == '#' {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            columns.insert(i as i64);
        }
    }

    let sol1_sum = solve_distances(&galaxies, &rows, &columns, 2);
    let sol2_sum = solve_distances(&galaxies, &rows, &columns, 1000000);

    (Solution::from(sol1_sum), Solution::from(sol2_sum))
}
