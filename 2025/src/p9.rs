use advent_of_code_2025::{read_file, Part, Which};

pub fn p9(choice: Which, part: Part) {
    let file_data: String = read_file(9, choice, None);
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
    let coords = data
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_str| num_str.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    // attempt naive solution:
    let mut max_a = 0;

    for idx in 0..coords.len() {
        let left = &coords[idx];
        for jdx in idx + 1..coords.len() {
            let right = &coords[jdx];
            let a = { { left[0] - right[0] }.abs() + 1 } * { { left[1] - right[1] }.abs() + 1 };
            if a > max_a {
                max_a = a;
            }
        }
    }

    println!("P1: {max_a}");
}
fn part2(data: &str) {
    // red tiles form edges, can only select regions which are fully enclosed by these borders
    // need to define the boundaries and define inside vs outside

    let coords = data
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_str| num_str.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    let mut areas = vec![];

    for idx in 0..coords.len() {
        let left = &coords[idx];
        for jdx in idx + 1..coords.len() {
            let right = &coords[jdx];
            let a = { { left[0] - right[0] }.abs() + 1 } * { { left[1] - right[1] }.abs() + 1 };
            areas.push(a);
        }
    }

    let mut grid: Vec<Vec<Option<bool>>> = vec![];
    let min_row = coords.iter().map(|coord| coord[0]).min().unwrap();
    let min_col = coords.iter().map(|coord| coord[1]).min().unwrap();
    let max_row = coords.iter().map(|coord| coord[0]).max().unwrap();
    let max_col = coords.iter().map(|coord| coord[1]).max().unwrap();
    for _ in min_row..=max_row {
        let g_r: Vec<Option<bool>> = vec![None; (max_col - min_col + 1).try_into().unwrap()];
        grid.push(g_r);
    }

    // now let's populate the grid with the red and green tiles
    let iter_coords = coords
        .clone()
        .into_iter()
        .cycle()
        .take(coords.len() + 1)
        .collect::<Vec<Vec<isize>>>();
    for window in iter_coords.windows(2) {
        // println!("{window:?}");
        let left = window[0].clone(); // 7, 1
        let right = window[1].clone(); // 11, 1
        let index = if left[0] == right[0] { 1 } else { 0 };

        // goes from left coord to right coord, 1 if col - 0 if row
        // add boundaries to the grid
        let mut iter = 0..=0 as isize;

        if left[index] < right[index] {
            iter = left[index]..=right[index];
        } else {
            iter = right[index]..=left[index];
        }

        let iter_count = iter.clone().count() - 1;

        for (idx, coord) in iter.enumerate() {
            if index == 1 {
                // fixed row varied col
                grid[{ left[0] - min_row } as usize][{ coord - min_col } as usize] =
                    if idx != 0 && idx != iter_count {
                        Some(false)
                    } else {
                        Some(true)
                    };
            } else {
                // varied row fixed col
                grid[{ coord - min_row } as usize][{ left[1] - min_col } as usize] =
                    if idx != 0 && idx != iter_count {
                        Some(false)
                    } else {
                        Some(true)
                    };
            }
        }
    }
    // println!(
    //     " {:?}",
    //     (min_col..=max_col)
    //         .map(|c| format!("{c}").chars().nth(0).unwrap())
    //         .collect::<Vec<char>>()
    // );
    // for (idx, row) in grid.iter().enumerate() {
    //     println!(
    //         "{}{:?}",
    //         min_row as usize + idx,
    //         row.iter()
    //             .map(|c| if let Some(chr) = c {
    //                 if *chr {
    //                     '#'
    //                 } else {
    //                     'X'
    //                 }
    //             } else {
    //                 '.'
    //             })
    //             .collect::<Vec<char>>()
    //     );
    // }

    //grid is correct now need to determine whether two corners form a box fully inside!
    let mut max_a = 0;
    for idx in 0..coords.len() {
        let left = &coords[idx];
        for jdx in idx + 1..coords.len() {
            let right = &coords[jdx];
            println!("pair: {:?}, {:?}", left, right);

            // validate is valid (go left to right using grid var)
            let row_bound = if left[0] > right[0] {
                ({ right[0] - min_row } as usize, { left[0] - min_row }
                    as usize)
            } else {
                ({ left[0] - min_row } as usize, { right[0] - min_row }
                    as usize)
            };
            let col_bound = if left[1] > right[1] {
                ({ right[1] - min_col } as usize, { left[1] - min_col }
                    as usize)
            } else {
                ({ left[1] - min_col } as usize, { right[1] - min_col }
                    as usize)
            };

            // check the boundary, if the boundary is good the shape is good
            let mut prev_boundary_1 = true;
            let mut prev_boundary_2 = true;
            let mut status_1_count = 0;
            let mut status_2_count = 0;
            for row in row_bound.0..=row_bound.1 {
                // check top and bottom col
                status_1_count += match grid[row][col_bound.0] {
                    Some(_) => {
                        prev_boundary_1 = true;
                        0
                    }
                    None => {
                        if prev_boundary_1 {
                            prev_boundary_1 = false;
                            1
                        } else {
                            0
                        }
                    }
                };
                status_2_count += match grid[row][col_bound.1] {
                    Some(_) => {
                        prev_boundary_2 = false;
                        0
                    }
                    None => {
                        if prev_boundary_2 {
                            prev_boundary_2 = false;
                            1
                        } else {
                            0
                        }
                    }
                };
            }

            println!("counts: {}, {}", status_1_count, status_2_count);

            if status_1_count < 2 && status_2_count < 2 {
                status_1_count = 0;
                status_2_count = 0;
                prev_boundary_1 = true;
                prev_boundary_2 = true;
                for col in col_bound.0..=col_bound.1 {
                    status_1_count += match grid[row_bound.0][col] {
                        Some(_) => {
                            prev_boundary_1 = true;
                            0
                        }
                        None => {
                            if prev_boundary_1 {
                                prev_boundary_1 = false;
                                1
                            } else {
                                0
                            }
                        }
                    };
                    status_2_count += match grid[row_bound.1][col] {
                        Some(_) => {
                            prev_boundary_1 = true;
                            0
                        }
                        None => {
                            if prev_boundary_1 {
                                prev_boundary_1 = false;
                                1
                            } else {
                                0
                            }
                        }
                    };
                }
            }
            println!("counts: {}, {}", status_1_count, status_2_count);
            if status_1_count < 2 && status_2_count < 2 {
                let a = { { left[0] - right[0] }.abs() + 1 } * { { left[1] - right[1] }.abs() + 1 };
                if a > max_a {
                    max_a = a;
                    println!("max a pair: {:?}, {:?}", left, right);
                };
            }
        }
    }

    println!("P2: {max_a}")
}
