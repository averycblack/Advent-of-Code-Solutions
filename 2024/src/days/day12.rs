use std::collections::{HashSet, VecDeque};

use aoclib::{
    solution::{Solution, SolutionPair},
    lib2d::{CARDINALS, Coordinates, Grid},
};

fn p1_bfs(grid: &Grid<char>, visited: &mut HashSet<Coordinates>, start: Coordinates) -> (u64, u64) {
    let c = grid.get_point(start).unwrap();
    let mut queue: VecDeque<Coordinates> = VecDeque::new();
    let mut area = 0;
    let mut perimeter = 0;
    let mut edge_points: HashSet<Coordinates> = HashSet::new();

    queue.push_back(start);

    // Find perimeter, edge tiles, and area
    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();

        if visited.contains(&p) {
            continue;
        }

        visited.insert(p);
        area += 1;

        perimeter += p
            .neighbors_cardinal()
            .iter()
            .filter(|p| {
                if c == grid.get_point(**p).unwrap_or(' ') {
                    queue.push_back(**p);
                    return false;
                }

                true
            })
            .count() as u64;

        if perimeter != 0 {
            edge_points.insert(p);
        }
    }

    let mut edges_visisted: HashSet<(Coordinates, usize)> = HashSet::new();
    let mut num_edges = 0;
    for e in edge_points {
        for d in 0..4_usize {
            if !edges_visisted.contains(&(e, d)) {
                num_edges += find_sides(grid, &mut edges_visisted, e, d);
            }
        }
    }

    (area * perimeter, area * num_edges)
}

fn find_sides(
    grid: &Grid<char>,
    visited: &mut HashSet<(Coordinates, usize)>,
    start: Coordinates,
    start_dir: usize,
) -> u64 {
    let mut p = start;
    let mut sides = 0;
    let c = grid.get_point(start).unwrap();
    let mut dir = start_dir;

    let test = start + CARDINALS[(dir + 3) % CARDINALS.len()];
    let test = grid.get_point(test).unwrap_or(' ');
    if test == c {
        return 0;
    }

    visited.insert((p, dir));

    loop {
        let left = CARDINALS[(dir + 3) % CARDINALS.len()];
        let cont = CARDINALS[dir];
        let left = grid.get_point(p + left).unwrap_or(' ');
        let cont = grid.get_point(p + cont).unwrap_or(' ');

        // Found an outcropping to the left, go left into the outcropping
        if left == c {
            dir = (dir + 3) % CARDINALS.len();
            sides += 1;
            p += CARDINALS[dir];

        // Can not continue straight, go right
        // If turning to the right, we stay in place in case
        // we need to turn in place again.
        } else if cont != c {
            dir = (dir + 1) % CARDINALS.len();
            sides += 1;

        // Continue straight
        } else {
            p += CARDINALS[dir];
        }

        visited.insert((p, dir));
        if p == start && start_dir == dir {
            break;
        }
    }

    sides
}

pub fn solve(str: String) -> SolutionPair {
    let grid = Grid::from_string(&str, |c| c);
    let mut sol1 = 0;
    let mut sol2 = 0;
    let mut visited: HashSet<Coordinates> = HashSet::new();

    let Coordinates(max_x, max_y) = grid.get_max_size();
    for y in 0..max_y {
        for x in 0..max_x {
            if visited.contains(&Coordinates(x, y)) {
                continue;
            }

            let (p1, p2) = p1_bfs(&grid, &mut visited, Coordinates(x, y));
            sol1 += p1;
            sol2 += p2;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
