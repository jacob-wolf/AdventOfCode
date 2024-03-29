use std::{collections::HashSet, fs::read_to_string};

fn main() {
    println!("{:?}", part1(&"input.txt"));
    println!("{:?}", part2(&"input.txt"));
}
#[derive(Debug, Clone, PartialEq)]
enum StepDirection {
    Right,
    Left,
    Up,
    Down,
}

fn is_step_valid(step_target: &char, step_direction: &StepDirection) -> bool {
    match step_direction {
        StepDirection::Right => ['-', 'J', '7'].contains(step_target),
        StepDirection::Left => ['-', 'F', 'L'].contains(step_target),
        StepDirection::Up => ['|', '7', 'F'].contains(step_target),
        StepDirection::Down => ['|', 'J', 'L'].contains(step_target),
    }
}

fn part1(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();
    let map = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (s_row_index, line) = map.iter().enumerate().find(|c| c.1.contains(&'S')).unwrap();
    let (s_col_index, _line) = line.iter().enumerate().find(|c| c.1.eq(&'S')).unwrap();

    let starting_coordinate = (s_row_index, s_col_index);
    let mut loop_coordinates: Vec<(usize, usize)> = vec![starting_coordinate.clone()];

    let mut previous_step_direction: StepDirection = StepDirection::Down;

    //to make a loop needs to connect on two sides, reviewed inputs to pick start direction and neglect any overflow errors

    let mut curr_position = if is_step_valid(
        &map[starting_coordinate.0 - 1][starting_coordinate.1],
        &StepDirection::Up,
    ) {
        previous_step_direction = StepDirection::Up;
        (starting_coordinate.0 - 1, starting_coordinate.1)
    } else if is_step_valid(
        &map[starting_coordinate.0][starting_coordinate.1 + 1],
        &StepDirection::Right,
    ) {
        previous_step_direction = StepDirection::Right;
        (starting_coordinate.0, starting_coordinate.1 + 1)
    } else {
        (starting_coordinate.0 + 1, starting_coordinate.1)
    };

    while curr_position.ne(&starting_coordinate) {
        loop_coordinates.push(curr_position.clone());
        let next_step_direction = match map[curr_position.0][curr_position.1] {
            '|' => {
                if previous_step_direction == StepDirection::Down {
                    StepDirection::Down
                } else {
                    StepDirection::Up
                }
            }
            '-' => {
                if previous_step_direction == StepDirection::Left {
                    StepDirection::Left
                } else {
                    StepDirection::Right
                }
            }
            'L' => {
                if previous_step_direction == StepDirection::Left {
                    StepDirection::Up
                } else {
                    StepDirection::Right
                }
            }
            'J' => {
                if previous_step_direction == StepDirection::Down {
                    StepDirection::Left
                } else {
                    StepDirection::Up
                }
            }
            '7' => {
                if previous_step_direction == StepDirection::Right {
                    StepDirection::Down
                } else {
                    StepDirection::Left
                }
            }
            'F' => {
                if previous_step_direction == StepDirection::Up {
                    StepDirection::Right
                } else {
                    StepDirection::Down
                }
            }
            _ => StepDirection::Down,
        };
        curr_position = match next_step_direction {
            StepDirection::Down => (curr_position.0 + 1, curr_position.1),
            StepDirection::Up => (curr_position.0 - 1, curr_position.1),
            StepDirection::Right => (curr_position.0, curr_position.1 + 1),
            StepDirection::Left => (curr_position.0, curr_position.1 - 1),
        };
        previous_step_direction = next_step_direction;
    }
    if loop_coordinates.len() % 2 == 0 {
        return loop_coordinates.len() / 2;
    } else {
        return { loop_coordinates.len() - 1 } / 2;
    };
}

fn part2(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();
    let mut map = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (s_row_index, line) = map.iter().enumerate().find(|c| c.1.contains(&'S')).unwrap();
    let (s_col_index, _line) = line.iter().enumerate().find(|c| c.1.eq(&'S')).unwrap();

    let starting_coordinate = (s_row_index, s_col_index);
    let mut loop_coordinates: Vec<(usize, usize)> = vec![starting_coordinate.clone()];

    let mut previous_step_direction: StepDirection = StepDirection::Down;

    //to make a loop needs to connect on two sides, reviewed inputs to pick start direction and neglect any overflow errors

    let mut curr_position = if starting_coordinate.0 != 0
        && is_step_valid(
            &map[starting_coordinate.0 - 1][starting_coordinate.1],
            &StepDirection::Up,
        ) {
        previous_step_direction = StepDirection::Up;
        (starting_coordinate.0 - 1, starting_coordinate.1)
    } else if starting_coordinate.1 != map[0].len() -1 && is_step_valid(
        &map[starting_coordinate.0][starting_coordinate.1 + 1],
        &StepDirection::Right,
    ) {
        previous_step_direction = StepDirection::Right;
        (starting_coordinate.0, starting_coordinate.1 + 1)
    } else {
        (starting_coordinate.0 + 1, starting_coordinate.1)
    };

    while curr_position.ne(&starting_coordinate) {
        loop_coordinates.push(curr_position.clone());
        let next_step_direction = match map[curr_position.0][curr_position.1] {
            '|' => {
                if previous_step_direction == StepDirection::Down {
                    StepDirection::Down
                } else {
                    StepDirection::Up
                }
            }
            '-' => {
                if previous_step_direction == StepDirection::Left {
                    StepDirection::Left
                } else {
                    StepDirection::Right
                }
            }
            'L' => {
                if previous_step_direction == StepDirection::Left {
                    StepDirection::Up
                } else {
                    StepDirection::Right
                }
            }
            'J' => {
                if previous_step_direction == StepDirection::Down {
                    StepDirection::Left
                } else {
                    StepDirection::Up
                }
            }
            '7' => {
                if previous_step_direction == StepDirection::Right {
                    StepDirection::Down
                } else {
                    StepDirection::Left
                }
            }
            'F' => {
                if previous_step_direction == StepDirection::Up {
                    StepDirection::Right
                } else {
                    StepDirection::Down
                }
            }
            _ => StepDirection::Down,
        };
        curr_position = match next_step_direction {
            StepDirection::Down => (curr_position.0 + 1, curr_position.1),
            StepDirection::Up => (curr_position.0 - 1, curr_position.1),
            StepDirection::Right => (curr_position.0, curr_position.1 + 1),
            StepDirection::Left => (curr_position.0, curr_position.1 - 1),
        };
        previous_step_direction = next_step_direction;
    }

    let s_replacement = determine_s_replacement(&starting_coordinate, &map);
    map[s_row_index][s_col_index] = s_replacement;

    let num_cols = map[0].len();
    let num_rows = map.len();
    let loop_set: HashSet<(usize, usize)> = HashSet::from_iter(loop_coordinates);
    let mut internal_count: usize = 0;
    for row_index in 0..num_rows {
        let mut is_inside = false;
        let mut opening_pair: Option<char> = None;

        for col_index in 0..num_cols {
            if loop_set.contains(&(row_index, col_index)) {
                //check if need to update inside status
                if map[row_index][col_index] == '|' {
                    is_inside = !is_inside;
                } else {
                    if let None = opening_pair {
                        if ['F', 'L'].contains(&map[row_index][col_index]) {
                            opening_pair = Some(map[row_index][col_index]);
                        }
                    } else {
                        if ['J', '7'].contains(&map[row_index][col_index]) {
                            let opener = opening_pair.unwrap();
                            if opener == 'F' && map[row_index][col_index] == 'J' {
                                is_inside = !is_inside;
                            } else if opener == 'L' && map[row_index][col_index] == '7' {
                                is_inside = !is_inside;
                            }
                            opening_pair = None;
                        }
                    }
                }
            } else {
                if is_inside {
                    internal_count += 1;
                }
            }
        }
    }
    internal_count
}

fn determine_s_replacement(starting_coordinate: &(usize, usize), map: &Vec<Vec<char>>) -> char {
    if ['7', 'F', '|'].contains(&map[starting_coordinate.0 - 1][starting_coordinate.1]) {
        // up is valid
        if ['|', 'J', 'L'].contains(&map[starting_coordinate.0 + 1][starting_coordinate.1]) {
            //down is valid
            return '|';
        } else {
            //down is invalid
            if ['J', '7', '-'].contains(&map[starting_coordinate.0][starting_coordinate.1 + 1]) {
                //right is valid
                return 'L';
            } else {
                //left is valid
                return 'J';
            }
        }
    } else {
        //up is invalid
        if ['|', 'J', 'L'].contains(&map[starting_coordinate.0 + 1][starting_coordinate.1]) {
            //down is valid
            if ['J', '7', '-'].contains(&map[starting_coordinate.0][starting_coordinate.1 + 1]) {
                //right is valid
                return 'F';
            } else {
                //left is valid
                return '7';
            }
        } else {
            // down is invalid
            return '-';
        }
    }
}
