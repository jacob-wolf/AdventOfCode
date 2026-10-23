use advent_of_code_2025::{read_file, Part, Which};
use std::collections::HashSet;

pub fn p5(choice: Which, part: Part) {
    let file_data: String = read_file(5, choice, None);
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

fn part1(data: &str) {
    // count fresh ingredients
    let ranges = data.split("\r\n\r\n").take(1).collect::<Vec<&str>>()[0];
    let ingredients = data.split("\r\n\r\n").skip(1).collect::<Vec<&str>>()[0];

    let mut valid_set = HashSet::new();
    let mut fresh_count = 0;

    ranges.lines().for_each(|line| {
        let min_max = line
            .split('-')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        valid_set.insert((min_max[0], min_max[1]));
    });

    ingredients.lines().for_each(|line| {
        let num = line.parse::<usize>().unwrap();

        if valid_set
            .iter()
            .any(|min_max| num >= min_max.0 && num <= min_max.1)
        {
            fresh_count += 1;
        }
    });
    println!("Part 1: {fresh_count}")
}
fn part2(data: &str) {
    // count unique fresh ingredient ids

    let ranges = data.split("\r\n\r\n").take(1).collect::<Vec<&str>>()[0];
    let mut valid_set: HashSet<(usize, usize)> = HashSet::new();
    ranges.lines().for_each(|line| {
        let min_max = line
            .split('-')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        valid_set.insert((min_max[0], min_max[1]));
    });

    let mut found_overlap = true;
    while found_overlap {
        found_overlap = false;
        let cloned_valid_set = valid_set.clone();
        cloned_valid_set
            .iter()
            .enumerate()
            .for_each(|(pair_idx, pair)| {
                if found_overlap {
                    return;
                };
                cloned_valid_set
                    .iter()
                    .skip(pair_idx + 1)
                    .for_each(|sub_pair| {
                        if found_overlap {
                            return;
                        };
                        // check if sub pair overlaps with pair
                        if pair.0 >= sub_pair.0 && pair.0 <= sub_pair.1 {
                            // overlap!
                            found_overlap = true;
                            valid_set.remove(pair);
                            valid_set.remove(sub_pair);
                            if pair.1 > sub_pair.1 {
                                valid_set.insert((sub_pair.0, pair.1));
                            } else {
                                valid_set.insert((sub_pair.0, sub_pair.1));
                            }
                        } else if pair.1 >= sub_pair.0 && pair.1 <= sub_pair.1 {
                            found_overlap = true;
                            valid_set.remove(pair);
                            valid_set.remove(sub_pair);
                            if pair.0 < sub_pair.0 {
                                valid_set.insert((pair.0, sub_pair.1));
                            } else {
                                valid_set.insert((sub_pair.0, sub_pair.1));
                            }
                        } else if pair.0 <= sub_pair.0 && pair.1 >= sub_pair.1 {
                            found_overlap = true;
                            // can just scrap the completely covered range
                            valid_set.remove(sub_pair);
                        }
                    })
            });
    }

    let mut count = 0;

    valid_set.iter().for_each(|(pair_low, pair_hi)| {
        count += 1 + pair_hi - pair_low;
    });

    println!("P2: {count}");
}
