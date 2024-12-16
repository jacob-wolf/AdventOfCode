use advent_of_code_2024::{read_file, Part, Which};
use std::collections::{HashMap, HashSet};

pub fn p16(choice: Which, part: Part) {
    let file_data: String = read_file(16, choice, None);
    let now = std::time::SystemTime::now();
    match part {
        Part::One => {
            part1(&file_data);
        }
        Part::Two => part2(&file_data),
    }
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

fn get_next_pos(pos: &(usize, usize), dir: &Dir) -> (usize, usize) {
    match dir {
        Dir::Pr => (pos.0 + 1, pos.1),
        Dir::Pc => (pos.0, pos.1 + 1),
        Dir::Nr => (pos.0 - 1, pos.1),
        Dir::Nc => (pos.0, pos.1 - 1),
    }
}

fn get_block_in_dir(pos: &(usize, usize), dir: &Dir, map: &Vec<Vec<Cell>>) -> Cell {
    match dir {
        Dir::Pr => map[pos.0 + 1][pos.1],
        Dir::Pc => map[pos.0][pos.1 + 1],
        Dir::Nr => map[pos.0 - 1][pos.1],
        Dir::Nc => map[pos.0][pos.1 - 1],
    }
}

fn cw(dir: &Dir) -> Dir {
    match dir {
        Dir::Pr => Dir::Nc,
        Dir::Pc => Dir::Pr,
        Dir::Nr => Dir::Pc,
        Dir::Nc => Dir::Nr,
    }
}
fn ccw(dir: &Dir) -> Dir {
    match dir {
        Dir::Pr => Dir::Pc,
        Dir::Pc => Dir::Nr,
        Dir::Nr => Dir::Nc,
        Dir::Nc => Dir::Pr,
    }
}

fn part1(data: &str) -> usize {
    // Start at S end at E. START FACING EAST
    // score is step count + 1000 * turn count
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = data
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

    let mut problems_to_check: Vec<((usize, usize), usize, Dir)> =
        vec![(start.clone(), 0, Dir::Pc)];

    let mut best_scores: HashMap<(usize, usize), usize> = HashMap::new();

    while !problems_to_check.is_empty() {
        let (curr_pos, curr_score, curr_direction) = problems_to_check.pop().unwrap();
        if let Some(curr_best) = best_scores.get(&curr_pos) {
            if *curr_best < curr_score {
                continue;
            }
            best_scores.insert(curr_pos.clone(), curr_score);
        } else {
            best_scores.insert(curr_pos.clone(), usize::MAX);
        }

        let directions_to_check = [
            curr_direction.clone(),
            cw(&curr_direction),
            ccw(&curr_direction),
        ];

        directions_to_check
            .iter()
            .enumerate()
            .for_each(
                |(index, dir)| match get_block_in_dir(&curr_pos, dir, &map) {
                    Cell::Wall => return,
                    Cell::Empty => {
                        if index > 0 {
                            let next_pos = get_next_pos(&curr_pos, dir);
                            let next_score = curr_score + 1000 + 1;
                            problems_to_check.push((next_pos, next_score, *dir));
                        } else {
                            let next_pos = get_next_pos(&curr_pos, dir);
                            let next_score = curr_score + 1;
                            problems_to_check.push((next_pos, next_score, *dir));
                        }
                    }
                },
            );
    }
    println!("{}", best_scores.get(&end).unwrap());
    *best_scores.get(&end).unwrap()
}
#[derive(Clone)]
struct Path {
    score: usize,
    points: HashSet<(usize, usize)>,
}

fn part2(data: &str) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = data
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
    let mut soln_paths: Vec<Path> = vec![];
    let mut start_map = HashSet::new();
    start_map.insert(start.clone());
    let mut problems_to_check: Vec<((usize, usize), Path, Dir)> = vec![(
        start.clone(),
        Path {
            score: 0,
            points: start_map,
        },
        Dir::Pc,
    )];
    let part1_score = part1(data);
    let mut best_scores: HashMap<(usize, usize), usize> = HashMap::new();
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            best_scores.insert((r, c), part1_score);
        }
    }

    while !problems_to_check.is_empty() {
        let (curr_pos, curr_path, curr_direction) = problems_to_check.pop().unwrap();
        if curr_path.score > part1_score
            || curr_path.score > best_scores.get(&curr_pos).unwrap() + 1000
        {
            continue;
        } else if best_scores.get(&curr_pos).unwrap() > &curr_path.score {
            best_scores.insert(curr_pos.clone(), curr_path.score);
        }

        if curr_pos.eq(&end) {
            soln_paths.push(curr_path.to_owned());
            continue;
        }

        let directions_to_check = [
            curr_direction.clone(),
            cw(&curr_direction),
            ccw(&curr_direction),
        ];
        directions_to_check
            .iter()
            .enumerate()
            .for_each(
                |(index, dir)| match get_block_in_dir(&curr_pos, dir, &map) {
                    Cell::Wall => return,
                    Cell::Empty => {
                        if curr_path.points.contains(&get_next_pos(&curr_pos, dir)) {
                            return;
                        }
                        let mut next_path_points = curr_path.points.to_owned();
                        next_path_points.insert(get_next_pos(&curr_pos, dir));
                        let mut next_score = curr_path.score + 1;
                        if index > 0 {
                            next_score += 1000;
                        }
                        let next_path = Path {
                            score: next_score,
                            points: next_path_points,
                        };
                        problems_to_check.push((get_next_pos(&curr_pos, dir), next_path, *dir))
                    }
                },
            );
    }

    let best_score = best_scores.get(&end).unwrap();
    let mut best_path_points_set: HashSet<(usize, usize)> = HashSet::new();

    soln_paths
        .iter()
        .filter(|path| path.score.eq(&best_score))
        .for_each(|best_path| {
            best_path.points.iter().for_each(|point| {
                best_path_points_set.insert(point.clone());
            });
        });
    println!("{}", best_path_points_set.len());
}
