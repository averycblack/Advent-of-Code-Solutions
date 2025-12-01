use aoclib::solution::{Solution, SolutionPair};

#[derive(Debug, Clone, Copy)]
struct LargeTuple(i64, i64);

fn solve_machine(a: LargeTuple, b: LargeTuple, z: LargeTuple) -> i64 {
    // Solve for:
    // a_x * n_a + b_x * n_b = z_x
    // a_y * n_a + b_y * n_b = z_y
    // Substitute n_a in second equation with first equation (mutated as below)
    // n_a = (z_x - (b_x * n_b)) / a_x
    // Solve for n_b
    // Solve for n_a by substituing n_b

    // n_b = ((y*a_x) - (a_y * x)) / ((-b_x * a_y) + (b_y * a_x))
    // n_a = (x - (b_x * n_b)) / a_x

    let n_b = ((z.1 * a.0) - (a.1 * z.0)) / ((-1 * b.0 * a.1) + (b.1 * a.0));
    let n_a = (z.0 - (b.0 * n_b)) / a.0;

    if a.0 * n_a + b.0 * n_b != z.0 || a.1 * n_a + b.1 * n_b != z.1 {
        return 0;
    }

    3 * n_a + n_b
}

pub fn solve(str: String) -> SolutionPair {
    let sol1: i64 = str
        .split("\n\n")
        .map(|lines| {
            let mut lines = lines.lines();
            let a = lines.next().unwrap();
            let b = lines.next().unwrap();
            let z = lines.next().unwrap();
            let a = LargeTuple(a[12..14].parse().unwrap(), a[18..].parse().unwrap());
            let b = LargeTuple(b[12..14].parse().unwrap(), b[18..].parse().unwrap());
            let mut z = z[9..].split(", Y=");
            let z = LargeTuple(
                z.next().unwrap().parse().unwrap(),
                z.next().unwrap().parse().unwrap(),
            );
            solve_machine(a, b, z)
        })
        .sum();

    let sol2: i64 = str
        .split("\n\n")
        .map(|lines| {
            let mut lines = lines.lines();
            let a = lines.next().unwrap();
            let b = lines.next().unwrap();
            let z = lines.next().unwrap();
            let a = LargeTuple(a[12..14].parse().unwrap(), a[18..].parse().unwrap());
            let b = LargeTuple(b[12..14].parse().unwrap(), b[18..].parse().unwrap());
            let mut z = z[9..].split(", Y=");
            let z = LargeTuple(
                z.next().unwrap().parse::<i64>().unwrap() + 10000000000000_i64,
                z.next().unwrap().parse::<i64>().unwrap() + 10000000000000_i64,
            );
            solve_machine(a, b, z)
        })
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
