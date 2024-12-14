use advent_of_code_2024::{read_file, Part, Which};
use std::collections::{HashMap, HashSet};
pub fn p14(choice: Which, part: Part) {
    let file_data: String = read_file(14, choice, None);
    let now = std::time::SystemTime::now();
    match part {
        Part::One => part1(&file_data),
        Part::Two => part2(&file_data),
    }
    match now.elapsed() {
        Ok(elapsed) => println!("Runtime: {} microseconds", elapsed.as_micros()),
        _ => panic!(),
    }
}
fn find_quadrant_index(
    row_mid: &isize,
    col_mid: &isize,
    row: &isize,
    col: &isize,
) -> Option<usize> {
    if row.eq(&row_mid) || col.eq(&col_mid) {
        return None;
    }
    if row < row_mid && col < col_mid {
        Some(0)
    } else if row < row_mid && col > col_mid {
        Some(1)
    } else if row > row_mid && col > col_mid {
        Some(2)
    } else if row > row_mid && col < col_mid {
        Some(3)
    } else {
        None
    }
}
fn part1(data: &str) {
    // let rows = 7; // test values
    // let cols = 11;
    let rows = 103;
    let cols = 101;
    let row_mid: isize = rows / 2;
    let col_mid = cols / 2;

    let mut quadrant_counts: [usize; 4] = [0, 0, 0, 0];

    data.lines().for_each(|line| {
        let kinematics = line
            .split_ascii_whitespace()
            .map(|spl| {
                let pair = spl
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .split(',')
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>();
                (pair[0], pair[1])
            })
            .collect::<Vec<(isize, isize)>>();

        let pos = kinematics[0];
        let velo = kinematics[1];

        let mut later_pos = (
            { pos.0 + 100 * velo.0 } % cols,
            { pos.1 + 100 * velo.1 } % rows,
        );
        if later_pos.0 < 0 {
            later_pos.0 += cols;
        }
        if later_pos.1 < 0 {
            later_pos.1 += rows;
        }

        println!("{pos:?} -> {later_pos:?}");
        let index = find_quadrant_index(&row_mid, &col_mid, &later_pos.1, &later_pos.0);
        match index {
            Some(idx) => quadrant_counts[idx] += 1,
            None => return,
        };
    });

    let ans = quadrant_counts.iter().product::<usize>();
    println!("{ans}");
}

fn print_tree(pos: &HashSet<(isize, isize)>, width: isize, height: isize) {
    for r in 0..height {
        for c in 0..width {
            if pos.contains(&(c, r)) {
                print!("X")
            } else {
                print!(" ")
            }
        }
        print!("\n");
    }
}

fn part2(data: &str) {
    let rows = 103;
    let cols = 101;
    let kinematics = data
        .lines()
        .map(|line| {
            let kinematics = line
                .split_ascii_whitespace()
                .map(|spl| {
                    let pair = spl
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .split(',')
                        .map(|num| num.parse::<isize>().unwrap())
                        .collect::<Vec<isize>>();
                    (pair[0], pair[1])
                })
                .collect::<Vec<(isize, isize)>>();

            let pos = kinematics[0];
            let velo = kinematics[1];
            (pos, velo)
        })
        .collect::<Vec<((isize, isize), (isize, isize))>>();

    // create a set when it is position 500
    let mut positions_set: HashSet<(isize, isize)> = HashSet::new();
    let mut positions = kinematics
        .iter()
        .cloned()
        .map(|i| i.0)
        .collect::<Vec<(isize, isize)>>();

    positions.iter().for_each(|p| {
        positions_set.insert(p.clone());
    });
    let velos = kinematics
        .into_iter()
        .map(|i| i.1)
        .collect::<Vec<(isize, isize)>>();
    let mut step_count: usize = 0;
    while step_count < 1_000_000 {
        // update all the pos and keep looking
        let mut row_map: HashMap<isize, usize> = HashMap::new();
        let mut new_positions = positions.clone();
        for (index, p_i) in positions.iter().cloned().enumerate() {
            let v = velos[index];
            let mut p_f = ({ p_i.0 + v.0 } % cols, { p_i.1 + v.1 } % rows);
            if p_f.0 < 0 {
                p_f.0 += cols;
            }
            if p_f.1 < 0 {
                p_f.1 += rows;
            }
            if let Some(val) = row_map.get(&p_f.1) {
                row_map.insert(p_f.1, val + 1);
            } else {
                row_map.insert(p_f.1, 1);
            }
            new_positions[index] = p_f;
            positions_set.remove(&p_i);
            positions_set.insert(p_f);
        }
        positions = new_positions;
        step_count += 1;

        let mut sequential_bot_row_count = 0;
        if row_map.iter().filter(|(_row, count)| *count > &30).count() > 1 {
            let populated_rows = row_map
                .iter()
                .filter(|(_row, count)| *count > &30)
                .map(|(row, _count)| row)
                .collect::<Vec<&isize>>();

            populated_rows.iter().for_each(|populated_row| {
                let mut populated_row_cols = positions
                    .iter()
                    .filter(|(_col, row)| row.eq(&populated_row))
                    .map(|(col, _row)| col)
                    .collect::<Vec<&isize>>();

                populated_row_cols.sort();

                let mut sequential_col_count = 0;

                populated_row_cols.windows(2).for_each(|window| {
                    if window[1] - window[0] == 1 {
                        sequential_col_count += 1;
                    }
                });

                if sequential_col_count > 10 {
                    sequential_bot_row_count += 1;
                }
            });
            if sequential_bot_row_count > 1 {
                println!("{step_count}");
                print_tree(&positions_set, cols, rows);
                break;
            }
        }
    }
    println!("{step_count}");
}
