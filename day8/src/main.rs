use ahash::RandomState;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    println!("{:?}", part1("input.txt"));
    println!("{:?}", part2("input.txt"));
}

fn part1(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();

    let step_directions = file.lines().next().unwrap().chars().collect::<Vec<char>>();
    let step_cycle_length = step_directions.len();

    let mut map: HashMap<&str, Vec<&str>, RandomState> = HashMap::default();
    file.lines().skip(2).for_each(|line| {
        let mut lines = line.split('=').map(|item| item.trim());
        let a = lines.next().unwrap().chars().as_str();
        let b_item = lines.next().unwrap();
        let mut chars = b_item.chars();
        chars.next();
        chars.next_back();
        let b = chars
            .as_str()
            .split(',')
            .map(|item: &str| item.trim())
            .collect::<Vec<&str>>();
        map.insert(a, b.clone());
    });

    let mut counter: usize = 0;
    let mut _curr_step_direction: char = step_directions[counter];
    let mut curr_position: &str = "AAA";

    while curr_position != "ZZZ" {
        _curr_step_direction = step_directions[counter % step_cycle_length];
        if _curr_step_direction == 'L' {
            curr_position = map.get(&curr_position).unwrap()[0];
        } else {
            curr_position = map.get(&curr_position).unwrap()[1];
        }
        counter = counter + 1;
    }

    counter
}

fn part2(path: &str) -> usize {
    let file: String = read_to_string(&path).unwrap();

    let step_directions: Vec<char> = file.lines().next().unwrap().chars().collect::<Vec<char>>();
    let step_cycle_length: usize = step_directions.len();

    let mut map: HashMap<&str, Vec<&str>, RandomState> = HashMap::default();
    file.lines().skip(2).for_each(|line| {
        let mut lines = line.split('=').map(|item| item.trim());
        let a = lines.next().unwrap().chars().as_str();
        let b_item = lines.next().unwrap();
        let mut chars = b_item.chars();
        chars.next();
        chars.next_back();
        let b = chars
            .as_str()
            .split(',')
            .map(|item: &str| item.trim())
            .collect::<Vec<&str>>();
        map.insert(a, b.clone());
    });

    let curr_positions: HashSet<&str, RandomState> = HashSet::from_iter(
        map.keys()
            .cloned()
            .filter(|position| position.ends_with('A')),
    );

    let mut minimum_solutions: Vec<usize> = vec![];

    for position in curr_positions.iter().cloned() {
        let mut current_position = position;
        let mut counter: usize = 0;
        let mut _curr_step_direction: char = step_directions[counter];
        while !current_position.ends_with('Z') {
            _curr_step_direction = step_directions[counter % step_cycle_length];
            if _curr_step_direction == 'L' {
                current_position = map.get(current_position).unwrap()[0];
            } else {
                current_position = map.get(current_position).unwrap()[1];
            }
            counter = counter + 1;
        }
        minimum_solutions.push(counter.clone());
    }
    let mut lcm: usize = 1;
    for number in minimum_solutions {
        lcm = number * lcm / gcd_loop(number, lcm);
    }

    lcm
}

fn gcd_loop(a: usize, b: usize) -> usize {
    let mut a_mut = a;
    let mut b_mut = b;
    while b_mut != 0 {
        let c = a_mut.clone();
        a_mut = b_mut.clone();
        b_mut = c.clone() % b_mut;
    }
    a_mut
}
