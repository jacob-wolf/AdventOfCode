use advent_of_code_2024::{read_file, Part, Which};
use std::collections::HashSet;

pub fn p10(choice: Which, part: Part) {
    let file_data: String = read_file(10, choice, None);
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

fn get_increasing_neighbors(
    row: &usize,
    col: &usize,
    max_row: &usize,
    max_col: &usize,
    map: &Vec<Vec<usize>>,
) -> Vec<(usize, usize)> {
    let mut positions = vec![];
    if row > &0 {
        if map[row - 1][*col] == map[*row][*col] + 1 {
            positions.push((*row - 1, *col));
        }
    }
    if row < max_row {
        if map[row + 1][*col] == map[*row][*col] + 1 {
            positions.push((*row + 1, *col));
        }
    }
    if col > &0 {
        if map[*row][*col - 1] == map[*row][*col] + 1 {
            positions.push((*row, *col - 1));
        }
    }
    if col < max_col {
        if map[*row][*col + 1] == map[*row][*col] + 1 {
            positions.push((*row, *col + 1));
        }
    }
    positions
}

fn part1(data: &str) {
    let mut trailheads: Vec<(usize, usize)> = vec![];

    let map = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, chr)| {
                    let val = chr.to_digit(10).unwrap() as usize;
                    if val == 0 {
                        trailheads.push((row, col));
                    }
                    val
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;

    let mut ans: usize = 0;

    for trailhead in trailheads.into_iter() {
        let mut problems: Vec<(usize, usize)> = vec![];
        problems.push(trailhead);
        let mut nines: HashSet<(usize, usize)> = HashSet::new();
        while !problems.is_empty() {
            let (curr_row, curr_col) = problems.pop().unwrap();
            if map[curr_row][curr_col].eq(&9) {
                nines.insert((curr_row, curr_col));
            }
            let branches = get_increasing_neighbors(&curr_row, &curr_col, &max_row, &max_col, &map);

            branches.into_iter().for_each(|branch| {
                problems.push(branch);
            });
        }
        ans += nines.len();
    }

    println!("{ans}");
}

fn part2(data: &str) {
    let mut trailheads: Vec<(usize, usize)> = vec![];

    let map = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, chr)| {
                    let val = chr.to_digit(10).unwrap() as usize;
                    if val == 0 {
                        trailheads.push((row, col));
                    }
                    val
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;

    let mut ans: usize = 0;

    for trailhead in trailheads.into_iter() {
        let mut problems: Vec<(usize, usize)> = vec![];
        problems.push(trailhead);
        while !problems.is_empty() {
            let (curr_row, curr_col) = problems.pop().unwrap();
            if map[curr_row][curr_col].eq(&9) {
                ans += 1;
            }
            let branches = get_increasing_neighbors(&curr_row, &curr_col, &max_row, &max_col, &map);

            branches.into_iter().for_each(|branch| {
                problems.push(branch);
            });
        }
    }

    println!("{ans}");
}
