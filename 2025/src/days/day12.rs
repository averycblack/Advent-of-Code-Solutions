use aoclib::{
    lib2d::Coordinates,
    solution::{Solution, SolutionPair},
};

fn row_to_bits(s: &str) -> u64 {
    s.chars().fold(0u64, |mut r, c| {
        r <<= 1;
        if c == '#' {
            r |= 1;
        }
        r
    })
}

const FLIP_MASK_MSB: u64 = 0b100100100;
const FLIP_MASK_LSB: u64 = 0b001001001;

fn p1(presents: &Vec<[u64; 4]>, dims: Coordinates, state: Vec<u64>, counts: [u64; 6]) -> bool {
    if counts.iter().all(|v| *v == 0) {
        return true;
    }

    for i in 0..6 {
        if counts[i] == 0 {
            continue;
        }

        for x in 0..(dims.0 - 2) {
            for y in 0..(dims.1 - 2) {
                for dir in 0..4 {
                    let shape = presents[i][dir];
                    let row1 = (shape & 0x7) << x;
                    let row2 = ((shape >> 3) & 0x7) << x;
                    let row3 = ((shape >> 6) & 0x7) << x;
                    if state[y as usize] & row1 != 0
                        || state[(y + 1) as usize] & row2 != 0
                        || state[(y + 2) as usize] & row3 != 0
                    {
                        continue;
                    }

                    let mut counts = counts.clone();
                    counts[i] -= 1;

                    let mut state = state.clone();
                    state[y as usize] |= row1;
                    state[(y + 1) as usize] |= row2;
                    state[(y + 2) as usize] |= row3;
                    if p1(presents, dims, state, counts) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn solve(str: String) -> SolutionPair {
    let sol2 = 0;

    let parts = str.split("\n\n").collect::<Vec<_>>();
    let presents: Vec<_> = parts[0..(parts.len() - 1)]
        .iter()
        .map(|&p| {
            let row = p.lines().collect::<Vec<_>>();
            let row1 = row_to_bits(row[1]);
            let row2 = row_to_bits(row[2]);
            let row3 = row_to_bits(row[3]);

            let row1_90 = ((row1 & 4) >> 2) | ((row2 & 4) >> 1) | ((row3 & 4) << 0);
            let row2_90 = ((row1 & 2) >> 1) | ((row2 & 2) >> 0) | ((row3 & 2) << 1);
            let row3_90 = ((row1 & 1) >> 0) | ((row2 & 1) << 1) | ((row3 & 1) << 2);

            let norm = (row1) | (row2 << 3) | (row3 << 6);
            let rot90 = (row1_90) | (row2_90 << 3) | (row3_90 << 6);
            let rot180 = (row1 << 6) | (row2 << 3) | (row3);
            let rot180_msb = rot180 & FLIP_MASK_MSB;
            let rot180_lsb = rot180 & FLIP_MASK_LSB;
            let rot180 =
                (rot180 & !(FLIP_MASK_LSB | FLIP_MASK_MSB)) | (rot180_msb >> 2) | (rot180_lsb << 2);
            let rot270 = (row1_90 << 6) | (row2_90 << 3) | (row3_90);
            let rot270_msb = rot270 & FLIP_MASK_MSB;
            let rot270_lsb = rot270 & FLIP_MASK_LSB;
            let rot270 =
                (rot270 & !(FLIP_MASK_LSB | FLIP_MASK_MSB)) | (rot270_msb >> 2) | (rot270_lsb << 2);
            // println!("{:03b}\n{:03b}\n{:03b}\n", row1, row2, row3);
            // println!("{:03b}\n{:03b}\n{:03b}\n", row1_90, row2_90, row3_90);
            // println!("{:03b}\n{:03b}\n{:03b}\n", rot180 & 0b111, (rot180 >> 3) & 0b111, rot180 >> 6);
            // println!("{:03b}\n{:03b}\n{:03b}\n", rot270 & 0b111, (rot270 >> 3) & 0b111, rot270 >> 6);
            // println!("=======");
            ([norm, rot90, rot180, rot270], norm.count_ones() as u64)
        })
        .collect();

    let sol1 = parts
        .last()
        .unwrap()
        .lines()
        .filter(|&t| {
            let (area, cnt_str) = t.split_once(": ").unwrap();
            let (x, y) = area.split_once('x').unwrap();
            let max_x: u64 = x.parse().unwrap();
            let max_y: u64 = y.parse().unwrap();

            // let mut cnt = [0u64; 6];
            // cnt_str.split_ascii_whitespace().enumerate().for_each(|(i, v)| {
            //     cnt[i] = v.parse().unwrap();
            // });
            // let cnt = cnt;

            // p1(&presents, Coordinates(max_x, max_y), vec![0u64; max_y as usize], cnt)

            let total_cnt = cnt_str
                .split_ascii_whitespace()
                .enumerate()
                .map(|(i, v)| v.parse::<u64>().unwrap() * presents[i].1)
                .sum::<u64>();
            let area: u64 = max_x * max_y;

            area > total_cnt
        })
        .count();

    (Solution::from(sol1), Solution::from(sol2))
}
