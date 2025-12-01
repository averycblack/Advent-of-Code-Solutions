use aoclib::solution::{Solution, SolutionPair};

#[derive(Clone, Copy)]
struct CPU {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
}

fn combo_operand(cpu: &CPU, op: u64) -> u64 {
    match op {
        0..=3 => op,
        4 => cpu.a,
        5 => cpu.b,
        6 => cpu.c,
        _ => panic!(),
    }
}

fn run(mut cpu: CPU, instructions: &Vec<u64>) -> Vec<u64> {
    let mut outputs = Vec::new();

    loop {
        let inst = *match instructions.get(cpu.pc) {
            Some(v) => v,
            None => break,
        };

        let op = *match instructions.get(cpu.pc + 1) {
            Some(v) => v,
            None => panic!(),
        };

        match inst {
            0 => cpu.a = cpu.a / 2_u64.pow(combo_operand(&cpu, op) as u32),
            1 => cpu.b ^= op,
            2 => cpu.b = combo_operand(&cpu, op) % 8,
            3 => {
                if cpu.a != 0 {
                    cpu.pc = op as usize;
                    continue;
                }
            }
            4 => cpu.b ^= cpu.c,
            5 => outputs.push(combo_operand(&cpu, op) % 8),
            6 => cpu.b = cpu.a / 2_u64.pow(combo_operand(&cpu, op) as u32),
            7 => cpu.c = cpu.a / 2_u64.pow(combo_operand(&cpu, op) as u32),
            _ => panic!("Invalid instruction {}", inst),
        }

        cpu.pc += 2;
    }

    outputs
}

fn p2(instructions: &Vec<u64>) -> u64 {
    let mut idx = 0;
    let mut a_vals = Vec::new();
    'next_op: loop {
        if idx == instructions.len() {
            return a_vals.iter().fold(0, |acc, v| (acc << 3) | *v);
        }

        if idx >= a_vals.len() {
            a_vals.push(0);
        }

        let a = a_vals[..=idx].iter().fold(0, |acc, v| (acc << 3) | *v);
        let op = instructions[instructions.len() - idx - 1];
        for i in a_vals[idx]..8 {
            let new_a = a | i;
            let mut b = i;
            b ^= 2;
            let c = new_a >> b;
            b ^= 7;
            b ^= c;

            if b % 8 == op {
                a_vals[idx] = i;
                idx += 1;
                continue 'next_op;
            }
        }

        a_vals[idx] = 0;
        idx -= 1;
        a_vals[idx] += 1;
        if a_vals[idx] > 7 {
            panic!();
        }
    }
}

pub fn solve(str: String) -> SolutionPair {
    let (registers, instructions) = str.split_once("\n\n").unwrap();
    let instructions: Vec<u64> = instructions[9..]
        .split(',')
        .map(|str| str.trim().parse().unwrap())
        .collect();
    let mut registers = registers.lines().map(|l| l[12..].parse().unwrap());
    let cpu = CPU {
        a: registers.next().unwrap(),
        b: registers.next().unwrap(),
        c: registers.next().unwrap(),
        pc: 0,
    };

    let sol1 = run(cpu, &instructions)
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let sol2 = p2(&instructions);

    (Solution::from(sol1), Solution::from(sol2))
}
