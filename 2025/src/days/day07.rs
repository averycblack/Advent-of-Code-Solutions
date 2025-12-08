use aoclib::{
    lib2d::{self, Coordinates},
    solution::{Solution, SolutionPair},
};

pub fn solve(str: String) -> SolutionPair {
    let tachyon = lib2d::Grid::from_string(&str, |c| c);
    let start = tachyon.find_one('S').unwrap();
    let mut sol1 = 0;

    let max_num = tachyon.get_max_size();
    let mut beams = vec![0 as u64; max_num.0 as usize];
    beams[start.0 as usize] = 1;

    for row in 1..max_num.1 {
        let mut next_row = vec![0 as u64; max_num.0 as usize];
        for col in 0..max_num.0 {
            let c = tachyon.get_point(Coordinates(col, row)).unwrap();
            let x = col as usize;
            if c == '.' {
                next_row[x] += beams[x];
            } else {
                next_row[x - 1] += beams[x];
                next_row[x + 1] += beams[x];
                if beams[x] > 0 {
                    sol1 += 1;
                }
            }
        }
        beams = next_row;
    }

    let sol2: u64 = beams.iter().sum();

    (Solution::from(sol1), Solution::from(sol2))
}
