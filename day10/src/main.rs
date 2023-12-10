use std::{fs::read_to_string, arch::x86_64::_mm_loaddup_pd};

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
    let max_x = loop_coordinates.iter().max().unwrap().0;
    let min_x = loop_coordinates.iter().min().unwrap().0;

    let max_y = loop_coordinates.iter().map(|(_,y)| y).max().unwrap();
    let min_y = loop_coordinates.iter().map(|(_,y)| y).min().unwrap();
    
    let x_extent_internal = max_x - min_x - 2;
    let y_extent_internal = max_y - min_y - 2;


    let corner_char_count = loop_coordinates.iter().map(|(row, col)| map[*row][*col]).filter(|c| ['7', 'F', 'J', 'L'].contains(c)).collect::<Vec<char>>().len() - 4;
    
    y_extent_internal*x_extent_internal - corner_char_count.div_euclid(2)

}
