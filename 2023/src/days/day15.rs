use aoclib::solution::{Solution, SolutionPair};

pub fn solve(str: String) -> SolutionPair {
    let mut boxes: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];

    let sol1: u32 = str
        .split(',')
        .map(|val| {
            val.chars()
                .fold(0, |accum, char| ((accum + char as u32) * 17) % 256)
        })
        .sum();

    str.split(',').for_each(|val| {
        let label = if val.ends_with('-') {
            &val[..val.len() - 1]
        } else {
            &val[..val.len() - 2]
        };
        let hash = label
            .chars()
            .fold(0, |accum, char| ((accum + char as u32) * 17) % 256);

        let bx = &mut boxes[hash as usize];
        if val.ends_with('-') {
            let iter = bx.iter().position(|(lbl, _)| *lbl == label);
            if iter.is_some() {
                bx.remove(iter.unwrap());
            }
        } else {
            let iter = bx.iter().position(|(lbl, _)| *lbl == label);
            let focus = val.chars().last().unwrap().to_digit(10).unwrap();
            if iter.is_none() {
                bx.push((label, focus));
            } else {
                bx[iter.unwrap()] = (label, focus);
            }
        }
    });

    let sol2: u32 = boxes
        .iter()
        .enumerate()
        .map(|(box_slot, b0x)| {
            b0x.iter()
                .enumerate()
                .map(|(slot_in_box, (_, focus))| {
                    (1 + box_slot as u32) * (1 + slot_in_box as u32) * focus
                })
                .sum::<u32>()
        })
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
