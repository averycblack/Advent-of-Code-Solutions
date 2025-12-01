use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

use aoclib::{
    lib2d::{CARDINALS, Coordinates, Grid},
    solution::{Solution, SolutionPair},
};

fn p1(grid: &Grid<char>, start: Coordinates) -> (usize, HashMap<(Coordinates, usize), usize>) {
    let mut queue = VecDeque::new();
    let mut scores: HashMap<(Coordinates, usize), usize> = HashMap::new();
    queue.push_back((start, 1, 0));

    let mut min_score = usize::MAX;

    while !queue.is_empty() {
        let (loc, dir, score) = queue.pop_front().unwrap();

        let delta = CARDINALS[dir];

        let loc_score = *scores.get(&(loc, dir)).unwrap_or(&usize::MAX);
        if score >= loc_score {
            continue;
        }

        queue.push_back((loc, (dir + 1) % CARDINALS.len(), score + 1000));
        queue.push_back((loc, (dir + 3) % CARDINALS.len(), score + 1000));
        scores.insert((loc, dir), score);

        let next = grid.get_point(loc + delta).unwrap();
        if next == '#' {
            continue;
        }

        if next == 'E' {
            min_score = min_score.min(score + 1);
            scores.insert((loc + delta, dir), score + 1);
            continue;
        }

        queue.push_back((loc + delta, dir, score + 1));
    }

    (min_score, scores)
}

fn p2(end: Coordinates, end_score: usize, scores: &HashMap<(Coordinates, usize), usize>) -> usize {
    let mut queue = VecDeque::new();

    for i in 0..4 {
        if *scores.get(&(end, i)).unwrap_or(&usize::MAX) == end_score {
            queue.push_back((end_score, end, i));
        }
    }

    let mut squares = 0;
    let mut counted = HashSet::new();
    while !queue.is_empty() {
        let (score, loc, dir) = queue.pop_front().unwrap();
        if *scores.get(&(loc, dir)).unwrap_or(&usize::MAX) != score {
            continue;
        }

        if !counted.contains(&loc) {
            squares += 1;
            counted.insert(loc);
        }

        if score > 0 {
            queue.push_back((score - 1, loc - CARDINALS[dir], dir));
        }
        if score > 1000 {
            let a = (dir + 1) % CARDINALS.len();
            let b = (dir + 3) % CARDINALS.len();
            queue.push_back((score - 1001, loc - CARDINALS[a], a));
            queue.push_back((score - 1001, loc - CARDINALS[b], b));
        }
    }

    squares
}

pub fn solve(str: String) -> SolutionPair {
    let grid = Grid::from_string(&str, |c| c);
    let start = grid.find_one('S').unwrap();
    let end = grid.find_one('E').unwrap();

    let (sol1, scores) = p1(&grid, start);
    let sol2 = p2(end, sol1, &scores);

    (Solution::from(sol1), Solution::from(sol2))
}
