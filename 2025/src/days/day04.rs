use aoclib::{
    lib2d::{self, Grid},
    solution::{Solution, SolutionPair},
};

fn iter(grid: &Grid<char>) -> (u64, Grid<char>) {
    let mut next = grid.clone();
    let mut count = 0;

    for (pt, val) in grid.iter() {
        if val != '@' {
            continue;
        }

        let rolls = lib2d::DIRS
            .iter()
            .filter(|del| grid.get_point(pt + **del).unwrap_or('.') == '@')
            .count();

        if rolls < 4 {
            count += 1;
            next.set_point(pt, '.');
        }
    }

    (count, next)
}

pub fn solve(str: String) -> SolutionPair {
    let mut sol2 = 0;

    let grid = Grid::from_string(&str, |c| c);
    let (sol1, _) = iter(&grid);

    let mut cur_grid = grid;
    loop {
        let (cnt, next) = iter(&cur_grid);
        sol2 += cnt;

        if cnt == 0 {
            break;
        }

        cur_grid = next;
    }

    (Solution::from(sol1), Solution::from(sol2))
}
