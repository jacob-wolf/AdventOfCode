use std::collections::HashMap;

use advent_of_code_2024::{read_file, Part, Which};

pub fn p20(choice: Which, part: Part) {
    let file_data: String = read_file(20, choice, None);
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
#[derive(Clone, Copy)]
enum Cell {
    Wall,
    Empty,
}
#[derive(Clone, Copy)]
enum Dir {
    Pr,
    Pc,
    Nr,
    Nc,
}

fn left(dir: &Dir) -> Dir {
    match dir {
        Dir::Pr => Dir::Pc,
        Dir::Pc => Dir::Nr,
        Dir::Nr => Dir::Nc,
        Dir::Nc => Dir::Pr,
    }
}

fn right(dir: &Dir) -> Dir {
    match dir {
        Dir::Pr => Dir::Nc,
        Dir::Pc => Dir::Pr,
        Dir::Nr => Dir::Pc,
        Dir::Nc => Dir::Nr,
    }
}

fn get_grid_cell_in_direction(
    pos: &(usize, usize),
    dir: &Dir,
    grid: &Vec<Vec<Cell>>,
    max_row: &usize,
    max_col: &usize,
) -> Option<(Cell, (usize, usize))> {
    match dir {
        Dir::Pr => {
            if pos.0 < *max_row {
                Some((grid[pos.0 + 1][pos.1], (pos.0 + 1, pos.1)))
            } else {
                None
            }
        }
        Dir::Pc => {
            if pos.1 < *max_col {
                Some((grid[pos.0][pos.1 + 1], (pos.0, pos.1 + 1)))
            } else {
                None
            }
        }
        Dir::Nr => {
            if pos.0 > 0 {
                Some((grid[pos.0 - 1][pos.1], (pos.0 - 1, pos.1)))
            } else {
                None
            }
        }
        Dir::Nc => {
            if pos.1 > 0 {
                Some((grid[pos.0][pos.1 - 1], (pos.0, pos.1 - 1)))
            } else {
                None
            }
        }
    }
}

fn part1(data: &str) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, chr)| match chr {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'S' => {
                        start = (row, col);
                        Cell::Empty
                    }
                    'E' => {
                        end = (row, col);
                        Cell::Empty
                    }
                    _ => panic!(),
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    let max_row = grid.len() - 1;
    let max_col = grid[0].len() - 1;

    let mut clean_problems: Vec<((usize, usize), Dir, usize)> = vec![(start, Dir::Pc, 0)];
    let mut best_scores_clean: HashMap<(usize, usize), usize> = HashMap::new();

    while !clean_problems.is_empty() {
        let (pos, dir, score) = clean_problems.pop().unwrap();
        if let Some(prev_best) = best_scores_clean.get(&pos) {
            if prev_best <= &score {
                continue;
            } else {
                best_scores_clean.insert(pos, score);
            }
        } else {
            best_scores_clean.insert(pos, score);
        }
        for next_dir in [dir, left(&dir), right(&dir)] {
            match match get_grid_cell_in_direction(&pos, &next_dir, &grid, &max_row, &max_col) {
                Some((cell, pos)) => match cell {
                    Cell::Wall => None,
                    Cell::Empty => Some(pos),
                },
                None => None,
            } {
                Some(pos) => clean_problems.push((pos, next_dir, score + 1)),
                None => {}
            }
        }
    }
    let clean_best_score = best_scores_clean.get(&end).unwrap();
    let mut cheat_problems: Vec<((usize, usize), Dir, usize, bool, bool)> =
        vec![(start, Dir::Pc, 0, false, false)];
    let mut cheat_scores: HashMap<usize, usize> = HashMap::new();

    while !cheat_problems.is_empty() {
        let (pos, dir, score, is_cheating, has_cheated) = cheat_problems.pop().unwrap();
        if pos.eq(&end) {
            if has_cheated {
                let count = cheat_scores.get(&score).unwrap_or(&0);
                cheat_scores.insert(score, count + 1);
            }
            continue;
        }
        for next_dir in [dir, left(&dir), right(&dir)] {
            match get_grid_cell_in_direction(&pos, &next_dir, &grid, &max_row, &max_col) {
                Some((cell, next_pos)) => match cell {
                    Cell::Wall => {
                        if !has_cheated && !is_cheating {
                            cheat_problems.push((next_pos, next_dir, score + 1, true, has_cheated));
                        }
                    }
                    Cell::Empty => {
                        if !is_cheating {
                            cheat_problems.push((
                                next_pos,
                                next_dir,
                                score + 1,
                                is_cheating,
                                has_cheated,
                            ));
                        } else {
                            if *best_scores_clean.get(&next_pos).unwrap() >= score + 100 {
                                let improvement =
                                    best_scores_clean.get(&next_pos).unwrap() - score - 1;
                                let count = cheat_scores
                                    .get(&{ clean_best_score - improvement })
                                    .unwrap_or(&0);
                                cheat_scores.insert(clean_best_score - improvement, count + 1);
                            }
                        }
                    }
                },
                None => {}
            }
        }
    }
    let mut saves_100_count: usize = 0;
    cheat_scores.keys().into_iter().for_each(|score| {
        let count = cheat_scores.get(&score).unwrap();
        if score + 100 <= *clean_best_score {
            saves_100_count += count;
        }
    });
    println!("{saves_100_count}");
}

fn part2(data: &str) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, chr)| match chr {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'S' => {
                        start = (row, col);
                        Cell::Empty
                    }
                    'E' => {
                        end = (row, col);
                        Cell::Empty
                    }
                    _ => panic!(),
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    let max_row = grid.len() - 1;
    let max_col = grid[0].len() - 1;

    let mut clean_problems: Vec<((usize, usize), Dir, usize)> = vec![(start, Dir::Pc, 0)];
    let mut best_scores_clean: HashMap<(usize, usize), usize> = HashMap::new();

    while !clean_problems.is_empty() {
        let (pos, dir, score) = clean_problems.pop().unwrap();
        if let Some(prev_best) = best_scores_clean.get(&pos) {
            if prev_best <= &score {
                continue;
            } else {
                best_scores_clean.insert(pos, score);
            }
        } else {
            best_scores_clean.insert(pos, score);
        }

        for next_dir in [dir, left(&dir), right(&dir)] {
            match match get_grid_cell_in_direction(&pos, &next_dir, &grid, &max_row, &max_col) {
                Some((cell, pos)) => match cell {
                    Cell::Wall => None,
                    Cell::Empty => Some(pos),
                },
                None => None,
            } {
                Some(pos) => clean_problems.push((pos, next_dir, score + 1)),
                None => {}
            }
        }
    }
    let mut found_cheats: HashMap<((usize, usize), (usize, usize)), usize> = HashMap::new();
    let mut cheat_scores: HashMap<usize, usize> = HashMap::new();

    best_scores_clean.iter().for_each(|(position, _score)| {
        let mut cheat_problems: Vec<((usize, usize), usize)> = vec![(position.clone(), 0)];
        let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
        while !cheat_problems.is_empty() {
            let (curr_position, curr_duration) = cheat_problems.pop().unwrap();
            if curr_duration > 20 {
                continue;
            }
            match visited.get(&curr_position) {
                Some(visited_duration) => {
                    if *visited_duration <= curr_duration {
                        continue;
                    }
                    visited.insert(curr_position, curr_duration);
                }
                None => {
                    visited.insert(curr_position, curr_duration);
                }
            }

            for dir in [Dir::Pc, Dir::Pr, Dir::Nr, Dir::Nc] {
                match get_grid_cell_in_direction(&curr_position, &dir, &grid, &max_row, &max_col) {
                    Some((cell, next_pos)) => match cell {
                        Cell::Wall => {
                            cheat_problems.push((next_pos, curr_duration + 1));
                        }
                        Cell::Empty => {
                            match found_cheats.get(&(position.clone(), next_pos)) {
                                Some(prev_duration) => {
                                    if *prev_duration > curr_duration + 1 {
                                        found_cheats.insert(
                                            (position.clone(), next_pos),
                                            curr_duration + 1,
                                        );
                                    }
                                }
                                None => {
                                    found_cheats
                                        .insert((position.clone(), next_pos), curr_duration + 1);
                                }
                            };
                            cheat_problems.push((next_pos, curr_duration + 1));
                        }
                    },
                    None => {}
                }
            }
        }
    });
    found_cheats.iter().for_each(|((start, end), duration)| {
        if *duration > 20 {
            return;
        }
        let clean_steps_to_end = *best_scores_clean.get(&end).unwrap();
        let cheat_steps_to_end = best_scores_clean.get(&start).unwrap() + duration;

        if cheat_steps_to_end + 50 <= clean_steps_to_end {
            let improvement = clean_steps_to_end - cheat_steps_to_end;
            let count = cheat_scores.get(&improvement).unwrap_or(&0);
            cheat_scores.insert(improvement, count + 1);
        }
    });
    let mut saves_100_count = 0;
    let mut cheat_outs = cheat_scores.iter().collect::<Vec<(&usize, &usize)>>();
    cheat_outs.sort_by(|a, b| a.0.cmp(&b.0));
    cheat_outs.iter().for_each(|(imp, count)| {
        if *imp >= &100 {
            saves_100_count += *count;
        }
    });

    println!("{saves_100_count}");
}
