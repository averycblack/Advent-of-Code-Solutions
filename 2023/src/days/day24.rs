use std::ops::{Add, Sub};

use aoclib::solution::{Solution, SolutionPair};

const WINDOW_MIN: f64 = 200000000000000.0;
const WINDOW_MAX: f64 = 400000000000000.0;
// const WINDOW_MIN: f64 = 7.0;
// const WINDOW_MAX: f64 = 27.0;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinates3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Sub<Coordinates3D> for Coordinates3D {
    type Output = Coordinates3D;
    fn sub(self, rhs: Coordinates3D) -> Self::Output {
        Coordinates3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Coordinates3D {
    fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(&self, rhs: Self) -> Coordinates3D {
        Coordinates3D {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }
}

impl Add<Coordinates3D> for Coordinates3D {
    type Output = Coordinates3D;
    fn add(self, rhs: Coordinates3D) -> Self::Output {
        Coordinates3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct HailStone {
    p: Coordinates3D,
    v: Coordinates3D,
}

fn parse_3d(s: &str) -> Coordinates3D {
    let v: Vec<f64> = s
        .split(',')
        .map(|val| val.trim().parse().unwrap())
        .collect();
    Coordinates3D {
        x: v[0],
        y: v[1],
        z: v[2],
    }
}

fn p1(hail: &Vec<HailStone>) -> usize {
    let mut res = 0;

    for i in 0..hail.len() {
        for j in i..hail.len() {
            let a = hail[i];
            let b = hail[j];

            if j == i {
                continue;
            }

            // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
            let p1 = a.p;
            let p2 = a.p + a.v;

            let p3 = b.p;
            let p4 = b.p + b.v;

            let denom = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);
            let x_num = (p1.x * p2.y - p1.y * p2.x) * (p3.x - p4.x)
                - (p1.x - p2.x) * (p3.x * p4.y - p3.y * p4.x);
            let y_num = (p1.x * p2.y - p1.y * p2.x) * (p3.y - p4.y)
                - (p1.y - p2.y) * (p3.x * p4.y - p3.y * p4.x);

            if denom == 0.0 {
                continue;
            }

            let x = x_num as f64 / denom as f64;
            let y = y_num as f64 / denom as f64;
            let t_y = (y - a.p.y as f64) / a.v.y as f64;
            let t_x = (x - b.p.x as f64) / b.v.x as f64;

            if t_y > 0.0
                && t_x > 0.0
                && x >= WINDOW_MIN
                && x <= WINDOW_MAX
                && y >= WINDOW_MIN
                && y <= WINDOW_MAX
            {
                res += 1;
            }
        }
    }

    res
}

// Get three linear independent hailstones
fn p2_get_hail(hail: &Vec<HailStone>) -> [HailStone; 3] {
    let mut idx1 = 1;
    let mut idx2 = 2;

    loop {
        let h1 = hail[0];
        let h2 = hail[idx1];
        let h3 = hail[idx2];

        let v1 = h1.v;
        let v2 = h2.v;
        let v3 = h3.v;

        // | p1_x p2_x p3_x |
        // | p1_y p2_y p3_y |
        // | p1_z p2_z p3_z |

        let det = (v1.x * v2.y * v3.z) + (v2.x * v3.y * v1.z) + (v3.x * v1.y * v2.z)
            - (v3.x * v2.y * v1.z)
            - (v2.x * v1.y * v3.z)
            - (v1.x * v3.y * v2.z);

        if det != 0.0 {
            return [h1, h2, h3];
        }

        idx1 *= 2;
        idx2 *= 2;
    }
}

fn p2(hail: &Vec<HailStone>) -> i64 {
    let [h1, h2, h3] = p2_get_hail(hail);

    let p1 = h1.p;
    let p2 = h2.p;
    let p3 = h3.p;

    let v1 = h1.v;
    let v2 = h2.v;
    let v3 = h3.v;

    // Calculate three planes
    let a = (p1 - p2).cross(v1 - v2);
    let b = (p1 - p3).cross(v1 - v3);
    let c = (p2 - p3).cross(v2 - v3);
    
    #[allow(non_snake_case)]
    let A = (p1 - p2).dot(v1.cross(v2));
    #[allow(non_snake_case)]
    let B = (p1 - p3).dot(v1.cross(v3));
    #[allow(non_snake_case)]
    let C = (p2 - p3).dot(v2.cross(v3));

    // Velocity
    // w = p * (bxc) + q * (cxa) + r * (axb)
    let w = Coordinates3D {
        x: A * b.cross(c).x + B * c.cross(a).x + C * a.cross(b).x,
        y: A * b.cross(c).y + B * c.cross(a).y + C * a.cross(b).y,
        z: A * b.cross(c).z + B * c.cross(a).z + C * a.cross(b).z,
    };

    let t = a.dot(b.cross(c));
    let w = Coordinates3D {
        x: w.x / t,
        y: w.y / t,
        z: w.z / t,
    };

    let v1_w = v1 - w;
    let v2_w = v2 - w;
    let wz = v1_w.cross(v2_w);

    let e = wz.dot(p2.cross(v2_w));
    let f = wz.dot(p1.cross(v1_w));
    let g = p1.dot(wz);
    let s = wz.dot(wz);

    // r = p_i + (v_i - w) * t
    let r = Coordinates3D {
        x: e * v1_w.x + -f * v2_w.x + g * wz.x,
        y: e * v1_w.y + -f * v2_w.y + g * wz.y,
        z: e * v1_w.z + -f * v2_w.z + g * wz.z,
    };

    let r = Coordinates3D {
        x: r.x / s,
        y: r.y / s,
        z: r.z / s,
    };

    println!("{:?}", w);
    println!("{:?}", r);
    (r.x + r.y + r.z) as i64
}

pub fn solve(str: String) -> SolutionPair {
    let hail: Vec<HailStone> = str
        .lines()
        .map(|h| {
            let (left, right) = h.split_once(" @ ").unwrap();
            HailStone {
                p: parse_3d(left),
                v: parse_3d(right),
            }
        })
        .collect();

    let sol1 = p1(&hail);
    let sol2 = p2(&hail);

    (Solution::from(sol1), Solution::from(sol2))
}
