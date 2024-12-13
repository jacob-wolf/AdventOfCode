use advent_of_code_2024::{read_file, Part, Which};
use std::{collections::HashSet, hash::Hash};
pub fn p12(choice: Which, part: Part) {
    let file_data: String = read_file(12, choice, None);
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

fn explore_region(
    row: &usize,
    col: &usize,
    max_row: &usize,
    max_col: &usize,
    map: &Vec<Vec<char>>,
) -> HashSet<(usize, usize)> {
    let mut region_set: HashSet<(usize, usize)> = HashSet::new();
    let target = map[*row][*col];
    let mut problems: Vec<(usize, usize)> = vec![(*row, *col)];

    while !problems.is_empty() {
        let (curr_row, curr_col) = problems.pop().unwrap();
        let new_coord = region_set.insert((curr_row, curr_col));
        if !new_coord {
            continue;
        }
        if curr_row > 0 && map[curr_row - 1][curr_col].eq(&target) {
            problems.push(({ curr_row - 1 }, curr_col));
        }
        if curr_row < *max_row && map[curr_row + 1][curr_col].eq(&target) {
            problems.push(({ curr_row + 1 }, curr_col));
        }
        if curr_col > 0 && map[curr_row][curr_col - 1].eq(&target) {
            problems.push((curr_row, { curr_col - 1 }));
        }
        if curr_col < *max_col && map[curr_row][curr_col + 1].eq(&target) {
            problems.push((curr_row, { curr_col + 1 }));
        }
    }

    region_set
}

fn calculate_perimiter(region: &HashSet<(usize, usize)>) -> usize {
    region
        .iter()
        .map(|item| {
            let (curr_row, curr_col) = item;
            let mut count = 0;
            if region.contains(&(curr_row + 1, *curr_col)) {
                count += 1;
            }
            if region.contains(&(*curr_row, curr_col + 1)) {
                count += 1;
            }
            if curr_row > &0 && region.contains(&(curr_row - 1, *curr_col)) {
                count += 1;
            }
            if curr_col > &0 && region.contains(&(*curr_row, curr_col - 1)) {
                count += 1;
            }
            4 - count
        })
        .sum::<usize>()
}

fn part1(data: &str) {
    let map = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;

    let mut seen_points: HashSet<(usize, usize)> = HashSet::new();
    let mut ans: usize = 0;
    map.iter().enumerate().for_each(|(row, row_data)| {
        row_data.iter().enumerate().for_each(|(col, _chr)| {
            if !seen_points.contains(&(row, col)) {
                let region = explore_region(&row, &col, &max_row, &max_col, &map);
                region.iter().for_each(|item| {
                    seen_points.insert(item.clone());
                });
                let peri = calculate_perimiter(&region);
                let area = region.len();
                ans += peri * area;
            }
        })
    });

    println!("{ans}");
}
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Pr,
    Nr,
    Pc,
    Nc,
}

fn cw(dir: &Direction) -> Direction {
    match dir {
        Direction::Pr => Direction::Nc,
        Direction::Nr => Direction::Pc,
        Direction::Pc => Direction::Pr,
        Direction::Nc => Direction::Nr,
    }
}

fn ccw(dir: &Direction) -> Direction {
    match dir {
        Direction::Nr => Direction::Nc,
        Direction::Pr => Direction::Pc,
        Direction::Nc => Direction::Pr,
        Direction::Pc => Direction::Nr,
    }
}

fn block_in_direction(
    curr_row: &usize,
    curr_col: &usize,
    region: &HashSet<(usize, usize)>,
    direction: &Direction,
) -> bool {
    match direction {
        Direction::Pr => region.contains(&(curr_row + 1, *curr_col)),
        Direction::Nr => curr_row != &0 && region.contains(&(curr_row - 1, *curr_col)),
        Direction::Pc => region.contains(&(*curr_row, curr_col + 1)),
        Direction::Nc => curr_col != &0 && region.contains(&(*curr_row, curr_col - 1)),
    }
}

fn get_next_border_state(
    curr_row: &usize,
    curr_col: &usize,
    region: &HashSet<(usize, usize)>,
    direction: &Direction,
    potential_holes: &mut HashSet<(usize, usize)>,
) -> (Direction, (usize, usize)) {
    if block_in_direction(curr_row, curr_col, region, &ccw(direction)) {
        return (
            ccw(direction),
            match ccw(direction) {
                Direction::Pr => (curr_row + 1, *curr_col),
                Direction::Nr => (curr_row - 1, *curr_col),
                Direction::Pc => (*curr_row, curr_col + 1),
                Direction::Nc => (*curr_row, curr_col - 1),
            },
        );
    } else if block_in_direction(curr_row, curr_col, region, direction) {
        if !block_in_direction(curr_row, curr_col, region, &cw(direction)) {
            match cw(direction) {
                Direction::Pr => {
                    potential_holes.insert((curr_row + 1, *curr_col));
                }
                Direction::Nr => {
                    if curr_row > &0 {
                        potential_holes.insert((curr_row - 1, *curr_col));
                    }
                }
                Direction::Pc => {
                    potential_holes.insert((*curr_row, curr_col + 1));
                }
                Direction::Nc => {
                    if curr_col > &0 {
                        potential_holes.insert((*curr_row, curr_col - 1));
                    }
                }
            };
            return (
                *direction,
                match direction {
                    Direction::Pr => (curr_row + 1, *curr_col),
                    Direction::Nr => (curr_row - 1, *curr_col),
                    Direction::Pc => (*curr_row, curr_col + 1),
                    Direction::Nc => (*curr_row, curr_col - 1),
                },
            );
        }
        return (
            *direction,
            match direction {
                Direction::Pr => (curr_row + 1, *curr_col),
                Direction::Nr => (curr_row - 1, *curr_col),
                Direction::Pc => (*curr_row, curr_col + 1),
                Direction::Nc => (*curr_row, curr_col - 1),
            },
        );
    } else if block_in_direction(curr_row, curr_col, region, &cw(direction)) {
        return (
            cw(direction),
            match cw(direction) {
                Direction::Pr => (curr_row + 1, *curr_col),
                Direction::Nr => (curr_row - 1, *curr_col),
                Direction::Pc => (*curr_row, curr_col + 1),
                Direction::Nc => (*curr_row, curr_col - 1),
            },
        );
    } else {
        return (cw(direction), (*curr_row, *curr_col));
    }
}

fn hole_in_direction(
    curr_row: &usize,
    curr_col: &usize,
    region: &HashSet<(usize, usize)>,
    direction: &Direction,
) -> bool {
    match direction {
        Direction::Pr => !region.contains(&(curr_row + 1, *curr_col)),
        Direction::Nr => !region.contains(&(curr_row - 1, *curr_col)),
        Direction::Pc => !region.contains(&(*curr_row, curr_col + 1)),
        Direction::Nc => !region.contains(&(*curr_row, curr_col - 1)),
    }
}

fn get_next_hole_border_state(
    curr_row: &usize,
    curr_col: &usize,
    region: &HashSet<(usize, usize)>,
    direction: &Direction,
) -> (Direction, (usize, usize)) {
    if hole_in_direction(curr_row, curr_col, region, &ccw(direction)) {
        return (
            ccw(direction),
            match ccw(direction) {
                Direction::Pr => (curr_row + 1, *curr_col),
                Direction::Nr => (curr_row - 1, *curr_col),
                Direction::Pc => (*curr_row, curr_col + 1),
                Direction::Nc => (*curr_row, curr_col - 1),
            },
        );
    } else if hole_in_direction(curr_row, curr_col, region, direction) {
        return (
            *direction,
            match direction {
                Direction::Pr => (curr_row + 1, *curr_col),
                Direction::Nr => (curr_row - 1, *curr_col),
                Direction::Pc => (*curr_row, curr_col + 1),
                Direction::Nc => (*curr_row, curr_col - 1),
            },
        );
    } else if hole_in_direction(curr_row, curr_col, region, &cw(direction)) {
        return (
            cw(direction),
            match cw(direction) {
                Direction::Pr => (curr_row + 1, *curr_col),
                Direction::Nr => (curr_row - 1, *curr_col),
                Direction::Pc => (*curr_row, curr_col + 1),
                Direction::Nc => (*curr_row, curr_col - 1),
            },
        );
    } else {
        return (cw(direction), (*curr_row, *curr_col));
    }
}

fn calculate_side_count(region: &HashSet<(usize, usize)>) -> usize {
    let mut seen_positions: HashSet<(Direction, (usize, usize))> = HashSet::new();
    let mut side_changes: usize = 0;

    let mut curr_position = region
        .iter()
        .min_by(|a, b| {
            let row_comp = a.0.cmp(&b.0);
            match row_comp {
                std::cmp::Ordering::Equal => a.1.cmp(&b.1),
                _ => row_comp,
            }
        })
        .unwrap()
        .clone();
    // don't even ask why this is here just know it's needed
    let mut potential_holes: HashSet<(usize, usize)> = HashSet::new();

    let mut curr_direction = Direction::Pc;
    while !seen_positions.contains(&(curr_direction, curr_position)) {
        seen_positions.insert((curr_direction, curr_position.clone()));
        let (next_direction, next_position) = get_next_border_state(
            &curr_position.0,
            &curr_position.1,
            region,
            &curr_direction,
            &mut potential_holes,
        );

        if next_direction != curr_direction {
            side_changes += 1;

            if next_position != curr_position {}
        }
        curr_direction = next_direction;
        curr_position = next_position;
    }
    let definite_holes = potential_holes
        .into_iter()
        .filter(|(curr_row, curr_col)| {
            block_in_direction(curr_row, curr_col, region, &Direction::Nc)
                && block_in_direction(curr_row, curr_col, region, &Direction::Nr)
                && block_in_direction(curr_row, curr_col, region, &Direction::Pc)
                && block_in_direction(curr_row, curr_col, region, &Direction::Pr)
        })
        .collect::<HashSet<(usize, usize)>>();
    let ext_border_positions = seen_positions
        .iter()
        .cloned()
        .map(|(_dir, coord)| coord)
        .collect::<HashSet<(usize, usize)>>();

    let internal_positions = region.difference(&ext_border_positions);

    let filtered_missing_neighbors_internally = internal_positions
        .into_iter()
        .cloned()
        .filter(|point| {
            let (curr_row, curr_col) = point;

            let mut count = 0;
            if region.contains(&(curr_row + 1, *curr_col)) {
                count += 1;
            }
            if region.contains(&(*curr_row, curr_col + 1)) {
                count += 1;
            }
            if curr_row > &0 && region.contains(&(curr_row - 1, *curr_col)) {
                count += 1;
            }
            if curr_col > &0 && region.contains(&(*curr_row, curr_col - 1)) {
                count += 1;
            }
            4 > count
        })
        .collect::<HashSet<(usize, usize)>>();

    if filtered_missing_neighbors_internally.len() == 0 {
        return side_changes;
    }
    let mut seen_hole_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut found_holes = filtered_missing_neighbors_internally
        .iter()
        .map(|point| {
            let mut holes = vec![];
            if hole_in_direction(&point.0, &point.1, region, &Direction::Pr) {
                holes.push((point.0 + 1, point.1));
            }
            if hole_in_direction(&point.0, &point.1, region, &Direction::Nr) {
                holes.push((point.0 - 1, point.1));
            }
            if hole_in_direction(&point.0, &point.1, region, &Direction::Pc) {
                holes.push((point.0, point.1 + 1));
            }
            if hole_in_direction(&point.0, &point.1, region, &Direction::Nc) {
                holes.push((point.0, point.1 - 1));
            }
            holes
        })
        .flatten()
        .collect::<HashSet<(usize, usize)>>();

    found_holes = found_holes
        .union(&definite_holes)
        .cloned()
        .collect::<HashSet<(usize, usize)>>();

    for hole in found_holes {
        if seen_hole_positions.contains(&hole) {
            continue;
        }
        let mut curr_seen_holes_set: HashSet<(Direction, (usize, usize))> = HashSet::new();
        let mut curr_hole_direction = Direction::Pr;
        if block_in_direction(&hole.0, &hole.1, region, &Direction::Nc) {
            curr_hole_direction = Direction::Nr;
        } else if block_in_direction(&hole.0, &hole.1, region, &Direction::Nr) {
            curr_hole_direction = Direction::Pc;
        } else if block_in_direction(&hole.0, &hole.1, region, &Direction::Pc) {
            curr_hole_direction = Direction::Pr;
        } else if block_in_direction(&hole.0, &hole.1, region, &Direction::Pr) {
            curr_hole_direction = Direction::Nc;
        }
        let mut curr_hole_position = hole.clone();
        while !curr_seen_holes_set.contains(&(curr_hole_direction, curr_hole_position)) {
            curr_seen_holes_set.insert((curr_hole_direction, curr_hole_position));
            let (next_hole_direction, next_hole_position) = get_next_hole_border_state(
                &curr_hole_position.0,
                &curr_hole_position.1,
                region,
                &curr_hole_direction,
            );
            if next_hole_direction != curr_hole_direction {
                side_changes += 1;
            }
            curr_hole_direction = next_hole_direction;
            curr_hole_position = next_hole_position;
        }

        curr_seen_holes_set
            .iter()
            .map(|(_, coord)| coord)
            .for_each(|coord| {
                seen_hole_positions.insert(coord.clone());
            });
    }
    side_changes
}

fn part2(data: &str) {
    let map = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;
    let mut seen_points: HashSet<(usize, usize)> = HashSet::new();
    let mut ans: usize = 0;
    map.iter().enumerate().for_each(|(row, row_data)| {
        row_data.iter().enumerate().for_each(|(col, _chr)| {
            if !seen_points.contains(&(row, col)) {
                let region = explore_region(&row, &col, &max_row, &max_col, &map);
                region.iter().for_each(|item| {
                    seen_points.insert(item.clone());
                });
                let area = region.len();
                let side_count = calculate_side_count(&region);
                ans += side_count * area;
            }
        })
    });

    println!("{ans}");
}
