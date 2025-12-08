use std::collections::HashMap;

use aoclib::solution::{Solution, SolutionPair};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coordinates3D {
    x: i64,
    y: i64,
    z: i64,
}

fn p1(
    circuits: &HashMap<usize, Vec<Coordinates3D>>,
    jct_to_circuit: &HashMap<Coordinates3D, usize>,
    distances: &Vec<(Coordinates3D, Coordinates3D, f64)>,
) -> usize {
    let mut circuits = circuits.clone();
    let mut jct_to_circuit = jct_to_circuit.clone();

    for i in 0..1000 {
        let (l, r, _) = distances[i];
        // To merge R into L, we must...
        // 1. Update junction to circuit mapping
        // 2. Append circuit containing R to circuit containing L
        // 3. Empty circuits array for R

        let l_circuit = jct_to_circuit[&l];
        let r_circuit = jct_to_circuit[&r];
        if l_circuit == r_circuit {
            // Same circuit, no change
            continue;
        }

        for idx in 0..circuits[&r_circuit].len() {
            let jct = circuits[&r_circuit][idx];
            // 1. Update junction to circuit mapping to point to L
            *jct_to_circuit.get_mut(&jct).unwrap() = l_circuit;
            // 2. Append R to L
            circuits.get_mut(&l_circuit).unwrap().push(jct);
        }

        // 3. Empty R
        circuits.get_mut(&r_circuit).unwrap().clear();
    }

    let mut circuits: Vec<&Vec<Coordinates3D>> = circuits.values().collect();
    circuits.sort_by(|l, r| r.len().cmp(&l.len()));
    circuits[0].len() * circuits[1].len() * circuits[2].len()
}

fn p2(
    circuits: &HashMap<usize, Vec<Coordinates3D>>,
    jct_to_circuit: &HashMap<Coordinates3D, usize>,
    distances: &Vec<(Coordinates3D, Coordinates3D, f64)>,
) -> i64 {
    let mut circuits = circuits.clone();
    let mut jct_to_circuit = jct_to_circuit.clone();

    for i in 0..(distances.len()) {
        let (l, r, _) = distances[i];
        // To merge R into L, we must...
        // 1. Update junction to circuit mapping
        // 2. Append circuit containing R to circuit containing L
        // 3. Empty circuits array for R

        let l_circuit = jct_to_circuit[&l];
        let r_circuit = jct_to_circuit[&r];
        if l_circuit == r_circuit {
            // Same circuit, no change
            continue;
        }

        for idx in 0..circuits[&r_circuit].len() {
            let jct = circuits[&r_circuit][idx];
            // 1. Update junction to circuit mapping to point to L
            *jct_to_circuit.get_mut(&jct).unwrap() = l_circuit;
            // 2. Append R to L
            circuits.get_mut(&l_circuit).unwrap().push(jct);
        }

        // 3. Empty R
        circuits.get_mut(&r_circuit).unwrap().clear();

        if circuits.values().filter(|arr| arr.len() > 0).count() == 1 {
            return l.x * r.x;
        }
    }

    panic!()
}

pub fn solve(str: String) -> SolutionPair {
    let mut jct_to_circuit: HashMap<Coordinates3D, usize> = HashMap::new();
    let mut circuits: HashMap<usize, Vec<Coordinates3D>> = HashMap::new();
    let mut distances: Vec<(Coordinates3D, Coordinates3D, f64)> = Vec::new();

    let junctions: Vec<Coordinates3D> = str
        .lines()
        .map(|l| {
            let mut args = l.split(',');
            Coordinates3D {
                x: args.next().unwrap().parse().unwrap(),
                y: args.next().unwrap().parse().unwrap(),
                z: args.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    for (idx, coord) in junctions.iter().enumerate() {
        jct_to_circuit.insert(*coord, idx);
        circuits.insert(idx, vec![*coord]);
        for i in 0..idx {
            let coord2 = junctions[i];
            let dist = ((coord.x - coord2.x).pow(2) as f64
                + (coord.y - coord2.y).pow(2) as f64
                + (coord.z - coord2.z).pow(2) as f64)
                .sqrt();
            distances.push((*coord, coord2, dist));
        }
    }

    distances.sort_by(|(_, _, dist), (_, _, dist2)| dist.total_cmp(dist2));

    let sol1 = p1(&circuits, &jct_to_circuit, &distances);
    let sol2 = p2(&circuits, &jct_to_circuit, &distances);

    (Solution::from(sol1), Solution::from(sol2))
}
