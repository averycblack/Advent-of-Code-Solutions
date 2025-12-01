use std::collections::{HashMap, HashSet, VecDeque};

use aoclib::solution::{Solution, SolutionPair};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum OP {
    XOR,
    OR,
    AND,
}

fn get_z(
    queue: &VecDeque<&str>,
    bit_vals: &HashMap<&str, bool>,
    results: &HashMap<&str, HashSet<&str>>,
    ops: &HashMap<&str, (OP, &str, &str)>,
) -> u64 {
    let mut bit_vals = bit_vals.clone();

    let mut queue = queue.clone();
    let mut z = 0;
    while !queue.is_empty() {
        let dest_str = queue.pop_front().unwrap();
        let dest = &results[dest_str];
        for &d in dest {
            if bit_vals.contains_key(d) {
                continue;
            }
            let (op, left, right) = ops[d];
            let left = if let Some(t) = bit_vals.get(left) {
                t
            } else {
                continue;
            };
            let right = if let Some(t) = bit_vals.get(right) {
                t
            } else {
                continue;
            };
            bit_vals.insert(
                d,
                match op {
                    OP::AND => left & right,
                    OP::XOR => left ^ right,
                    OP::OR => left | right,
                },
            );
            if d.starts_with("z") {
                let shift: u64 = d[1..3].parse().unwrap();
                let val = if *bit_vals.get(d).unwrap() { 1 } else { 0 };
                z |= val << shift;
            } else {
                queue.push_back(d);
            }
        }
    }

    z
}

fn find_val(find: &str, start: &str, ops: &HashMap<&str, (OP, &str, &str)>) -> bool {
    if start == find {
        return true;
    }

    if start.starts_with("x") || start.starts_with("y") {
        return false;
    }

    let (_, l, r) = ops[start];
    find_val(find, l, ops) || find_val(find, r, ops)
}

pub fn solve(str: String) -> SolutionPair {
    let mut ops: HashMap<&str, (OP, &str, &str)> = HashMap::new();
    let mut results: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut bit_vals = HashMap::new();
    let mut queue = VecDeque::new();

    let (init, gates) = str.split_once("\n\n").unwrap();

    for i in init.lines() {
        queue.push_back(&i[0..3]);
        let set = i.ends_with("1");
        if i.starts_with("x") {
            bit_vals.insert(&i[0..3], set);
        } else {
            bit_vals.insert(&i[0..3], set);
        }
    }

    for g in gates.lines() {
        let res: Vec<&str> = g.split_ascii_whitespace().collect();
        let left = res[0];
        let right = res[2];
        let result = res[4];
        let op = match res[1] {
            "XOR" => OP::XOR,
            "OR" => OP::OR,
            "AND" => OP::AND,
            _ => panic!(),
        };

        ops.insert(result, (op, left, right));
        let left_parents = results.entry(left).or_insert(HashSet::new());
        left_parents.insert(result);
        let right_parents = results.entry(right).or_insert(HashSet::new());
        right_parents.insert(result);
    }

    let sol1 = get_z(&queue, &bit_vals, &results, &ops);

    // Either left or right here is valid
    let mut last = ops["z01"].2;
    let mut invalid = Vec::new();

    // Go check z0 and z1 manually!
    for z in 2..45 {
        let key = format!("z{z:02}");
        let (op, left, right) = ops[key.as_str()];
        // Top level z assignments are always XORs unless it is z45
        if op != OP::XOR {
            println!("{}", key.as_str());
            invalid.push(key);
            continue;
        }

        if right.starts_with("x")
            || right.starts_with("y")
            || left.starts_with("x")
            || left.starts_with("y")
        {
            continue;
        }

        let (r_o, _, _) = ops[right];
        let (l_o, _, _) = ops[left];

        // One op is always an XOR, other is either an AND or OR
        if r_o != OP::XOR && l_o != OP::XOR {
            // Try to find the matching XOR
            let key = format!("x{z:02}");
            for &d in &results[key.as_str()] {
                if ops[d].0 == OP::XOR {
                    println!("{d}");
                    invalid.push(d.to_string());
                }
            }

            // Each argument should reference the previous
            // Find the matching argument to try and narrow down the incorrect one
            // Only XORs have x##/y## inputs, so we safe to search directly
            let left_valid = find_val(last, left, &ops);
            let right_valid = find_val(last, right, &ops);

            if !left_valid && !right_valid {
                panic!("Both should not be invalid!");
            }

            if left_valid {
                last = left;
                println!("{right}");
                invalid.push(right.to_string());
            } else {
                last = right;
                println!("{left}");
                invalid.push(left.to_string());
            }
        } else {
            // Find new last (TODO)
        }
    }

    for (n, (o, l, r)) in &ops {
        if o != &OP::XOR {
            continue;
        }

        if n.starts_with("z")
            || l.starts_with("x")
            || l.starts_with("y")
            || r.starts_with("x")
            || r.starts_with("y")
        {
            continue;
        }

        println!("{}", n);
        invalid.push(n.to_string());
    }

    invalid.sort();
    let sol2 = invalid.join(",");

    (Solution::from(sol1), Solution::from(sol2))
}
