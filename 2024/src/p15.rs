use advent_of_code_2024::{read_file, Part, Which};
use std::collections::HashSet;

pub fn p15(choice: Which, part: Part) {
    let file_data: String = read_file(15, choice, None);
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
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Box,
    Wall,
    Empty,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell2 {
    BoxLeft,
    BoxRight,
    Wall,
    Empty,
}

fn count_boxes(map: &Vec<Vec<Cell>>) -> usize {
    map.iter()
        .flatten()
        .filter(|item| item.eq(&&Cell::Box))
        .count()
}

fn part1(data: &str) {
    let mut robot_pos: (usize, usize) = (0, 0);
    let mut map = data
        .split("\r\n\r\n")
        .nth(0)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, chr)| match chr {
                    '.' => Cell::Empty,
                    '#' => Cell::Wall,
                    'O' => Cell::Box,
                    '@' => {
                        robot_pos = (row_idx, col_idx);
                        Cell::Empty
                    }
                    _ => panic!(),
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    let steps = data
        .split("\r\n\r\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| line.chars())
        .flatten()
        .collect::<Vec<char>>();

    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;
    let initial_box_count = count_boxes(&map);

    steps.iter().for_each(|step| {
        let mut boxes_to_shift: Vec<(usize, usize)> = vec![];
        let mut iter_pos = robot_pos.clone();

        match step {
            '>' => {
                iter_pos.1 += 1;
                loop {
                    if iter_pos.1 < max_col {
                        match map[iter_pos.0][iter_pos.1] {
                            Cell::Box => boxes_to_shift.push((iter_pos.0, iter_pos.1)),
                            Cell::Wall => {
                                break;
                            }
                            Cell::Empty => {
                                boxes_to_shift.iter().rev().for_each(|(bxr, bxc)| {
                                    map[*bxr][*bxc + 1] = Cell::Box;
                                    map[*bxr][*bxc] = Cell::Empty;
                                });
                                robot_pos.1 += 1;
                                break;
                            }
                        };
                        iter_pos.1 += 1;
                    } else {
                        break;
                    }
                }
            }
            '<' => {
                iter_pos.1 -= 1;
                loop {
                    if iter_pos.1 > 0 {
                        match map[iter_pos.0][iter_pos.1] {
                            Cell::Box => boxes_to_shift.push((iter_pos.0, iter_pos.1)),
                            Cell::Wall => {
                                break;
                            }
                            Cell::Empty => {
                                boxes_to_shift.iter().rev().for_each(|(bxr, bxc)| {
                                    map[*bxr][*bxc] = Cell::Empty;
                                    map[*bxr][*bxc - 1] = Cell::Box;
                                });
                                robot_pos.1 -= 1;
                                break;
                            }
                        }
                        iter_pos.1 -= 1;
                    } else {
                        break;
                    }
                }
            }
            'v' => {
                iter_pos.0 += 1;
                loop {
                    if iter_pos.0 < max_row {
                        match map[iter_pos.0][iter_pos.1] {
                            Cell::Box => boxes_to_shift.push((iter_pos.0, iter_pos.1)),
                            Cell::Wall => {
                                break;
                            }
                            Cell::Empty => {
                                boxes_to_shift.iter().rev().for_each(|(bxr, bxc)| {
                                    map[*bxr][*bxc] = Cell::Empty;
                                    map[*bxr + 1][*bxc] = Cell::Box;
                                });
                                robot_pos.0 += 1;
                                break;
                            }
                        }
                        iter_pos.0 += 1;
                    } else {
                        break;
                    }
                }
            }
            '^' => {
                iter_pos.0 -= 1;
                loop {
                    if iter_pos.0 > 0 {
                        match map[iter_pos.0][iter_pos.1] {
                            Cell::Box => boxes_to_shift.push((iter_pos.0, iter_pos.1)),
                            Cell::Wall => {
                                break;
                            }
                            Cell::Empty => {
                                boxes_to_shift.iter().rev().for_each(|(bxr, bxc)| {
                                    map[*bxr][*bxc] = Cell::Empty;
                                    map[*bxr - 1][*bxc] = Cell::Box;
                                });
                                robot_pos.0 -= 1;
                                break;
                            }
                        }
                        iter_pos.0 -= 1;
                    } else {
                        break;
                    }
                }
            }
            _ => panic!(),
        }

        assert_eq!(count_boxes(&map), initial_box_count);
    });
    let mut ans: usize = 0;
    map.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter()
            .enumerate()
            .for_each(|(col_idx, cell)| match cell {
                Cell::Box => {
                    ans += 100 * row_idx + col_idx;
                }
                Cell::Wall => {}
                Cell::Empty => {}
            });
    });
    println!("{ans}");
}

fn count_boxes2(map: &Vec<Vec<Cell2>>) -> usize {
    map.iter()
        .flatten()
        .filter(|item| item.eq(&&Cell2::BoxLeft) || item.eq(&&Cell2::BoxRight))
        .count()
        / 2
}

fn part2(data: &str) {
    let mut robot_pos: (usize, usize) = (0, 0);
    let mut map = data
        .split("\r\n\r\n")
        .nth(0)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, chr)| match chr {
                    '.' => [Cell2::Empty, Cell2::Empty],
                    '#' => [Cell2::Wall, Cell2::Wall],
                    'O' => [Cell2::BoxLeft, Cell2::BoxRight],
                    '@' => {
                        robot_pos = (row_idx, col_idx * 2);
                        [Cell2::Empty, Cell2::Empty]
                    }
                    _ => panic!(),
                })
                .flatten()
                .collect::<Vec<Cell2>>()
        })
        .collect::<Vec<Vec<Cell2>>>();
    let max_row = map.len() - 1;
    let max_col = 2 * { map[0].len() - 1 };
    let initial_box_count = count_boxes2(&map);
    let steps = data
        .split("\r\n\r\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| line.chars())
        .flatten()
        .collect::<Vec<char>>();
    steps.iter().for_each(|step| {
        let mut iter_pos = robot_pos.clone();
        match step {
            '>' => {
                let mut boxes_to_shift: Vec<(usize, usize)> = vec![];
                iter_pos.1 += 1;
                loop {
                    if iter_pos.1 < max_col {
                        match map[iter_pos.0][iter_pos.1] {
                            Cell2::BoxLeft => boxes_to_shift.push((iter_pos.0, iter_pos.1)),
                            Cell2::BoxRight => {}
                            Cell2::Wall => {
                                break;
                            }
                            Cell2::Empty => {
                                boxes_to_shift.iter().rev().for_each(|(bxr, bxc)| {
                                    map[*bxr][*bxc + 2] = Cell2::BoxRight;
                                    map[*bxr][*bxc + 1] = Cell2::BoxLeft;
                                    map[*bxr][*bxc] = Cell2::Empty;
                                });
                                robot_pos.1 += 1;
                                break;
                            }
                        }
                        iter_pos.1 += 1;
                    } else {
                        break;
                    }
                }
            }
            '<' => {
                let mut boxes_to_shift: Vec<(usize, usize)> = vec![];
                iter_pos.1 -= 1;
                loop {
                    if iter_pos.1 > 0 {
                        match map[iter_pos.0][iter_pos.1] {
                            Cell2::BoxLeft => boxes_to_shift.push((iter_pos.0, iter_pos.1)),
                            Cell2::BoxRight => {}
                            Cell2::Wall => {
                                break;
                            }
                            Cell2::Empty => {
                                boxes_to_shift.iter().rev().for_each(|(bxr, bxc)| {
                                    map[*bxr][*bxc + 1] = Cell2::Empty;
                                    map[*bxr][*bxc] = Cell2::BoxRight;
                                    map[*bxr][*bxc - 1] = Cell2::BoxLeft;
                                });
                                robot_pos.1 -= 1;
                                break;
                            }
                        }
                        iter_pos.1 -= 1;
                    } else {
                        break;
                    }
                }
            }
            'v' => {
                iter_pos.0 += 1;
                let mut boxes_to_shift: HashSet<(usize, usize)> = HashSet::new();
                let mut cols_to_check: HashSet<usize> = HashSet::new();
                cols_to_check.insert(iter_pos.1);
                let mut blocked = false;
                loop {
                    if iter_pos.0 < max_row {
                        for col in cols_to_check.clone().iter() {
                            match map[iter_pos.0][*col] {
                                Cell2::BoxLeft => {
                                    cols_to_check.insert(col + 1);
                                    boxes_to_shift.insert((iter_pos.0, *col));
                                    boxes_to_shift.insert((iter_pos.0, *col + 1));
                                }
                                Cell2::BoxRight => {
                                    cols_to_check.insert(col - 1);
                                    boxes_to_shift.insert((iter_pos.0, *col));
                                    boxes_to_shift.insert((iter_pos.0, *col - 1));
                                }
                                Cell2::Wall => {
                                    boxes_to_shift.clear();
                                    cols_to_check.clear();
                                    blocked = true;
                                    break;
                                }
                                Cell2::Empty => {
                                    cols_to_check.remove(col);
                                }
                            }
                        }
                        if cols_to_check.len() == 0 {
                            let mut shift_boxes =
                                boxes_to_shift.into_iter().collect::<Vec<(usize, usize)>>();
                            shift_boxes.sort_by(|a, b| {
                                //want bottom boxes to move first larger row move first
                                b.0.cmp(&a.0) // sort descending row index
                            });
                            shift_boxes.iter().for_each(|(bxr, bxc)| {
                                map[*bxr + 1][*bxc] = map[*bxr][*bxc];
                                map[*bxr][*bxc] = Cell2::Empty;
                            });
                            if !blocked {
                                robot_pos.0 += 1;
                            }
                            break;
                        }
                        iter_pos.0 += 1;
                    } else {
                        break;
                    }
                }
            }
            '^' => {
                iter_pos.0 -= 1;
                let mut boxes_to_shift: HashSet<(usize, usize)> = HashSet::new();
                let mut cols_to_check: HashSet<usize> = HashSet::new();
                cols_to_check.insert(iter_pos.1);
                let mut blocked = false;
                loop {
                    if iter_pos.0 > 0 {
                        for col in cols_to_check.clone().iter() {
                            match map[iter_pos.0][*col] {
                                Cell2::BoxLeft => {
                                    cols_to_check.insert(col + 1);
                                    boxes_to_shift.insert((iter_pos.0, *col));
                                    boxes_to_shift.insert((iter_pos.0, *col + 1));
                                }
                                Cell2::BoxRight => {
                                    cols_to_check.insert(col - 1);
                                    boxes_to_shift.insert((iter_pos.0, *col));
                                    boxes_to_shift.insert((iter_pos.0, *col - 1));
                                }
                                Cell2::Wall => {
                                    boxes_to_shift.clear();
                                    cols_to_check.clear();
                                    blocked = true;
                                    break;
                                }
                                Cell2::Empty => {
                                    cols_to_check.remove(col);
                                }
                            }
                        }
                        if cols_to_check.len() == 0 {
                            let mut shift_boxes =
                                boxes_to_shift.into_iter().collect::<Vec<(usize, usize)>>();
                            shift_boxes.sort_by(|a, b| {
                                //want top boxes to move first smaller row move first
                                a.0.cmp(&b.0) // sort descending row index
                            });
                            shift_boxes.iter().for_each(|(bxr, bxc)| {
                                map[*bxr - 1][*bxc] = map[*bxr][*bxc];
                                map[*bxr][*bxc] = Cell2::Empty;
                            });
                            if !blocked {
                                robot_pos.0 -= 1;
                            }
                            break;
                        }
                        iter_pos.0 -= 1;
                    } else {
                        break;
                    }
                }
            }
            _ => panic!(),
        }
        assert_eq!(count_boxes2(&map), initial_box_count);
    });

    let mut ans: usize = 0;
    map.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter()
            .enumerate()
            .for_each(|(col_idx, cell)| match cell {
                Cell2::BoxLeft => {
                    ans += 100 * row_idx + col_idx;
                }
                Cell2::BoxRight => {}
                Cell2::Wall => {}
                Cell2::Empty => {}
            });
    });
    println!("{ans}");
}
