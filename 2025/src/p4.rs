use advent_of_code_2025::{read_file, Part, Which};
use std::collections::HashSet;

pub fn p4(choice: Which, part: Part) {
    let file_data: String = read_file(4, choice, None);
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

fn count_adjs(
    max_row: usize,
    max_col: usize,
    row: usize,
    col: usize,
    data: &Vec<Vec<bool>>,
) -> usize {
    let skip_top = row == 0;
    let skip_left = col == 0;
    let skip_bottom = row == max_row - 1;
    let skip_right = col == max_col - 1;

    let mut adjs_count = 0;

    if !skip_top && !skip_left {
        if data[row - 1][col - 1] {
            adjs_count += 1;
        }
    }
    if !skip_top && !skip_right {
        if data[row - 1][col + 1] {
            adjs_count += 1;
        }
    }
    if !skip_top {
        if data[row - 1][col] {
            adjs_count += 1;
        }
    }
    if !skip_bottom && !skip_left {
        if data[row + 1][col - 1] {
            adjs_count += 1;
        }
    }
    if !skip_bottom && !skip_right {
        if data[row + 1][col + 1] {
            adjs_count += 1;
        }
    }
    if !skip_bottom {
        if data[row + 1][col] {
            adjs_count += 1;
        }
    }
    if !skip_left {
        if data[row][col - 1] {
            adjs_count += 1;
        }
    }
    if !skip_right {
        if data[row][col + 1] {
            adjs_count += 1;
        }
    }
    adjs_count
}

fn part1(data: &str) {
    // find cells with @ with fewer than 4 adjacent @
    let max_row = data.lines().count();
    let mut max_col = 0;
    let bool_data = data
        .lines()
        .map(|row| {
            max_col = row.len();
            row.chars()
                .map(|col| match col {
                    '@' => true,
                    _ => false,
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let mut total_accessible = 0;

    bool_data.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, col)| {
            if *col {
                let count = count_adjs(max_row, max_col, row_idx, col_idx, &bool_data);
                if count < 4 {
                    total_accessible += 1;
                }
            }
        })
    });

    println!("part 1: {total_accessible}")
}

fn part2(data: &str) {
    let max_row = data.lines().count();
    let mut max_col = 0;
    let mut bool_data = data
        .lines()
        .map(|row| {
            max_col = row.len();
            row.chars()
                .map(|col| match col {
                    '@' => true,
                    _ => false,
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let mut removed_set: HashSet<(usize, usize)> = HashSet::new();
    let mut total_accessible = 0;
    let mut previous_removed = 1;
    while previous_removed > 0 {
        bool_data.iter().enumerate().for_each(|(row_idx, row)| {
            row.iter().enumerate().for_each(|(col_idx, is_tp)| {
                if *is_tp {
                    let count = count_adjs(max_row, max_col, row_idx, col_idx, &bool_data);
                    if count < 4 {
                        removed_set.insert((row_idx, col_idx));
                    }
                }
            });
        });
        previous_removed = removed_set.len();
        total_accessible += previous_removed;
        //println!("removed {previous_removed} on this pass");

        // now update the bool_data to set removed pairs to false
        removed_set.iter().for_each(|pair| {
            bool_data[pair.0][pair.1] = false;
        });
        removed_set = HashSet::new();
    }
    println!("part 2: {total_accessible}")
}
