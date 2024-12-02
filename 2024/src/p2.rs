use advent_of_code_2024::{read_file, Part, Which};
pub fn p2(choice: Which, part: Part) {
    let file_data: String = read_file(2, choice, None);
    match part {
        Part::One => part1(file_data),
        Part::Two => part2(file_data),
    }
}

fn part1(data: String) {
    let mut nums: Vec<Vec<i32>> = vec![];
    data.lines().for_each(|line| {
        let mut x: Vec<i32> = vec![];
        line.split_ascii_whitespace().for_each(|item| {
            x.push(item.parse::<i32>().unwrap());
        });
        nums.push(x);
    });
    let mut safety_count = 0;

    for (_index, line) in nums.iter().enumerate() {
        let gaps = line
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<i32>>();
        let is_ascending = gaps.iter().sum::<i32>() > 0;
        let mut is_valid = true;
        for (_index, gap) in gaps.iter().enumerate() {
            if gap.abs() < 1 || gap.abs() > 3 {
                is_valid = false;
                break;
            }
            match is_ascending {
                true => {
                    if gap < &0 {
                        is_valid = false;
                        break;
                    }
                }
                false => {
                    if gap > &0 {
                        is_valid = false;
                        break;
                    }
                }
            };
        }
        if is_valid {
            safety_count += 1;
        }
    }

    println!("{safety_count}");
}
fn part2(data: String) {
    // Find first problem gap and merge with its neighbor to the left
    // detect the first problematic gap
    // merge left with neighbor
    // run same algorithm afterwards

    // possible gaps
    // 2 -1 2 1
    // -2 -2 0 -3
    // 7 2 1 1 1 0
    // 0 1 3 3 2 2

    // problem gaps are 0 or opposite from expected sign. Problem gaps that are to large with correct sign are unfixable

    let mut nums: Vec<Vec<i32>> = vec![];
    data.lines().for_each(|line| {
        let mut x: Vec<i32> = vec![];
        line.split_ascii_whitespace().for_each(|item| {
            x.push(item.parse::<i32>().unwrap());
        });
        nums.push(x);
    });
    let mut safety_count = 0;

    for (_index, line) in nums.iter().enumerate() {
        let gaps = line
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<i32>>();

        let is_ascending = gaps.iter().sum::<i32>() > 0;

        let mut is_valid = true;

        let invalid_gaps = gaps
            .iter()
            .enumerate()
            .filter(|(_gap_index, gap)| {
                if gap.abs() > 3 || gap.abs() < 1 {
                    return true;
                }
                match is_ascending {
                    true => **gap < 0,
                    false => **gap > 0,
                }
            })
            .collect::<Vec<(usize, &i32)>>();

        let l = invalid_gaps.len();
        if l >= 3 {
            // can't fix 3 invalid gaps
            is_valid = false;
        } else if l == 2 {
            let index_0 = invalid_gaps[0].0;
            let index_1 = invalid_gaps[1].0;
            let gap_0 = invalid_gaps[0].1;
            let gap_1 = invalid_gaps[1].1;

            // to fix both problem gaps need these next to each other and need them to sum to a valid gap
            let gap_distance = index_1 - index_0;
            let gap_sum = gap_0 + gap_1;
            if gap_distance != 1 {
                is_valid = false;
            } else if gap_sum.abs() > 3 || gap_sum == 0 {
                //merging the gaps (eliminating the problem number still doesn't solve this issue)
                is_valid = false;
            } else {
                match is_ascending {
                    true => {
                        if gap_sum < 0 {
                            is_valid = false;
                        }
                    }
                    false => {
                        if gap_sum > 0 {
                            is_valid = false;
                        }
                    }
                }
            }
        } else if l == 1 {
            // need to check if merging left or right will make this a valid gap
            // if merge left and index 0 then can just drop it
            // if merge right and last item then can just drop it
            let (problem_gap_index, problem_gap) = invalid_gaps[0];

            let left_neighbor_gap = if problem_gap_index > 0 {
                Some(gaps[problem_gap_index - 1])
            } else {
                None
            };
            let right_neighbor_gap = if problem_gap_index < gaps.len() - 1 {
                Some(gaps[problem_gap_index + 1])
            } else {
                None
            };
            
            if let None = left_neighbor_gap {
                is_valid = true;
            } else if let None = right_neighbor_gap {
                is_valid = true;
            } else { 
                // need to check both left and right
                let left_gap = left_neighbor_gap.unwrap();
                let right_gap = right_neighbor_gap.unwrap();

                let check_left: bool = {
                    let gap_sum = left_gap + problem_gap;
                    if gap_sum.abs() > 3 || gap_sum == 0 {
                        false
                    } else {
                        match is_ascending {
                            true => {
                                gap_sum > 0
                            }
                            false => {
                                gap_sum < 0
                            }
                        }
                    }
                };
                let check_right: bool = {
                    let gap_sum = right_gap + problem_gap;
                    if gap_sum.abs() > 3 || gap_sum == 0 {
                        false
                    } else {
                        match is_ascending {
                            true => {
                                gap_sum > 0
                            }
                            false => {
                                gap_sum < 0
                            }
                        }
                    }
                };
                if check_left || check_right {
                    is_valid = true;
                } else {
                    is_valid = false;
                }
            }
        }

        if is_valid {
            safety_count += 1;
        }
    }

    println!("{safety_count}");
}
