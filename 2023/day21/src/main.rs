use std::{collections::HashSet, fs::read_to_string, hash::Hash};
fn main() {
    let test = (&"test.txt", 6);
    println!("part 1: {}", part1(test.0, test.1));
    println!("part 2: {}", part2(&"test.txt"));
}

enum Plot {
    Start,
    Rock,
    Plot,
}

enum Direction {
    Pr,
    Nr,
    Pc,
    Nc,
}

fn part1(path: &str, max_steps: usize) -> usize {
    let data = read_to_string(&path).unwrap();
    let grid = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| match chr {
                    '.' => Plot::Plot,
                    '#' => Plot::Rock,
                    'S' => Plot::Start,
                    _ => panic!(),
                })
                .collect::<Vec<Plot>>()
        })
        .collect::<Vec<Vec<Plot>>>();

    // bfs grid
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut problems: Vec<(Option<Direction>, (usize, usize), usize)> = vec![];
    while !problems.is_empty() {
        let (prev_direction, (curr_r, curr_c), steps_taken) = problems.pop();
        visited.insert((curr_r, curr_c));
    }
    todo!();
}

fn part2(path: &str) -> usize {
    let data = read_to_string(&path).unwrap();
    todo!();
}
