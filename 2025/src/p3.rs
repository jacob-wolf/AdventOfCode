use advent_of_code_2025::{read_file, Part, Which};

pub fn p3(choice: Which, part: Part) {
    let file_data: String = read_file(3, choice, None);
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
    // row is a bank
    // num is a voltage level 1-9
    // pick two volt levels in each row and add them up power wise (left hand side num 10^1 * value + right hand side num 10^0 * value)
    // find largest num each row can produce
    // then sum them up

    let mut sum = 0;

    data.lines().for_each(|line| {
        let nums: Vec<usize> = line
            .chars()
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect();
        // find largest power num in each row ignoring last digit, then find largest num after that

        let mut largest_found = 0;
        let mut largest_found_idx = 0;

        for (idx, num) in nums.iter().take(nums.len() - 1).enumerate() {
            if num > &largest_found {
                largest_found = *num;
                largest_found_idx = idx;
            }
        }

        let mut second_largest_found = 0;

        for num in nums.iter().skip(largest_found_idx + 1) {
            if num > &second_largest_found {
                second_largest_found = *num;
            }
        }

        sum += 10 * largest_found + second_largest_found;
    });
    println!("part 1: {}", sum);
}
fn part2(data: &str) {
    // now need 12 digits
    let mut sum = 0;

    data.lines().for_each(|line| {
        let nums = line
            .chars()
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        // first num skip last 11 (run the take operation) skip first 0
        // second num skip last 10 (run take operation to get all but last 10) then run skip operation to skip 0..=previous num_idx because can't repeat cells and can't be left
        // etc.

        let mut digits: Vec<usize> = vec![0; 12];
        let mut digit_idxs: Vec<usize> = vec![0; 12];

        for idx in 0..12 {
            let mut skip_amt = 0;
            if idx > 0 {
                skip_amt = digit_idxs[idx - 1] + 1;
            }

            for (num_idx, num) in nums
                .iter()
                .enumerate()
                .take(nums.len() + idx - 11) // take all but last (11 - idx) digits to ensure 12 selections
                .skip(skip_amt) // skip all digits up to and including the digit selected for the previous battery cell
            {
                if num > &digits[idx] {
                    digits[idx] = *num;
                    digit_idxs[idx] = num_idx;
                }
            }
        }
        let mut value = 0;
        for idx in 0..12 {
            value += { 10_u64.pow((11 - idx).try_into().unwrap()) } * digits[idx as usize] as u64;
        }
        sum += value;
    });

    println!("part 2: {sum}");
}
