use aoclib::{
    lib2d::{Coordinates, EAST, NORTH, SOUTH, WEST},
    solution::{Solution, SolutionPair},
};

#[derive(Debug)]
struct Order {
    count: i32,
    dir: Coordinates,
}

fn p1(orders: &Vec<Order>) -> i32 {
    let mut loc = Coordinates(0, 0);
    let mut vertices = Vec::new();

    vertices.push(Coordinates(0, 0));
    let mut pts: i32 = 0;

    for order in orders {
        loc += order.dir * order.count;
        vertices.push(loc);
        pts += order.count;
    }

    // Pick's Theorem - A = interior + (boundary / 2) - 1
    // Shoelace Theorem - A = (1/2) Sum^n_1 (x_i * y_i+1 - x_i+1 * y_i)
    // Boundary + Interior = A + boundary / 2 + 1
    (vertices
        .windows(2)
        .fold(0, |res, ver| {
            let a = ver[0];
            let b = ver[1];
            res + ((a.0 * b.1) - (a.1 * b.0)) as i64
        })
        .abs() as i32
        / 2)
        + (pts / 2)
        + 1
}

pub fn solve(str: String) -> SolutionPair {
    let mut p1_orders = Vec::new();
    let mut p2_orders = Vec::new();

    for l in str.lines() {
        let a: Vec<&str> = l.split_ascii_whitespace().collect();
        let dir = match a[0] {
            "U" => NORTH,
            "D" => SOUTH,
            "L" => WEST,
            "R" => EAST,
            _ => panic!(),
        };

        p1_orders.push(Order {
            count: a[1].parse().unwrap(),
            dir,
        });

        println!("{} {}", &a[2][2..7], &a[2][7..8]);
        let p2_count = i32::from_str_radix(&a[2][2..7], 16).unwrap();
        let dir = match &a[2][7..8] {
            "0" => EAST,
            "1" => SOUTH,
            "2" => WEST,
            "3" => NORTH,
            _ => panic!(),
        };

        p2_orders.push(Order {
            count: p2_count,
            dir,
        });
    }

    println!("{:?}", p2_orders);
    let sol1 = p1(&p1_orders);
    let sol2 = p1(&p2_orders);

    (Solution::from(sol1), Solution::from(sol2))
}
