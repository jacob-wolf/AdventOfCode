use advent_of_code_2024::{read_file, Part, Which};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

pub fn p5(choice: Which, part: Part) {
    let file_data: String = read_file(5, choice, None);
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

fn part1(data: &str) {
    let line_split_idx = data
        .lines()
        .enumerate()
        .find(|(_split_idx, line)| line.trim().is_empty())
        .unwrap()
        .0;
    // map from number to vec of numbers that come after
    let mut rules_map: HashMap<usize, Vec<usize>> = HashMap::new();

    data.lines().take(line_split_idx).for_each(|line| {
        let before = line.split('|').nth(0).unwrap().parse::<usize>().unwrap();
        let after = line.split('|').nth(1).unwrap().parse::<usize>().unwrap();
        if let None = rules_map.get(&before) {
            rules_map.insert(before, vec![]);
        }
        let mut curr_array = rules_map.get(&before).unwrap().clone();
        curr_array.push(after);
        rules_map.insert(before, curr_array);
    });

    let result = data
        .lines()
        .skip(line_split_idx + 1)
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        // filter for valid
        .filter(|update| {
            // put the update numbers in a set
            let mut before_set: HashSet<&usize> = HashSet::from_iter(update.iter());
            for num in update.iter().rev() {
                before_set.remove(num); // remove last element
                if let None = rules_map.get(num) {
                    continue;
                }
                let rules_array = rules_map.get(num).unwrap();
                if rules_array
                    .iter()
                    .any(|element| before_set.contains(&element))
                {
                    // num is last & num should be before => invalid
                    return false;
                }
            }
            true
        })
        .map(|update_arr| {
            let m = update_arr.len() / 2;
            update_arr[m]
        })
        .sum::<usize>();

    println!("{result}");
}
fn part2(data: &str) {
    let line_split_idx = data
        .lines()
        .enumerate()
        .find(|(_split_idx, line)| line.trim().is_empty())
        .unwrap()
        .0;
    // map from number to vec of numbers that come after
    let mut rules_map: HashMap<usize, Vec<usize>> = HashMap::new();

    data.lines().take(line_split_idx).for_each(|line| {
        let before = line.split('|').nth(0).unwrap().parse::<usize>().unwrap();
        let after = line.split('|').nth(1).unwrap().parse::<usize>().unwrap();
        if let None = rules_map.get(&before) {
            rules_map.insert(before, vec![]);
        }
        let mut curr_array = rules_map.get(&before).unwrap().clone();
        curr_array.push(after);
        rules_map.insert(before, curr_array);
    });

    let result = data
        .lines()
        .skip(line_split_idx + 1)
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        // filter for invalid (inverted boolean returns from part 1)
        .filter(|update| {
            // put the update numbers in a set
            let mut before_set: HashSet<&usize> = HashSet::from_iter(update.iter());
            for num in update.iter().rev() {
                before_set.remove(num); // remove last element
                if let None = rules_map.get(num) {
                    continue;
                }
                let rules_array = rules_map.get(num).unwrap();
                if rules_array
                    .iter()
                    .any(|element| before_set.contains(&element))
                {
                    // num is last & num should be before => invalid
                    return true;
                }
            }
            false
        })
        .map(|invalid_update| {
            let mut updates_to_sort = invalid_update.clone();
            updates_to_sort.sort_by(|a, b| {
                if let Some(rules) = rules_map.get(a) {
                    if rules.contains(b) {
                        return Ordering::Less; // a comes before b rule
                    }
                }
                if let Some(rules) = rules_map.get(b) {
                    if rules.contains(a) {
                        return Ordering::Greater; // b comes before a rule
                    }
                }
                return Ordering::Equal; // no rules found
            });
            updates_to_sort
        })
        .map(|update_arr| {
            let m = update_arr.len() / 2;
            update_arr[m]
        })
        .sum::<usize>();

    println!("{result}");
}
