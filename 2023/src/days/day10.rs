use std::collections::VecDeque;

use aoclib::solution::{Solution, SolutionPair};

type XY = (usize, usize);
type DELTA = (i64, i64);

const NORTH: DELTA = (0, -1);
const WEST: DELTA = (-1, 0);
const SOUTH: DELTA = (0, 1);
const EAST: DELTA = (1, 0);

const DIRS: [DELTA; 4] = [NORTH, WEST, SOUTH, EAST];

pub fn solve(str: String) -> SolutionPair {
    let map: Vec<Vec<char>> = str.lines().map(|line| line.chars().collect()).collect();

    let (start_x, start_y) = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            let x_res = line
                .iter()
                .enumerate()
                .find_map(|(x, pipe)| if *pipe == 'S' { Some(x) } else { None });

            match x_res {
                Some(x) => Some((x, y)),
                None => None,
            }
        })
        .unwrap();

    let mut score_map: Vec<Vec<u64>> = vec![vec![u64::MAX; map[0].len()]; map.len()];
    let mut queue: VecDeque<(u64, XY)> = VecDeque::new();

    score_map[start_y][start_x] = 0;

    // NORTH
    if start_y != 0 && ['|', '7', 'F'].contains(&map[start_y - 1][start_x]) {
        queue.push_back((0, (start_x, start_y - 1)));
    }

    // EAST
    if ['-', 'J', '7'].contains(&map[start_y][start_x + 1]) {
        queue.push_back((0, (start_x + 1, start_y)));
    }

    // SOUTH
    if ['|', 'L', 'J'].contains(&map[start_y + 1][start_x]) {
        queue.push_back((0, (start_x, start_y + 1)));
    }

    // WEST
    if start_x != 0 && ['-', 'L', 'F'].contains(&map[start_y][start_x - 1]) {
        queue.push_back((0, (start_x - 1, start_y)));
    }

    let mut max_steps = 0;

    while !queue.is_empty() {
        let (prev_score, (x, y)) = queue.pop_front().unwrap();

        if prev_score + 1 >= score_map[y][x] {
            continue;
        }

        max_steps = max_steps.max(prev_score + 1);
        score_map[y][x] = prev_score + 1;
        let pipe = map[y][x];
        let valid_dirs: [DELTA; 2] = match pipe {
            '|' => [NORTH, SOUTH],
            '-' => [WEST, EAST],
            'L' => [NORTH, EAST],
            'J' => [NORTH, WEST],
            '7' => [SOUTH, WEST],
            'F' => [SOUTH, EAST],
            _ => {
                panic!("Invalid pipe: {} {}", x, y)
            }
        };

        for (dx, dy) in valid_dirs {
            let new_x = ((x as i64) + dx) as usize;
            let new_y = ((y as i64) + dy) as usize;

            queue.push_back((prev_score + 1, (new_x, new_y)));
        }
    }

    let mut outside_tiles = 0;
    let mut new_map = vec![vec!['z'; map[0].len() * 2]; map.len() * 2];

    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            let new_y = y * 2;
            let new_x = x * 2;

            new_map[new_y + 1][new_x + 1] = '.';
            new_map[new_y + 1][new_x] = if ['|', '7', 'F'].contains(char)
                || (*char == 'S' && ['|', 'L', 'J'].contains(&map[y + 1][x]))
            {
                '|'
            } else {
                '.'
            };
            new_map[new_y][new_x + 1] = if ['-', 'F', 'L'].contains(char)
                || (*char == 'S' && ['-', 'J', '7'].contains(&line[x + 1]))
            {
                '-'
            } else {
                '.'
            };
            new_map[new_y][new_x] = *char;
        }
    }

    for line in new_map.iter() {
        println!("{:?}", line.iter().collect::<String>());
    }

    println!(
        "\n{}\n",
        std::iter::repeat('=')
            .take(new_map[0].len() + 2)
            .collect::<String>()
    );

    for start_y in 0..new_map.len() {
        for start_x in [0, new_map[0].len() - 1] {
            if new_map[start_y][start_x] == 'O'
                || (new_map[start_y][start_x] != '.'
                    && score_map[start_y / 2][start_x / 2] != u64::MAX)
            {
                continue;
            }

            let mut cur_tiles = 0;
            let mut queue: VecDeque<XY> = VecDeque::new();
            queue.push_back((start_x, start_y));

            while !queue.is_empty() {
                let (x, y) = queue.pop_front().unwrap();

                // usize be goofy and overflow - this check should be good enough :tm:
                // Realistically I should be used a signed type but then I need to type cast to usize a lot :(
                if x >= new_map[0].len() || y >= new_map.len() {
                    continue;
                }

                // Either want . or not O with max score
                // Can't be . or any tiles with a score
                if new_map[y][x] == 'O'
                    || (new_map[y][x] != '.' && score_map[y / 2][x / 2] != u64::MAX)
                {
                    continue;
                }

                new_map[y][x] = 'O';
                if x % 2 == 0 && y % 2 == 0 {
                    cur_tiles += 1;
                }

                for (dx, dy) in DIRS {
                    let new_x = ((x as i64) + dx) as usize;
                    let new_y = ((y as i64) + dy) as usize;

                    queue.push_back((new_x, new_y));
                }
            }

            outside_tiles += cur_tiles;
        }
    }

    for line in new_map {
        println!("{:?}", line.iter().collect::<String>());
    }
    (
        Solution::from(max_steps),
        Solution::from(map.len() * map[0].len() - (max_steps * 2 + outside_tiles) as usize),
    )
}
