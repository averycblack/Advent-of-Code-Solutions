use aoclib::solution::{Solution, SolutionPair};

fn invalid_p1(val: u64) -> bool {
    let str = val.to_string();
    if str.len() % 2 != 0 {
        return false;
    }

    let (l, r) = str.split_at(str.len() / 2);
    l == r
}

fn invalid_p2(val: u64) -> bool {
    let str = val.to_string();
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
            let l = l.parse().unwrap();
            let r = r.parse().unwrap();
            (l, r)
        })
        .fold((0, 0), |mut accum, (l, r)| {
            for i in l..=r {
                if invalid_p1(i) {
                    accum.0 += i;
                }

                if invalid_p2(i) {
                    accum.1 += i;
                }
            }
            accum
        });

    (Solution::from(sol1), Solution::from(sol2))
}
