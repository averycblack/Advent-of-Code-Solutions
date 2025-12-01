use aoclib::{
    lib2d::{Coordinates, EAST, Grid, NORTH, SOUTH, WEST},
    solution::{Solution, SolutionPair},
};

fn p1(grid: &Grid<char>, movs: &str, start: Coordinates) -> i32 {
    let mut pos = start;
    let mut g = grid.clone();
    'next_move: for m in movs.chars() {
        let dir = match m {
            '>' => EAST,
            '<' => WEST,
            '^' => NORTH,
            'v' => SOUTH,
            _ => continue, // newline?
        };

        let mut i = 0;
        loop {
            let t = g.get_point(pos + (dir * (i + 1))).unwrap();
            if t == '#' {
                // Unable to move
                continue 'next_move;
            } else if t == '.' {
                // Found open space. Break out to move boxes
                break;
            }

            i += 1;
        }

        // move boxes
        for i in 1..=i {
            g.set_point(pos + (dir * (i + 1)), 'O');
        }

        // clear old space
        g.set_point(pos + dir, '.');
        pos += dir;
    }

    let mut sol1 = 0;
    for y in 0..g.max_size.1 {
        for x in 0..g.max_size.0 {
            if g.get_point(Coordinates(x, y)).unwrap() == 'O' {
                sol1 += x + (100 * y);
            }
        }
    }
    sol1
}

fn check_box(g: &Grid<char>, dir: Coordinates, loc: Coordinates) -> bool {
    let t = g.get_point(loc).unwrap();
    let o = match t {
        '[' => loc + EAST + dir,
        ']' => loc + WEST + dir,
        '.' => return true,
        '#' => return false,
        _ => panic!(),
    };

    if dir == EAST {
        return check_box(g, EAST, loc + EAST + EAST);
    } else if dir == WEST {
        return check_box(g, WEST, loc + WEST + WEST);
    }

    check_box(g, dir, loc + dir) && check_box(g, dir, o)
}

fn move_box(g: &mut Grid<char>, dir: Coordinates, loc: Coordinates) {
    let t = g.get_point(loc).unwrap();
    let (o, c) = match t {
        '[' => (loc + EAST, ']'),
        ']' => (loc + WEST, '['),
        '.' | '#' => return,
        _ => panic!(),
    };

    // Move boxes in reverse order so that we can clean up after ourselves
    if dir == WEST {
        move_box(g, WEST, loc + WEST + WEST);
    } else if dir == EAST {
        move_box(g, EAST, loc + EAST + EAST);
    } else {
        move_box(g, dir, loc + dir);
        move_box(g, dir, o + dir);
    }

    g.set_point(loc + dir, t);
    g.set_point(o + dir, c);
    if dir == NORTH || dir == SOUTH {
        g.set_point(loc, '.');
        g.set_point(o, '.');
    }
}

// Everything is twice as wide!
fn p2(grid: &Grid<char>, movs: &str, start: Coordinates) -> i32 {
    let mut g = grid.clone();
    let mut new_arr = Vec::new();

    for l in &grid.grid {
        let mut new_l = Vec::new();
        for c in l {
            match c {
                '#' => {
                    new_l.push('#');
                    new_l.push('#')
                }
                'O' => {
                    new_l.push('[');
                    new_l.push(']')
                }
                '.' => {
                    new_l.push('.');
                    new_l.push('.')
                }
                _ => panic!(),
            }
        }
        new_arr.push(new_l);
    }

    let mut pos = Coordinates(start.0 * 2, start.1);
    g.grid = new_arr;
    g.max_size.0 *= 2;
    for m in movs.chars() {
        let dir = match m {
            '>' => EAST,
            '<' => WEST,
            '^' => NORTH,
            'v' => SOUTH,
            _ => continue, // newline?
        };

        // unable to move
        if !check_box(&g, dir, pos + dir) {
            continue;
        }

        move_box(&mut g, dir, pos + dir);
        if dir == EAST || dir == WEST {
            g.set_point(pos + dir, '.');
        }

        pos += dir;
    }

    let mut sol1 = 0;
    for y in 0..g.max_size.1 {
        for x in 0..g.max_size.0 {
            if g.get_point(Coordinates(x, y)).unwrap() == '[' {
                sol1 += x + (100 * y);
            }
        }
    }
    sol1
}

#[allow(dead_code)]
fn print_grid(g: &Grid<char>, pos: Coordinates) {
    for y in 0..g.max_size.1 {
        for x in 0..g.max_size.0 {
            if Coordinates(x, y) == pos {
                print!("@");
            } else {
                print!("{}", g.get_point(Coordinates(x, y)).unwrap());
            }
        }
        println!("");
    }
}

pub fn solve(str: String) -> SolutionPair {
    let (grid, movs) = str.split_once("\n\n").unwrap();
    let mut grid = Grid::from_string(grid, |c| c);
    let start = grid.find_one('@').unwrap();
    grid.set_point(start, '.');
    let grid = grid; // Remove mutability

    let sol1 = p1(&grid, &movs, start);
    let sol2 = p2(&grid, &movs, start);

    (Solution::from(sol1), Solution::from(sol2))
}
