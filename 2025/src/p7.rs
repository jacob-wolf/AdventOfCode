use advent_of_code_2025::{read_file, Part, Which};
use std::collections::{HashMap, HashSet};

pub fn p7(choice: Which, part: Part) {
    let file_data: String = read_file(7, choice, None);
    let now = std::time::SystemTime::now();
    match part {
        Part::One => part1(&file_data),
        Part::Two => part2(&file_data),
    };
    match now.elapsed() {
        Ok(elapsed) => println!("Runtime: {} microseconds", elapsed.as_micros()),
        _ => panic!(),
    }
}

fn part1(data: &str) {
    let data_vec = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| match chr {
                    'S' => true,
                    '^' => true,
                    _ => false,
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let mut beam_col_idxs: HashSet<usize> = HashSet::new();
    let mut split_count = 0;
    for (row_idx, row) in data_vec.iter().enumerate() {
        if row_idx == 0 {
            if let Some((idx, _)) = row.iter().enumerate().find(|(_, item)| **item) {
                beam_col_idxs.insert(idx);
            }
        }

        for (idx, item) in row.iter().enumerate() {
            if *item && beam_col_idxs.contains(&idx) {
                // split the beam
                beam_col_idxs.insert(idx - 1);
                beam_col_idxs.insert(idx + 1);
                beam_col_idxs.remove(&idx);
                split_count += 1;
            }
        }
    }
    println!("P1: {split_count}")
}
fn part2(data: &str) {
    let mut col_count = 0;
    let data_vec = data
        .lines()
        .map(|line| {
            col_count = line.len();
            line.chars()
                .map(|chr| match chr {
                    'S' => true,
                    '^' => true,
                    _ => false,
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let mut beam_col_idxs: HashMap<usize, usize> = HashMap::new();
    for i in 0..col_count {
        beam_col_idxs.insert(i, 0);
    }

    for (row_idx, row) in data_vec.iter().enumerate() {
        let mut new_beam_col_idxs = beam_col_idxs.clone();
        for (col_idx, is_splitter) in row.iter().enumerate() {
            if row_idx == 0 && *is_splitter {
                // in the first row find the S and add 1 timeline
                new_beam_col_idxs.insert(col_idx, 1);
                beam_col_idxs = new_beam_col_idxs.clone();
                break;
            }

            // in subsequent rows propagate the wave plus multiplicity
            let curr_multiplicity = beam_col_idxs.get(&col_idx).unwrap();

            if *is_splitter {
                // need to split the beam
                let prev_left = beam_col_idxs.get(&{ col_idx - 1 }).unwrap();
                let prev_right = beam_col_idxs.get(&{ col_idx + 1 }).unwrap();

                new_beam_col_idxs.insert(col_idx - 1, prev_left + curr_multiplicity);
                new_beam_col_idxs.insert(col_idx + 1, prev_right + curr_multiplicity);
                new_beam_col_idxs.insert(col_idx, 0);
            }

            beam_col_idxs = new_beam_col_idxs.clone();
        }
    }
    println!(
        "P2: {}",
        beam_col_idxs.iter().map(|(_, v)| v).sum::<usize>()
    );
}
