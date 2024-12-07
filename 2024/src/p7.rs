use advent_of_code_2024::{read_file, Part, Which};
pub fn p7(choice: Which, part: Part) {
    let file_data: String = read_file(7, choice, None);
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
    let problems = data
        .lines()
        .map(|line| {
            let res = line
                .split(':')
                .nth(0)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            let nums = line
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|spl| spl.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (res, nums)
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    let mut result: usize = 0;
    for (target, nums) in problems {
        let mut stack_to_check: Vec<(usize, usize, usize)> = vec![];
        stack_to_check.push((target, 1, nums[0]));
        let max_idx = nums.len() - 1;
        while !stack_to_check.is_empty() {
            let (target, curr_idx, curr_total) = stack_to_check.pop().unwrap();
            let curr_num = nums[curr_idx];
            if curr_idx == max_idx {
                if target == curr_total + curr_num || target == curr_total * curr_num {
                    result += target;
                    break;
                }
                continue;
            }
            // add branch
            let add_next_total = curr_total + curr_num;
            let next_index = curr_idx + 1;
            if add_next_total <= target {
                stack_to_check.push((target, next_index, add_next_total));
            }
            // mul branch
            let mul_next_total = curr_total * curr_num;
            if mul_next_total <= target {
                stack_to_check.push((target, next_index, mul_next_total));
            }
        }
    }

    println!("{result}");
}

fn concat(left: usize, right: usize) -> usize {
    let mut left_str = left.to_string();
    left_str.push_str(right.to_string().as_str());
    left_str.parse::<usize>().unwrap()
}
fn part2(data: &str) {
    let problems = data
        .lines()
        .map(|line| {
            let res = line
                .split(':')
                .nth(0)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            let nums = line
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|spl| spl.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (res, nums)
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    let mut result: usize = 0;
    for (target, nums) in problems {
        let mut stack_to_check: Vec<(usize, usize, usize)> = vec![];
        stack_to_check.push((target, 1, nums[0]));
        let max_idx = nums.len() - 1;
        while !stack_to_check.is_empty() {
            let (target, curr_idx, curr_total) = stack_to_check.pop().unwrap();
            let curr_num = nums[curr_idx];
            if curr_idx == max_idx {
                if target == curr_total + curr_num
                    || target == curr_total * curr_num
                    || target == concat(curr_total, curr_num)
                {
                    result += target;
                    break;
                }
                continue;
            }
            // add branch
            let add_next_total = curr_total + curr_num;
            let next_index = curr_idx + 1;
            if add_next_total <= target {
                stack_to_check.push((target, next_index, add_next_total));
            }
            // mul branch
            let mul_next_total = curr_total * curr_num;
            if mul_next_total <= target {
                stack_to_check.push((target, next_index, mul_next_total));
            }
            // concat branch
            let concat_next_total = concat(curr_total, curr_num);
            if concat_next_total <= target {
                stack_to_check.push((target, next_index, concat_next_total));
            }
        }
    }

    println!("{result}");
}
