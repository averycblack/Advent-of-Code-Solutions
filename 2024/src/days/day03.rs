use std::str::Chars;

use aoclib::solution::{Solution, SolutionPair};

fn p1(str: &str) -> i32 {
    let mut sol1: i32 = 0;
    for i in 0..str.len() {
        if !str[i..].starts_with("mul(") {
            continue;
        }

        let mut iter = str[i..].chars();

        iter.advance_by(4).unwrap();
        let num1 = iter
            .clone()
            .take_while(|c| c.is_numeric())
            .collect::<String>();
        if num1 == "" {
            continue;
        }

        iter.advance_by(num1.len()).ok();
        let num1 = match num1.parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if iter.next().unwrap_or_default() != ',' {
            continue;
        }

        let num2 = iter
            .clone()
            .take_while(|c| c.is_numeric())
            .collect::<String>();
        if num2 == "" {
            continue;
        }

        iter.advance_by(num2.len()).ok();
        let num2 = match num2.parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if iter.next().unwrap_or_default() != ')' {
            continue;
        }

        sol1 += num1 * num2;
    }

    sol1
}

struct CompIntParseError;
fn get_num_arg(iter: &mut Chars) -> Result<i32, CompIntParseError> {
    let arg = iter
        .clone()
        .take_while(|c| c.is_numeric())
        .collect::<String>();
    if arg == "" {
        return Err(CompIntParseError);
    }

    iter.advance_by(arg.len()).ok();
    match arg.parse::<i32>() {
        Ok(num) => return Ok(num),
        Err(_) => return Err(CompIntParseError),
    };
}

fn p2(str: &str) -> i32 {
    let mut result: i32 = 0;
    let mut enabled: bool = true;
    for i in 0..str.len() {
        if str[i..].starts_with("mul(") && enabled {
            let mut iter = str[i..].chars();

            iter.advance_by(4).unwrap();
            let num1 = get_num_arg(&mut iter);
            if let Err(CompIntParseError) = num1 {
                continue;
            }

            if iter.next().unwrap_or_default() != ',' {
                continue;
            }

            let num2 = get_num_arg(&mut iter);
            if let Err(CompIntParseError) = num2 {
                continue;
            }

            if iter.next().unwrap_or_default() != ')' {
                continue;
            }

            let num1 = num1.ok().unwrap();
            let num2 = num2.ok().unwrap();
            result += num1 * num2;
        }

        if str[i..].starts_with("do()") {
            enabled = true;
        }

        if str[i..].starts_with("don't()") {
            enabled = false;
        }
    }

    result
}

pub fn solve(str: String) -> SolutionPair {
    // Your solution here...
    let sol1: i32 = p1(&str);
    let sol2: i32 = p2(&str);

    (Solution::from(sol1), Solution::from(sol2))
}
