use aoclib::{
    solution::{Solution, SolutionPair},
    lib2d::{Coordinates, DIRS, Grid, NORTHEAST, NORTHWEST, SOUTHEAST, SOUTHWEST},
};

fn find_xmas(grid: &Grid<char>, start: Coordinates) -> i32 {
    let mut count: i32 = 0;

    'delta: for d in DIRS {
        let mut cur = start;
        for c in "MAS".chars() {
            cur += d;
            let cur_c = match grid.get_point(cur) {
                Some(c) => c,
                None => {
                    continue 'delta;
                }
            };

            if cur_c != c {
                continue 'delta;
            }
        }

        count += 1;
    }

    count
}

fn check_x_mas_pair(grid: &Grid<char>, cor1: Coordinates, cor2: Coordinates) -> bool {
    let r1 = grid.get_point(cor1);
    let r2 = grid.get_point(cor2);
    if let None = r1 {
        return false;
    }

    if let None = r2 {
        return false;
    }

    let r1 = r1.unwrap();
    let r2 = r2.unwrap();
    (r1 == 'M' && r2 == 'S') || (r1 == 'S' && r2 == 'M')
}

fn find_x_mas(grid: &Grid<char>, start: Coordinates) -> bool {
    check_x_mas_pair(grid, start + NORTHWEST, start + SOUTHEAST)
        && check_x_mas_pair(grid, start + NORTHEAST, start + SOUTHWEST)
}

pub fn solve(str: String) -> SolutionPair {
    let mut sol1: i32 = 0;
    let mut sol2: u64 = 0;

    let grid = Grid::from_string(&str, |c| c);
    let Coordinates(max_y, max_x) = grid.max_size;

    for y in 0..max_y {
        for x in 0..max_x {
            let c = match grid.get_point(Coordinates(x, y)) {
                Some(c) => c,
                None => {
                    continue;
                }
            };

            if c != 'X' {
                continue;
            }

            sol1 += find_xmas(&grid, Coordinates(x, y));
        }
    }

    for y in 0..max_y {
        for x in 0..max_x {
            let c = match grid.get_point(Coordinates(x, y)) {
                Some(c) => c,
                None => {
                    continue;
                }
            };

            if c != 'A' || !find_x_mas(&grid, Coordinates(x, y)) {
                continue;
            }

            sol2 += 1;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
