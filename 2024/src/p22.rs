use std::collections::{HashMap, HashSet};

use advent_of_code_2024::{read_file, Part, Which};

pub fn p22(choice: Which, part: Part) {
    let file_data: String = read_file(22, choice, Some(part));
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

fn next_num(curr: &isize) -> isize {
    let s1 = { (curr * 64) ^ curr } % 16777216;
    let s2 = { (s1 / 32) ^ s1 } % 16777216;
    let s3 = { (s2 * 2048) ^ s2 } % 16777216;
    s3
}

fn part1(data: &str) {
    let mut sum = 0;
    data.lines()
        .map(|line| line.trim().parse::<isize>().unwrap())
        .for_each(|start| {
            let mut num = start;
            for _ in 0..2000 {
                num = next_num(&num);
            }
            sum += num;
        });
    println!("{sum}");
}

fn part2(data: &str) {
    let mut prices = vec![];
    let monkey_pricing_changes = data
        .lines()
        .map(|line| line.trim().parse::<isize>().unwrap())
        .map(|start| {
            let mut list = vec![start % 10];
            let mut curr = start;
            for _ in 0..2000 {
                curr = next_num(&curr);
                list.push(curr % 10);
            }
            prices.push(list.clone());
            list.windows(2).map(|x| x[1] - x[0]).collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();
    // populate each sequence with the sell price
    let mut sequence_map: HashMap<[isize; 4], isize> = HashMap::new();
    monkey_pricing_changes
        .iter()
        .enumerate()
        .for_each(|(price_idx, changes)| {
            let mut seen_change_set: HashSet<[isize; 4]> = HashSet::new();
            changes
                .windows(4)
                .map(|window| [window[0], window[1], window[2], window[3]])
                .enumerate()
                .for_each(|(window_idx, window)| {
                    if !seen_change_set.contains(&window) {
                        seen_change_set.insert(window);
                        // CHANGE THIS COUNT TO GET THE PRICE AFTER THIS SEQUENCE\
                        let curr_price = prices[price_idx][window_idx + 4];
                        let acc_price = sequence_map.get(&window).unwrap_or(&0);
                        sequence_map.insert(window, acc_price + curr_price);
                    }
                });
        });

    let mut pairs: Vec<(&[isize; 4], &isize)> =
        sequence_map.iter().collect::<Vec<(&[isize; 4], &isize)>>();
    pairs.sort_by(|a, b| b.1.cmp(a.1));
    println!("{:?}", pairs[0].1);
}
