use advent_of_code_2024::{read_file, Part, Which};
use std::collections::HashSet;
pub fn p6(choice: Which, part: Part) {
    let file_data: String = read_file(6, choice, None);
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

#[derive(PartialEq, Eq, Clone)]
enum CellType {
    Obstacle,
    Empty,
}
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}
#[derive(Clone, Hash, Debug)]
struct GuardState {
    direction: Direction,
    row: usize,
    col: usize,
}
impl PartialEq for GuardState {
    fn eq(&self, other: &Self) -> bool {
        self.direction.eq(&other.direction) && self.row.eq(&other.row) && self.col.eq(&other.col)
    }
    fn ne(&self, other: &Self) -> bool {
        self.direction.ne(&other.direction) || self.row.ne(&other.row) || self.col.ne(&other.col)
    }
}
impl Eq for GuardState {}

fn right_90(dir: Direction) -> Direction {
    match dir {
        Direction::N => Direction::E,
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
    }
}

/// Checks if along edges and pointing outside the boundary
fn next_is_valid(guard_state: &GuardState, max_row: &usize, max_col: &usize) -> bool {
    return !match guard_state.direction {
        Direction::N => guard_state.row == 0,
        Direction::S => guard_state.row == *max_row,
        Direction::W => guard_state.col == 0,
        Direction::E => guard_state.col == *max_col,
    };
}

/// Obtain next guard state from curr
fn next_guard_state(guard_state: &GuardState, map: &Vec<Vec<CellType>>) -> GuardState {
    match guard_state.direction {
        Direction::N => match map[guard_state.row - 1][guard_state.col] {
            CellType::Obstacle => GuardState {
                row: guard_state.row,
                col: guard_state.col,
                direction: right_90(guard_state.direction.clone()),
            },
            CellType::Empty => GuardState {
                direction: guard_state.direction.clone(),
                row: guard_state.row - 1,
                col: guard_state.col,
            },
        },
        Direction::E => match map[guard_state.row][guard_state.col + 1] {
            CellType::Obstacle => GuardState {
                row: guard_state.row,
                col: guard_state.col,
                direction: right_90(guard_state.direction.clone()),
            },
            CellType::Empty => GuardState {
                direction: guard_state.direction.clone(),
                row: guard_state.row,
                col: guard_state.col + 1,
            },
        },
        Direction::S => match map[guard_state.row + 1][guard_state.col] {
            CellType::Obstacle => GuardState {
                row: guard_state.row,
                col: guard_state.col,
                direction: right_90(guard_state.direction.clone()),
            },
            CellType::Empty => GuardState {
                direction: guard_state.direction.clone(),
                row: guard_state.row + 1,
                col: guard_state.col,
            },
        },
        Direction::W => match map[guard_state.row][guard_state.col - 1] {
            CellType::Obstacle => GuardState {
                row: guard_state.row,
                col: guard_state.col,
                direction: right_90(guard_state.direction.clone()),
            },
            CellType::Empty => GuardState {
                direction: guard_state.direction.clone(),
                row: guard_state.row,
                col: guard_state.col - 1,
            },
        },
    }
}

fn part1(data: &str) {
    let mut guard_position: (usize, usize) = (0, 0);
    let map = data
        .lines()
        .enumerate()
        .map(|(y_coord, line)| {
            line.chars()
                .enumerate()
                .map(|(x_coord, chr)| match chr {
                    '.' => CellType::Empty,
                    '#' => CellType::Obstacle,
                    '^' => {
                        guard_position = (y_coord, x_coord);
                        CellType::Empty
                    }
                    _ => panic!(),
                })
                .collect::<Vec<CellType>>()
        })
        .collect::<Vec<Vec<CellType>>>();
    let (max_row, max_col) = (map.len() - 1, map[0].len() - 1);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut guard_state = GuardState {
        row: guard_position.0,
        col: guard_position.1,
        direction: Direction::N,
    };

    while next_is_valid(&guard_state, &max_row, &max_col) {
        visited.insert((guard_state.row, guard_state.col));
        guard_state = next_guard_state(&guard_state, &map);
    }
    // final position requires an invalid update, but check if adding to set fixes things
    visited.insert((guard_state.row, guard_state.col));
    println!("{}", visited.len());
}
fn part2(data: &str) {
    let mut guard_position: (usize, usize) = (0, 0);
    let map: Vec<Vec<CellType>> = data
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, chr)| match chr {
                    '.' => CellType::Empty,
                    '#' => CellType::Obstacle,
                    '^' => {
                        guard_position = (row_idx, col_idx);
                        CellType::Empty
                    }
                    _ => panic!(),
                })
                .collect::<Vec<CellType>>()
        })
        .collect::<Vec<Vec<CellType>>>();
    let forbidden_obstacle_position = guard_position.clone();
    let (max_row, max_col) = (map.len() - 1, map[0].len() - 1);
    let mut obstacle_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut guard_state = GuardState {
        row: guard_position.0,
        col: guard_position.1,
        direction: Direction::N,
    };

    let starting_guard_state = guard_state.clone();

    while next_is_valid(&guard_state, &max_row, &max_col) {
        // check if adding an obstacle to next cell creates a loop
        let new_obstacle_position = match guard_state.direction {
            Direction::N => (guard_state.row - 1, guard_state.col),
            Direction::E => (guard_state.row, guard_state.col + 1),
            Direction::S => (guard_state.row + 1, guard_state.col),
            Direction::W => (guard_state.row, guard_state.col - 1),
        };
        if new_obstacle_position.ne(&forbidden_obstacle_position)
            && map[new_obstacle_position.0][new_obstacle_position.1].ne(&CellType::Obstacle)
        {
            // not an obstacle nor starting guard position
            let mut potential_new_map = map.clone();
            potential_new_map[new_obstacle_position.0][new_obstacle_position.1] =
                CellType::Obstacle;
            let mut seen_state_set: HashSet<GuardState> = HashSet::new();
            let mut potential_loop_guard_state = starting_guard_state.clone();
            while next_is_valid(&potential_loop_guard_state, &max_row, &max_col) {
                if !seen_state_set.insert(potential_loop_guard_state.clone()) {
                    // loop!
                    obstacle_positions.insert(new_obstacle_position);
                    break;
                }
                potential_loop_guard_state =
                    next_guard_state(&potential_loop_guard_state, &potential_new_map);
            }
        }
        guard_state = next_guard_state(&guard_state, &map);
    }
    println!("{}", obstacle_positions.len());
}
