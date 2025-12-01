use aoclib::solution::{Solution, SolutionPair};

#[derive(Debug)]
struct File {
    size: usize,
    start: usize,
    id: usize,
}

fn p1(blocks: &Vec<File>) -> usize {
    let mut sol1 = 0;
    let mut idx = 0;
    let mut blocks_idx = 0;
    let mut blocks_end_idx = blocks.len() - 1;
    let mut blocks_end_idx_size = blocks[blocks_end_idx].size;
    'outer: while blocks_idx < blocks_end_idx {
        // Files
        let file = &blocks[blocks_idx];
        for _ in 0..file.size {
            sol1 += blocks_idx * idx;
            idx += 1;
        }

        // Fill empty space with data from end of disk
        let consume = blocks[blocks_idx + 1].start - (file.start + file.size);
        for _ in 0..consume {
            while blocks_end_idx_size == 0 {
                blocks_end_idx -= 1;

                if blocks_idx >= blocks_end_idx {
                    break 'outer;
                }

                blocks_end_idx_size = blocks[blocks_end_idx].size;
            }
            sol1 += blocks_end_idx * idx;
            blocks_end_idx_size -= 1;
            idx += 1;
        }

        // Index
        blocks_idx += 1;
    }

    while blocks_end_idx_size > 0 {
        sol1 += blocks_end_idx * idx;
        blocks_end_idx_size -= 1;
        idx += 1;
    }

    sol1
}

fn p2(disk_map: &mut Vec<File>) -> u64 {
    for file_num in (0..disk_map.len()).rev() {
        let mut file_idx = disk_map.len();
        for search_idx in (0..disk_map.len()).rev() {
            if disk_map[search_idx].id == file_num {
                file_idx = search_idx;
                break;
            }
        }

        let file = &disk_map[file_idx];

        for search_idx in 0..file_idx {
            let a = &disk_map[search_idx];
            let b = &disk_map[search_idx + 1];
            let empty_size = b.start - (a.start + a.size);
            if file.size <= empty_size {
                disk_map.insert(
                    search_idx + 1,
                    File {
                        size: file.size,
                        start: a.start + a.size,
                        id: file.id,
                    },
                );
                disk_map.remove(file_idx + 1);
                break;
            }
        }
    }

    disk_map
        .iter()
        .map(|f| {
            if f.start == 0 {
                return sequence_sum(f.size as u64 - 1);
            }

            let sum_start = sequence_sum(f.start as u64 - 1);
            let sum_end = sequence_sum(f.start as u64 + f.size as u64 - 1);
            (sum_end - sum_start) * f.id as u64
        })
        .sum()
}

fn sequence_sum(n: u64) -> u64 {
    (n + 1) * n / 2
}

pub fn solve(str: String) -> SolutionPair {
    let disk_map: Vec<u32> = str
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut file_map: Vec<File> = Vec::new();
    file_map.push(File {
        size: disk_map[0] as usize,
        start: 0,
        id: 0,
    });

    let mut start = file_map[0].size;
    for idx in (1..disk_map.len()).step_by(2) {
        let empty_size = disk_map[idx] as usize;
        file_map.push(File {
            size: disk_map[idx + 1] as usize,
            start: start + empty_size,
            id: (idx + 1) / 2,
        });

        start += empty_size + disk_map[idx + 1] as usize;
    }

    let sol1 = p1(&file_map);
    let sol2 = p2(&mut file_map);

    (Solution::from(sol1), Solution::from(sol2))
}
