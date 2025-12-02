use aoclib::solution::{Solution, SolutionPair};

fn invalid_p1(str: &str) -> bool {
    if str.len() % 2 != 0 {
        return false;
    }

    let (l, r) = str.split_at(str.len() / 2);
    l == r
}

fn invalid_p2(str: &str) -> bool {
    'outer: for len in 1..=(str.len() / 2) {
        if str.len() % len != 0 {
            continue;
        }

        let pattern = &str[0..len];
        for i in (len..str.len()).step_by(len) {
            if &str[i..(i + len)] != pattern {
                continue 'outer;
            }
        }

        return true;
    }
    false
}

pub fn solve(str: String) -> SolutionPair {
    let (sol1, sol2) = str
        .replace("\n", "")
        .split(',')
        .map(|s| {
            let (l, r) = s.split_once('-').unwrap();
            let l: u64 = l.parse().unwrap();
            let r: u64 = r.parse().unwrap();
            (l, r)
        })
        .fold((0, 0), |mut accum, (l, r)| {
            for i in l..=r {
                let str = i.to_string();
                if invalid_p1(&str) {
                    accum.0 += i;
                }

                if invalid_p2(&str) {
                    accum.1 += i;
                }
            }
            accum
        });

    (Solution::from(sol1), Solution::from(sol2))
}
