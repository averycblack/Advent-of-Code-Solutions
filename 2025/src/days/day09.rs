use std::collections::BinaryHeap;

use aoclib::{
    lib2d::Coordinates,
    solution::{Solution, SolutionPair},
};

#[derive(PartialEq, Eq)]
struct Area {
    a: Coordinates,
    b: Coordinates,
    area: u64,
}

impl PartialOrd for Area {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Area {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area.cmp(&other.area)
    }
}

pub fn solve(str: String) -> SolutionPair {
    let pts: Vec<Coordinates> = str
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            Coordinates(x, y)
        })
        .collect();

    let mut areas = BinaryHeap::new();
    for i in 0..pts.len() {
        for j in (i + 1)..pts.len() {
            let a = pts[i];
            let b = pts[j];
            let diff = (a - b).abs();
            areas.push(Area {
                a: a,
                b: b,
                area: (diff.0 + 1) as u64 * (diff.1 + 1) as u64,
            });
        }
    }

    let mut lines: Vec<(Coordinates, Coordinates)> = pts.windows(2).map(|s| (s[0], s[1])).collect();
    lines.push((*pts.last().unwrap(), *pts.first().unwrap()));
    let lines = lines;

    let sol1 = areas.peek().unwrap().area;
    let sol2 = loop {
        let area = match areas.pop() {
            Some(a) => a,
            None => break 0,
        };

        let min = area.a.min(&area.b); // Top left
        let max = area.a.max(&area.b); // Bottom right

        // check for any points inside polygon
        if pts
            .iter()
            .any(|pt| pt.0 > min.0 && pt.0 < max.0 && pt.1 > min.1 && pt.1 < max.1)
        {
            continue;
        }

        // Check for any intersecting lines
        if lines.iter().any(|(a, b)| {
            let l_min = a.min(b);
            let l_max = a.max(b);
            if a.1 == b.1 {
                // Horizontal
                l_min.0 <= min.0 && l_max.0 >= max.0 && a.1 > min.1 && a.1 < max.1
            } else {
                // Vertical
                l_min.1 <= min.1 && l_max.1 >= max.1 && a.0 > min.0 && a.0 < max.0
            }
        }) {
            continue;
        }

        break area.area;
    };

    (Solution::from(sol1), Solution::from(sol2))
}
