use advent_of_code_2024::{read_file, Part, Which};
use std::collections::HashMap;

pub fn p11(choice: Which, part: Part) {
    let file_data: String = read_file(11, choice, None);
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
    //rules if 0 it becomes 1
    // if even digit count splits into two stones with lh digits and rh digits
    // 2024*curr_val otherwise
    // 25 blinks
    let mut seen_stones: HashMap<usize, Vec<(String, usize)>> = HashMap::new();
    seen_stones.insert(0, vec![(String::from("1"), 1)]);
    let stones = data
        .split_ascii_whitespace()
        .map(|ston| ston.to_string())
        .collect::<Vec<String>>();

    let mut ans = 0;
    let curr_stones = stones
        .into_iter()
        .map(|stone| (0 as usize, stone.clone(), stone.parse::<usize>().unwrap()))
        .collect::<Vec<(usize, String, usize)>>();

    curr_stones
        .iter()
        .cloned()
        .for_each(|(blink_num, str_rep, num_rep)| {
            let mut problems: Vec<(usize, String, usize)> = vec![(blink_num, str_rep, num_rep)];
            while !problems.is_empty() {
                let (curr_blinks, curr_str, curr_num) = problems.pop().unwrap();
                if curr_blinks == 25 {
                    ans += 1;
                    continue;
                }
                let opt_next_ans = seen_stones.get(&curr_num);
                match opt_next_ans {
                    Some(next_stones) => {
                        next_stones
                            .iter()
                            .cloned()
                            .for_each(|(next_str_rep, next_num)| {
                                problems.push((curr_blinks + 1, next_str_rep, next_num));
                            });
                    }
                    None => {
                        if curr_str.len() % 2 == 0 {
                            let splidx = curr_str.len() / 2;
                            let left = curr_str[0..splidx].to_string();
                            let left_num = left.parse::<usize>().unwrap();
                            let right = curr_str[splidx..].to_string();
                            let right_num = right.parse::<usize>().unwrap();

                            seen_stones.insert(
                                curr_num,
                                vec![(left.clone(), left_num), (right_num.to_string(), right_num)],
                            );
                            problems.push((curr_blinks + 1, left, left_num));
                            problems.push((curr_blinks + 1, right_num.to_string(), right_num));
                        } else {
                            let next_num = { curr_num * 2024 };
                            seen_stones.insert(curr_num, vec![(next_num.to_string(), next_num)]);
                            problems.push((curr_blinks + 1, next_num.to_string(), next_num));
                        }
                    }
                };
            }
        });

    println!("{ans}");
}

// above method WAAAYY too slow... number count balloons exponentially (don't want to even read a cache map that much)
// what if only calculate for the unique items
// there aren't actually that many unique stone numbers (all should reduce to single digits)
// 0 -> 1
// 1 -> 2024 -> 20 24 -> 2 0 2 4
// 2 -> 4048 -> 40 48 -> 4 0 4 8
// 3 -> 6072 -> 60 72 -> 6 0 7 2
// 4 -> 8096 -> 80 96 -> 8 0 9 6
// 5 -> 10120 -> 20482880 -> 2048 2880 -> 20 48 28 80 -> 2 0 4 8 2 8 8 0
// 6 -> 12144 -> 24579456 -> 2457 9456 -> 24 57 94 56 -> 2 4 5 7 9 4 5 6
// 7 -> 14168 -> 28676032 -> 2867 6032 -> 28 67 60 32 -> 2 8 6 7 6 0 3 2
// 8 -> 16192 -> 32772608 -> 3277 2608 -> 32 77 26 08 -> 3 2 7 7 2 6 0 8
// 9 -> 18216 -> 36869184 -> 3686 9184 -> 36 86 91 84 -> 3 6 8 6 9 1 8 4

fn part2(data: &str) {
    let starting_rocks = data
        .split_ascii_whitespace()
        .map(|item| item.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut curr_rocks_map: HashMap<usize, usize> = HashMap::new();
    starting_rocks.into_iter().for_each(|item| {
        curr_rocks_map.insert(item, 1);
    });

    let mut blinks = 0;
    while blinks < 75 {
        blinks += 1;
        let mut next_rocks_map: HashMap<usize, usize> = HashMap::new();

        curr_rocks_map.iter().for_each(|(rock_num, rock_count)| {
            if rock_num.eq(&0) {
                match next_rocks_map.get(&1) {
                    Some(prev_ins_next_count) => {
                        next_rocks_map.insert(1, rock_count.clone() + prev_ins_next_count);
                    }
                    None => {
                        next_rocks_map.insert(1, rock_count.clone());
                    }
                }
            } else if rock_num.to_string().len() % 2 == 0 {
                let splidx = rock_num.to_string().len() / 2;
                let left = rock_num.to_string()[0..splidx].parse::<usize>().unwrap();
                match next_rocks_map.get(&left) {
                    Some(prev_ins) => {
                        next_rocks_map.insert(left, prev_ins + rock_count);
                    }
                    None => {
                        next_rocks_map.insert(left, *rock_count);
                    }
                }
                let right = rock_num.to_string()[splidx..].parse::<usize>().unwrap();
                match next_rocks_map.get(&right) {
                    Some(prev_ins) => {
                        next_rocks_map.insert(right, prev_ins + rock_count);
                    }
                    None => {
                        next_rocks_map.insert(right, *rock_count);
                    }
                }
            } else {
                match next_rocks_map.get(&{ 2024 * rock_num }) {
                    Some(prev_ins_next_count) => {
                        next_rocks_map.insert(rock_num * 2024, rock_count + prev_ins_next_count);
                    }
                    None => {
                        next_rocks_map.insert(rock_num * 2024, *rock_count);
                    }
                }
            }
        });

        curr_rocks_map = next_rocks_map;
    }

    let ans = curr_rocks_map
        .iter()
        .map(|(_rock_type, rock_count)| rock_count)
        .sum::<usize>();
    println!("{ans}");
}
