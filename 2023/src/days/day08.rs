use std::collections::HashMap;

use aoclib::{
    math::lcm,
    solution::{Solution, SolutionPair},
};

fn calc_steps(dirs: &Vec<char>, nodes: &HashMap<&str, (&str, &str)>, start_node: &str) -> u64 {
    let mut node = start_node;
    let mut steps = 0;
    while !node.ends_with('Z') {
        let dir = dirs[steps % dirs.len()];
        let choices = &nodes[&node];

        node = if dir == 'L' { choices.0 } else { choices.1 };

        steps += 1;
    }

    steps as u64
}

pub fn solve(str: String) -> SolutionPair {
    let mut lines = str.lines();
    let dirs: Vec<char> = lines.next().unwrap().chars().collect();

    let nodes: HashMap<&str, (&str, &str)> = lines
        .skip(1)
        .map(|line| {
            let node = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (node, (left, right))
        })
        .collect();

    // Part 1
    let steps = calc_steps(&dirs, &nodes, "AAA");

    // Part 2
    let cur_nodes: Vec<&str> = nodes
        .iter()
        .filter_map(
            |(key, _)| {
                if key.ends_with('A') { Some(*key) } else { None }
            },
        )
        .collect();

    let cycle_lengths: Vec<u64> = cur_nodes
        .into_iter()
        .map(|start_node| calc_steps(&dirs, &nodes, start_node))
        .collect();

    (Solution::from(steps), Solution::from(lcm(cycle_lengths)))
}
