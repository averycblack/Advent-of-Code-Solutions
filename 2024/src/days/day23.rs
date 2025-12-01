use std::collections::{HashMap, HashSet};

use aoclib::solution::{Solution, SolutionPair};

#[derive(Debug)]
struct Computer<'a> {
    name: &'a str,
    conns: HashSet<&'a str>,
}

pub fn solve(str: String) -> SolutionPair {
    let mut comps: HashMap<&str, Computer> = HashMap::new();

    for l in str.lines() {
        let (l_name, r_name) = l.split_once('-').unwrap();
        let l = comps.entry(l_name).or_insert(Computer {
            name: l_name,
            conns: HashSet::new(),
        });
        l.conns.insert(r_name);
        let r = comps.entry(r_name).or_insert(Computer {
            name: r_name,
            conns: HashSet::new(),
        });
        r.conns.insert(l_name);
    }

    let mut sol1 = 0;
    let mut sets_seen = HashSet::new();
    for (name, first) in &comps {
        for first_conn in &first.conns {
            let second = comps.get(first_conn).unwrap();
            for second_conn in &second.conns {
                let third = comps.get(second_conn).unwrap();
                if third.conns.contains(*name) {
                    if !name.starts_with('t')
                        && !second.name.starts_with('t')
                        && !third.name.starts_with('t')
                    {
                        continue;
                    }

                    let mut key = vec![name, second.name, third.name];
                    key.sort();
                    if !sets_seen.contains(&key) {
                        sets_seen.insert(key);
                        sol1 += 1;
                    }
                }
            }
        }
    }

    let mut sol2_size = 0;
    let mut sol2 = "".to_string();

    for (name, comp) in &comps {
        let conns = &comp.conns;
        let mut leftovers = comp.conns.clone();
        let mut shared_conns = vec![*name];

        'next_conn: for conn in conns {
            for other in &leftovers {
                if other == conn {
                    continue;
                }

                let other_comp = comps.get(other).unwrap();
                if !other_comp.conns.contains(conn) {
                    leftovers.remove(conn);
                    continue 'next_conn;
                }
            }

            shared_conns.push(conn);
        }

        if shared_conns.len() > sol2_size {
            shared_conns.sort();
            sol2 = shared_conns.join(",");
            sol2_size = shared_conns.len();
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
