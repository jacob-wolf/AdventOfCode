use std::{collections::HashSet, fs::read_to_string};
fn main() {
    println!("{}", part1(&"input.txt"));
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
    navigate_grid(&grid, &mut energized_set, &mut solved_map);

    energized_set.len()
}

fn navigate_grid(
    grid: &Vec<Vec<CellType>>,
    energized: &mut HashSet<(usize, usize)>,
    solved: &mut HashSet<((usize, usize), BeamDirection)>,
) {
    let grid_max = (grid.len() - 1, grid[0].len() - 1);
    println!("{grid_max:?}");
    let mut stack_to_check: Vec<((usize, usize), BeamDirection)> =
        vec![((0, 0), BeamDirection::Right)];

    while !stack_to_check.is_empty() {
        let problem: ((usize, usize), BeamDirection) = stack_to_check.pop().unwrap();

        if solved.contains(&problem) {
            continue;
        }
        // we know the position and direction, and haven't solved this problem yet
        let (position, direction) = problem.clone();
        println!(
            "Reached {position:?} containing {:?} by going {direction:?}",
            grid[position.0][position.1]
        );
        energized.insert(position.clone());
        solved.insert(problem.clone());
        match grid[position.0][position.1] {
            CellType::Empty => {
                stack_to_check.push((
                    get_next_position(&position, &direction, &grid_max),
                    direction,
                ));
            }
            CellType::Vert => match direction {
                BeamDirection::Left => {
                    //splits beam
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Up, &grid_max),
                        BeamDirection::Up,
                    ));
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Down, &grid_max),
                        BeamDirection::Down,
                    ));
                }
                BeamDirection::Right => {
                    //splits beam

                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Up, &grid_max),
                        BeamDirection::Up,
                    ));

                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Down, &grid_max),
                        BeamDirection::Down,
                    ));
                }
                _ => {
                    stack_to_check.push((
                        get_next_position(&position, &direction, &grid_max),
                        direction,
                    ));
                }
            },
            CellType::Horiz => match direction {
                BeamDirection::Up => {
                    //splits beam
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Right, &grid_max),
                        BeamDirection::Right,
                    ));
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Left, &grid_max),
                        BeamDirection::Left,
                    ));
                }
                BeamDirection::Down => {
                    //splits beam
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Right, &grid_max),
                        BeamDirection::Right,
                    ));
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Left, &grid_max),
                        BeamDirection::Left,
                    ));
                }
                _ => {
                    stack_to_check.push((
                        get_next_position(&position, &direction, &grid_max),
                        direction,
                    ));
                }
            },
            CellType::BSlash => match direction {
                // "\"
                BeamDirection::Down => {
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Right, &grid_max),
                        BeamDirection::Right,
                    ));
                }
                BeamDirection::Up => {
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Left, &grid_max),
                        BeamDirection::Left,
                    ));
                }
                BeamDirection::Right => {
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Down, &grid_max),
                        BeamDirection::Down,
                    ));
                }
                BeamDirection::Left => {
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Up, &grid_max),
                        BeamDirection::Up,
                    ));
                }
            },
            CellType::FSlash => match direction {
                // "/"
                BeamDirection::Down => {
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Left, &grid_max),
                        BeamDirection::Left,
                    ));
                }
                BeamDirection::Up => {
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Right, &grid_max),
                        BeamDirection::Right,
                    ));
                }
                BeamDirection::Right => {
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Up, &grid_max),
                        BeamDirection::Up,
                    ));
                }
                BeamDirection::Left => {
                    stack_to_check.push((
                        get_next_position(&position, &BeamDirection::Down, &grid_max),
                        BeamDirection::Down,
                    ));
                }
            },
        }
    }
} //7413 too high

fn get_next_position(
    curr_position: &(usize, usize),
    direction: &BeamDirection,
    grid_max: &(usize, usize),
) -> (usize, usize) {
    match direction {
        BeamDirection::Right => {
            if curr_position.1.eq(&grid_max.1) {
                return *curr_position;
            }
            (curr_position.0, curr_position.1 + 1)
        }
        BeamDirection::Left => {
            if curr_position.1.eq(&0) {
                return *curr_position;
            }
            (curr_position.0, curr_position.1 - 1)
        }
        BeamDirection::Down => {
            if curr_position.0.eq(&grid_max.0) {
                return *curr_position;
            }
            (curr_position.0 + 1, curr_position.1)
        }
        BeamDirection::Up => {
            if curr_position.0.eq(&0) {
                return *curr_position;
            }
            (curr_position.0 - 1, curr_position.1)
        }
    }
}
