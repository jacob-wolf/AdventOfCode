use std::{collections::HashSet, fs::read_to_string};
fn main() {
    println!("{}", part1(&"input.txt"));
    println!("{}", part2(&"input.txt"));
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum CellType {
    Vert,
    Horiz,
    BSlash,
    FSlash,
    Empty,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum BeamDirection {
    Right,
    Left,
    Down,
    Up,
}

fn part1(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();

    let grid: Vec<Vec<CellType>> = file
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|chr: char| match chr {
                    '.' => CellType::Empty,
                    '/' => CellType::FSlash,
                    '\\' => CellType::BSlash,
                    '|' => CellType::Vert,
                    '-' => CellType::Horiz,
                    _ => panic!("Parse Error"),
                })
                .collect::<Vec<CellType>>()
        })
        .collect::<Vec<Vec<CellType>>>();

    let mut energized_set: HashSet<(usize, usize)> = HashSet::new();
    let mut solved_map: HashSet<((usize, usize), BeamDirection)> = HashSet::new();
    navigate_grid(
        (0, 0),
        BeamDirection::Right,
        &grid,
        &mut energized_set,
        &mut solved_map,
    );

    energized_set.len()
}

fn navigate_grid(
    start_position: (usize, usize),
    start_direction: BeamDirection,
    grid: &Vec<Vec<CellType>>,
    energized: &mut HashSet<(usize, usize)>,
    solved: &mut HashSet<((usize, usize), BeamDirection)>,
) {
    let grid_max: (usize, usize) = (grid.len() - 1, grid[0].len() - 1);
    let mut stack_to_check: Vec<((usize, usize), BeamDirection)> =
        vec![(start_position, start_direction)];

    while !stack_to_check.is_empty() {
        let problem: ((usize, usize), BeamDirection) = stack_to_check.pop().unwrap();

        if solved.contains(&problem) {
            energized.insert(problem.0.clone());
            continue;
        }
        // we know the position and direction, and haven't solved this problem yet
        let (position, direction) = problem.clone();
        energized.insert(position.clone());
        solved.insert(problem.clone());

        let next_directions = get_next_direction(&direction, &grid[position.0][position.1]);

        for new_direction in next_directions.iter() {
            let next_position = get_next_position(&position, &new_direction, &grid_max);
            if let Some(new_position) = next_position {
                stack_to_check.push((new_position, *new_direction));
            }
        }
    }
}

fn get_next_position(
    curr_position: &(usize, usize),
    direction: &BeamDirection,
    grid_max: &(usize, usize),
) -> Option<(usize, usize)> {
    match direction {
        BeamDirection::Right => {
            if curr_position.1.eq(&grid_max.1) {
                return None;
            }
            Some((curr_position.0, curr_position.1 + 1))
        }
        BeamDirection::Left => {
            if curr_position.1.eq(&0) {
                return None;
            }
            Some((curr_position.0, curr_position.1 - 1))
        }
        BeamDirection::Down => {
            if curr_position.0.eq(&grid_max.0) {
                return None;
            }
            Some((curr_position.0 + 1, curr_position.1))
        }
        BeamDirection::Up => {
            if curr_position.0.eq(&0) {
                return None;
            }
            Some((curr_position.0 - 1, curr_position.1))
        }
    }
}

fn get_next_direction(curr_direction: &BeamDirection, curr_type: &CellType) -> Vec<BeamDirection> {
    match curr_type {
        CellType::Empty => vec![curr_direction.clone()],
        CellType::Horiz => {
            if curr_direction.eq(&BeamDirection::Down) || curr_direction.eq(&BeamDirection::Up) {
                vec![BeamDirection::Left, BeamDirection::Right]
            } else {
                vec![curr_direction.clone()]
            }
        }
        CellType::Vert => {
            if curr_direction.eq(&BeamDirection::Left) || curr_direction.eq(&BeamDirection::Right) {
                vec![BeamDirection::Up, BeamDirection::Down]
            } else {
                vec![curr_direction.clone()]
            }
        }
        CellType::FSlash => match curr_direction {
            BeamDirection::Down => vec![BeamDirection::Left],
            BeamDirection::Up => vec![BeamDirection::Right],
            BeamDirection::Right => vec![BeamDirection::Up],
            BeamDirection::Left => vec![BeamDirection::Down],
        },
        CellType::BSlash => match curr_direction {
            BeamDirection::Down => vec![BeamDirection::Right],
            BeamDirection::Up => vec![BeamDirection::Left],
            BeamDirection::Right => vec![BeamDirection::Down],
            BeamDirection::Left => vec![BeamDirection::Up],
        },
    }
}

fn part2(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();

    let grid: Vec<Vec<CellType>> = file
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|chr: char| match chr {
                    '.' => CellType::Empty,
                    '/' => CellType::FSlash,
                    '\\' => CellType::BSlash,
                    '|' => CellType::Vert,
                    '-' => CellType::Horiz,
                    _ => panic!("Parse Error"),
                })
                .collect::<Vec<CellType>>()
        })
        .collect::<Vec<Vec<CellType>>>();

    determine_max_grid(&grid)
}

fn determine_max_grid(grid: &Vec<Vec<CellType>>) -> usize {
    let min_grid: (usize, usize) = (0, 0);
    let max_grid: (usize, usize) = (grid.len(), grid[0].len());
    let starting_configs = generate_starting_configs(&min_grid, &max_grid);

    let results = starting_configs
        .iter()
        .map(|(start_position, start_direction)| {
            let mut energized_set: HashSet<(usize, usize)> = HashSet::new();
            let mut solved_map: HashSet<((usize, usize), BeamDirection)> = HashSet::new();
            navigate_grid(
                *start_position,
                *start_direction,
                grid,
                &mut energized_set,
                &mut solved_map,
            );
            energized_set.len()
        })
        .collect::<Vec<usize>>();
    *results.iter().max().unwrap()
}

fn generate_starting_configs(
    min_grid: &(usize, usize),
    max_grid: &(usize, usize),
) -> Vec<((usize, usize), BeamDirection)> {
    let mut configs: Vec<((usize, usize), BeamDirection)> = vec![];
    for row_coordinate in min_grid.0..max_grid.0 {
        configs.push(((row_coordinate, min_grid.1), BeamDirection::Right));
        configs.push(((row_coordinate, max_grid.1 - 1), BeamDirection::Left));
    }
    for col_coordinate in min_grid.1..max_grid.1 {
        configs.push(((min_grid.0, col_coordinate), BeamDirection::Down));
        configs.push(((max_grid.0 - 1, col_coordinate), BeamDirection::Up));
    }

    configs
}
