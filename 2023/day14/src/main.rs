use std::{collections::HashSet, fs::read_to_string};
fn main() {
    println!("{}", part1(&"input.txt"));
    println!("{}", part2(&"input.txt"));
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Type {
    Cube,
    Round,
    Empty,
}

fn part1(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();
    let rocks = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| match chr {
                    '.' => Type::Empty,
                    '#' => Type::Cube,
                    'O' => Type::Round,
                    _ => panic!("Parse issue"),
                })
                .collect::<Vec<Type>>()
        })
        .collect::<Vec<Vec<Type>>>();
    count_rocks_after_upshift(&rocks)
}

fn count_rocks_after_upshift(rocks: &Vec<Vec<Type>>) -> usize {
    let mut total: usize = 0;
    for col_index in 0..rocks[0].len() {
        let mut min_row: usize = 0;
        for row_index in 0..rocks.len() {
            match rocks[row_index][col_index] {
                Type::Cube => {
                    min_row = row_index + 1;
                }
                Type::Round => {
                    total += rocks.len() - min_row;
                    min_row += 1;
                }
                Type::Empty => {
                    continue;
                }
            }
        }
    }

    total
}

fn part2(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();
    let mut rocks = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| match chr {
                    '.' => Type::Empty,
                    '#' => Type::Cube,
                    'O' => Type::Round,
                    _ => panic!("Parse issue"),
                })
                .collect::<Vec<Type>>()
        })
        .collect::<Vec<Vec<Type>>>();

    let mut loads: Vec<Vec<Vec<Type>>> = vec![rocks.clone()];
    let mut configuration_set: HashSet<Vec<Vec<Type>>> = HashSet::new();
    configuration_set.insert(rocks.clone());

    for curr_index in 0..1_000_000_000 as usize {
        shift_rocks_cycle(&mut rocks);
        if configuration_set.contains(&rocks) {
            let mut cycle_first_index: Option<usize> = None;
            let mut cycle_first_repeat_index: Option<usize> = None;
            for (index, config) in loads.iter().enumerate() {
                if rocks.eq(config) {
                    // loads[index] describes the state after index cycles
                    cycle_first_index = Some(index);
                    // curr_index = 0 applies the 1st rotation cycle, curr = 1 applies 2nd rotation etc.
                    // repeat therefore occurs after curr_index + 1 rotation cycles
                    cycle_first_repeat_index = Some(curr_index + 1);
                    break;
                }
            }
            if let Some(prev) = cycle_first_index {
                if let Some(curr) = cycle_first_repeat_index {
                    let remaining = { 1_000_000_000 - prev } % { curr - prev };
                    //only need to apply remaining more steps, but have already done this in finding the first cycle
                    return count_rocks(&loads[prev + remaining]);
                }
            }
        }
        configuration_set.insert(rocks.clone());
        loads.push(rocks.clone());
    }

    count_rocks(&rocks)
}

fn count_rocks(rocks: &Vec<Vec<Type>>) -> usize {
    let mut total: usize = 0;
    for (row_index, row) in rocks.iter().enumerate() {
        total += { rocks.len() - row_index }
            * row.iter().filter(|item| { *item }.eq(&Type::Round)).count()
    }

    total
}

fn shift_rocks_cycle(rocks: &mut Vec<Vec<Type>>) {
    // north
    for col_index in 0..rocks[0].len() {
        let mut running_shift_index: usize = 0;
        for row_index in 0..rocks.len() {
            match rocks[row_index][col_index] {
                Type::Empty => running_shift_index += 1,
                Type::Cube => running_shift_index = 0,
                Type::Round => {
                    if running_shift_index.ne(&0) {
                        rocks[row_index - running_shift_index][col_index] = Type::Round;
                        rocks[row_index][col_index] = Type::Empty;
                    }
                }
            }
        }
    }
    // west
    for row_index in 0..rocks.len() {
        let mut running_shift_index: usize = 0;
        for col_index in 0..rocks[0].len() {
            match rocks[row_index][col_index] {
                Type::Empty => running_shift_index += 1,
                Type::Cube => running_shift_index = 0,
                Type::Round => {
                    if running_shift_index.ne(&0) {
                        rocks[row_index][col_index - running_shift_index] = Type::Round;
                        rocks[row_index][col_index] = Type::Empty;
                    }
                }
            }
        }
    }
    // south
    for col_index in 0..rocks[0].len() {
        let mut running_shift_index: usize = 0;
        for row_index in (0..rocks.len()).rev() {
            match rocks[row_index][col_index] {
                Type::Empty => running_shift_index += 1,
                Type::Cube => running_shift_index = 0,
                Type::Round => {
                    if running_shift_index.ne(&0) {
                        rocks[row_index + running_shift_index][col_index] = Type::Round;
                        rocks[row_index][col_index] = Type::Empty;
                    }
                }
            }
        }
    }
    // east
    for row_index in 0..rocks.len() {
        let mut running_shift_index: usize = 0;
        for col_index in (0..rocks[0].len()).rev() {
            match rocks[row_index][col_index] {
                Type::Empty => running_shift_index += 1,
                Type::Cube => running_shift_index = 0,
                Type::Round => {
                    if running_shift_index.ne(&0) {
                        rocks[row_index][col_index + running_shift_index] = Type::Round;
                        rocks[row_index][col_index] = Type::Empty;
                    }
                }
            }
        }
    }
}
