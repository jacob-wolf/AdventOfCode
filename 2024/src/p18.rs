use advent_of_code_2024::{read_file, Part, Which};
use std::collections::{HashMap, HashSet};

pub fn p18(choice: Which, part: Part) {
    let file_data: String = read_file(18, choice, None);
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

fn direction_valid(
    pos: &(usize, usize),
    dir: &Dir,
    grid: &HashSet<(usize, usize)>,
    target: &(usize, usize),
) -> Option<(usize, usize)> {
    match dir {
        Dir::Pr => {
            if pos.0 < target.0 && !grid.contains(&(pos.0 + 1, pos.1)) {
                Some((pos.0 + 1, pos.1))
            } else {
                None
            }
        }
        Dir::Pc => {
            if pos.1 < target.1 && !grid.contains(&(pos.0, pos.1 + 1)) {
                Some((pos.0, pos.1 + 1))
            } else {
                None
            }
        }
        Dir::Nr => {
            if pos.0 > 0 && !grid.contains(&(pos.0 - 1, pos.1)) {
                Some((pos.0 - 1, pos.1))
            } else {
                None
            }
        }
        Dir::Nc => {
            if pos.1 > 0 && !grid.contains(&(pos.0, pos.1 - 1)) {
                Some((pos.0, pos.1 - 1))
            } else {
                None
            }
        }
    }
}

fn drop_byte(byte: &(usize, usize), grid: &mut HashSet<(usize, usize)>) {
    grid.insert(byte.clone());
}

fn part1(data: &str) {
    let bytes_to_fall = data
        .lines()
        .map(|line| {
            let row = line
                .split(',')
                .nth(0)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            let col = line
                .split(',')
                .nth(1)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            (row, col)
        })
        .collect::<Vec<(usize, usize)>>();

    let start = (0, 0);
    let target = (70, 70); // 6,6 in test

    let mut grid = HashSet::new();
    for idx in 0..1024 {
        //0..12 in test
        drop_byte(&bytes_to_fall[idx], &mut grid);
    }

    let mut problems: Vec<((usize, usize), usize, Dir)> = vec![(start, 0, Dir::Pc)];
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    while !problems.is_empty() {
        let (pos, steps, dir) = problems.pop().unwrap();
        if let Some(prev_score) = visited.get(&pos) {
            if prev_score <= &steps {
                continue;
            } else {
                visited.insert(pos.clone(), steps);
            }
        } else {
            visited.insert(pos.clone(), steps);
        }

        if pos.eq(&target) {
            continue;
        }

        if let Some(new_pos) = direction_valid(&pos, &left(&dir), &grid, &target) {
            problems.push((new_pos, steps + 1, left(&dir)));
        }
        if let Some(new_pos) = direction_valid(&pos, &right(&dir), &grid, &target) {
            problems.push((new_pos, steps + 1, right(&dir)));
        }
        if let Some(new_pos) = direction_valid(&pos, &dir, &grid, &target) {
            problems.push((new_pos, steps + 1, dir));
        }
    }

    println!("{}", visited.get(&target).unwrap());
}

fn part2(data: &str) {
    let bytes_to_fall = data
        .lines()
        .map(|line| {
            let row = line
                .split(',')
                .nth(0)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            let col = line
                .split(',')
                .nth(1)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            (row, col)
        })
        .collect::<Vec<(usize, usize)>>();

    let start = (0, 0);
    let target = (70, 70); // 6,6 in test

    let mut lower = 1024; // 12 in test
    let mut upper = bytes_to_fall.len() - 1;
    let mut mid = { upper - lower } / 2 + lower;
    while lower + 1 < upper {
        mid = { upper - lower } / 2 + lower;
        let mut grid = HashSet::new();
        for idx in 0..mid {
            drop_byte(&bytes_to_fall[idx], &mut grid);
        }
        let mut problems: Vec<((usize, usize), usize, Dir)> = vec![(start, 0, Dir::Pc)];
        let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
        while !problems.is_empty() {
            let (pos, steps, dir) = problems.pop().unwrap();
            if let Some(prev_score) = visited.get(&pos) {
                if prev_score <= &steps {
                    continue;
                } else {
                    visited.insert(pos.clone(), steps);
                }
            } else {
                visited.insert(pos.clone(), steps);
            }

            if pos.eq(&target) {
                break;
            }

            if let Some(new_pos) = direction_valid(&pos, &left(&dir), &grid, &target) {
                problems.push((new_pos, steps + 1, left(&dir)));
            }
            if let Some(new_pos) = direction_valid(&pos, &right(&dir), &grid, &target) {
                problems.push((new_pos, steps + 1, right(&dir)));
            }
            if let Some(new_pos) = direction_valid(&pos, &dir, &grid, &target) {
                problems.push((new_pos, steps + 1, dir));
            }
        }

        if let Some(_) = visited.get(&target) {
            // not blocked!
            lower = mid;
        } else {
            // blocked!
            upper = mid;
        }
    }
    println!("{},{}", bytes_to_fall[lower].0, bytes_to_fall[lower].1);
}
