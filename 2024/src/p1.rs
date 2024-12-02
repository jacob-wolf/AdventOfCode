use std::collections::HashMap;

use advent_of_code_2024::{read_file, Part, Which};
pub fn p1(choice: Which, part: Part) {
    let file_data: String = read_file(1, choice, None);
    match part {
        Part::One => part1(file_data),
        Part::Two => part2(file_data),
    }
}

fn part1(data: String) {
    let mut right_list: Vec<isize> = vec![];
    let mut left_list: Vec<isize> = vec![];

    data.lines().for_each(|line| {
        let line_nums = line.split_ascii_whitespace().collect::<Vec<&str>>();
        left_list.push(line_nums[0].parse::<isize>().unwrap());
        right_list.push(line_nums[1].parse::<isize>().unwrap());
    });

    left_list.sort();
    right_list.sort();

    let mut distance: isize = 0;

    for index in 0..left_list.len() {
        distance += { right_list[index] - left_list[index] }.abs();
    }
    println!("{distance}");
}
fn part2(data: String) {
    let mut left_to_right_map: HashMap<isize, isize> = HashMap::new();
    let mut left_list: Vec<isize> = vec![];
    data.lines().for_each(|line| {
        let line_nums = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let left_val = line_nums[0].parse::<isize>().unwrap();
        left_list.push(left_val);
        let right_val = line_nums[1].parse::<isize>().unwrap();

        if let None = left_to_right_map.get_mut(&left_val) {
            left_to_right_map.insert(left_val, 0);
        }

        if let Some(val) = left_to_right_map.get(&right_val) {
            left_to_right_map.insert(right_val, *val + 1);
        } else {
            left_to_right_map.insert(right_val, 1);
        }
    });

    let mut similarity: isize = 0;
    for index in 0..left_list.len() {
        let p1 = left_list[index];
        let p2 = left_to_right_map.get(&left_list[index]).unwrap();
        similarity += p1 * p2;
    }

    println!("{similarity}");
}
