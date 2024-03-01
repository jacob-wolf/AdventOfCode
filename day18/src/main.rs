use std::{collections::HashSet, fs::read_to_string};
fn main() {
    println!("{}", part1(&"input.txt"));
    println!("{}", part2(&"input.txt"));
}
#[derive(Debug, Copy, Clone)]
enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Copy, Clone)]
struct Plan {
    direction: Dir,
    length: u32,
}

fn part1(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();
    //what is the area of the shape
    let plans: Vec<Plan> = file
        .lines()
        .map(|line| {
            let items = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let direction = match items[0] {
                "R" => Dir::E,
                "L" => Dir::W,
                "U" => Dir::N,
                "D" => Dir::S,
                _ => panic!("Not a direction"),
            };

            let length = items[1].parse::<u32>().unwrap();

            Plan { direction, length }
        })
        .collect::<Vec<Plan>>();

    let mut border_set: HashSet<(isize, isize)> = HashSet::new();
    populate_borders(&plans, &mut border_set);

    analyze_borders(&border_set)
}

fn populate_borders(plans: &Vec<Plan>, border_set: &mut HashSet<(isize, isize)>) {
    let mut curr_position = (0, 0);
    border_set.insert(curr_position.clone());

    for plan in plans {
        for _ in 0..plan.length {
            match plan.direction {
                Dir::N => {
                    curr_position = (curr_position.0 - 1, curr_position.1);
                }
                Dir::S => {
                    curr_position = (curr_position.0 + 1, curr_position.1);
                }
                Dir::E => {
                    curr_position = (curr_position.0, curr_position.1 + 1);
                }
                Dir::W => {
                    curr_position = (curr_position.0, curr_position.1 - 1);
                }
            }
            border_set.insert(curr_position.clone());
        }
    }
}

fn analyze_borders(border_set: &HashSet<(isize, isize)>) -> usize {
    let row_min = *border_set.iter().map(|(r, _c)| r).min().unwrap();
    let row_max = *border_set.iter().map(|(r, _c)| r).max().unwrap();
    let col_min = *border_set.iter().map(|(_r, c)| c).min().unwrap();
    let col_max = *border_set.iter().map(|(_r, c)| c).max().unwrap();

    let mut count: usize = 0;

    for row in row_min..row_max + 1 {
        let mut is_inside = false;
        let mut entry_option: Option<Dir> = None;

        for col in col_min..col_max + 1 {
            if border_set.contains(&(row, col)) {
                //on the border
                if let None = entry_option {
                    //if both above and below definitely crossed
                    if border_set.contains(&(row - 1, col)) && border_set.contains(&(row - 1, col))
                    {
                        is_inside = !is_inside;
                    } else {
                        if border_set.contains(&(row - 1, col)) {
                            entry_option = Some(Dir::N)
                        } else {
                            entry_option = Some(Dir::S)
                        }
                    }
                }
                //println!("found ({row},{col})");
                count += 1;
            } else {
                //not on the border
                if let Some(entry_detection) = entry_option {
                    match entry_detection {
                        Dir::N => {
                            if border_set.contains(&(row + 1, col - 1)) {
                                is_inside = !is_inside;
                            }
                        }
                        Dir::S => {
                            if border_set.contains(&(row - 1, col - 1)) {
                                is_inside = !is_inside;
                            }
                        }
                        _ => panic!("shouldn't get here..."),
                    }
                    entry_option = None;
                }

                if is_inside {
                    //not on the border, check if is inside
                    //println!("found ({row},{col})");
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(path: &str) -> isize {
    let file = read_to_string(&path).unwrap();

    let plans: Vec<Plan> = file
        .lines()
        .map(|line| {
            let items = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let rgb_color = items[2].trim_matches(|c: char| c.eq(&'(') || c.eq(&')'));

            let direction = match rgb_color.chars().nth(6).unwrap() {
                '0' => Dir::E,
                '1' => Dir::S,
                '2' => Dir::W,
                '3' => Dir::N,
                _ => panic!(),
            };

            let length = rgb_color
                .chars()
                .enumerate()
                .skip(1)
                .take(5)
                .map(|(index, c)| {
                    //subtract len of the direction number and the #
                    let d = c.to_digit(16).unwrap();
                    let p = { 16 as u32 }.pow({ rgb_color.len() - index - 2 } as u32);
                    d * p
                })
                .sum();

            Plan { direction, length }
        })
        .collect::<Vec<Plan>>();
    trapezoidal_area_method(&plans)
}

fn trapezoidal_area_method(plans: &Vec<Plan>) -> isize {
    let mut coordinate_locations: Vec<(isize, isize)> = vec![(0, 0)];
    let mut curr_coordinate: (isize, isize) = (0, 0);

    let mut running_total_area: isize = 2; // account for the "four missing corners" of area 1/4 but before the half so double it to 2 

    for plan in plans {
        match plan.direction {
            Dir::N => {
                curr_coordinate = (curr_coordinate.0 - plan.length as isize, curr_coordinate.1)
            }
            Dir::S => {
                curr_coordinate = (curr_coordinate.0 + plan.length as isize, curr_coordinate.1)
            }
            Dir::E => {
                curr_coordinate = (curr_coordinate.0, curr_coordinate.1 + plan.length as isize)
            }
            Dir::W => {
                curr_coordinate = (curr_coordinate.0, curr_coordinate.1 - plan.length as isize)
            }
        };
        coordinate_locations.push(curr_coordinate);

        //account for the area of the border, but add it before dividing by 2 because only half a unit is missing
        running_total_area += plan.length as isize; 
    }

    let num_coords = coordinate_locations.len();

    for index in 0..num_coords {
        if index.eq(&{ num_coords - 1 }) {
            let i = coordinate_locations[index];
            let i_1 = coordinate_locations[0];
            running_total_area += { i.1 + i_1.1 } * { i_1.0 - i.0 };
        } else {
            let i = coordinate_locations[index];
            let i_1 = coordinate_locations[index + 1];
            running_total_area += { i.1 + i_1.1 } * { i_1.0 - i.0 };
        }
    }

    let area: isize = { running_total_area.abs() / 2 };
    area
}
