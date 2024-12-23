use advent_of_code_2024::{read_file, Part, Which};

pub fn p19(choice: Which, part: Part) {
    let file_data: String = read_file(19, choice, None);
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
    let towels = data
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|towel| towel.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let desired = data
        .lines()
        .skip(2)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut solution_count = 0;

    desired.iter().for_each(|target| {
        let mut reachable_by_idx = vec![false; target.len() + 1];
        reachable_by_idx[0] = true;

        for idx in 1..target.len() + 1 {
            for towel in towels.iter() {
                if idx >= towel.len() && target[idx - towel.len()..idx].eq(&towel[..]) {
                    reachable_by_idx[idx] = reachable_by_idx[idx] || reachable_by_idx[idx - towel.len()];
                }
            }
        }
        solution_count += match reachable_by_idx[target.len()] {
            true => 1,
            false => 0,
        }
    });
    println!("{solution_count}");
}

fn part2(data: &str) {
    let towels = data
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|towel| towel.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let desired = data
        .lines()
        .skip(2)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut solution_count = 0;
    desired.iter().for_each(|target| {
        let mut count_by_idx = vec![0 as usize; target.len() + 1];
        count_by_idx[0] = 1; // 1 way to reach idx 0

        for idx in 1..target.len() + 1 {
            for towel in towels.iter() {
                if idx >= towel.len() && target[idx - towel.len()..idx].eq(&towel[..]) {
                    count_by_idx[idx] += count_by_idx[idx - towel.len()];
                }
            }
        }
        solution_count += count_by_idx[target.len()];
    });
    println!("{solution_count}");
}
