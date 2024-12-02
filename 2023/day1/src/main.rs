use std::fs::read_to_string;
use std::collections::HashMap;
fn main() {
    println!(
        "part 1: {}",
        decode_file_part1(&"C:\\Users\\65434\\Documents\\AdventOfCode\\p1\\src\\input.txt")
    );
    println!(
        "part 2: {}",
        decode_file_part2(&"C:\\Users\\65434\\Documents\\AdventOfCode\\p1\\src\\input.txt")
    );
}

fn decode_file_part1(filepath: &str) -> u32 {
    let mut running_total: u32 = 0;
    read_to_string(filepath)
        .unwrap()
        .lines()
        .into_iter()
        .for_each(|line| running_total += decode_line_part1(&line));
    running_total
}

fn decode_line_part1(line_input: &str) -> u32 {
    let first = line_input
        .chars()
        .find(|c: &char| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap();
    let last = line_input
        .chars()
        .rfind(|c: &char| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap();

    10 * first + last
}

fn decode_file_part2(filepath: &str) -> u32 {
    let str_patterns = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let str_map = str_patterns
        .iter()
        .cloned()
        .enumerate()
        .map(|(a, b)| (b, { a.clone() + 1 } as u32))
        .collect::<HashMap<&str, u32>>();
    let mut running_total: u32 = 0;
    read_to_string(filepath)
        .unwrap()
        .lines()
        .into_iter()
        .for_each(|line| running_total += decode_line_part2(&line, &str_map));
    running_total
}

fn decode_line_part2(line_input: &str, str_map: &HashMap<&str, u32>) -> u32 {
    // find first num and/or first digit
    let min_digit_index = line_input.find(|c: char| c.is_ascii_digit()).unwrap();
    let min_digit_from_str = str_map
        .keys()
        .map(|pat| match line_input.find(pat) {
            Some(num) => (num, pat, true),
            None => (usize::MAX, pat, false),
        })
        .filter(|(_, _, truth)| *truth)
        .min()
        .clone();

    let start: u32 = match min_digit_from_str {
        Some(result) => {
            if result.0 < min_digit_index {
                str_map.get(result.1).unwrap().clone()
            } else {
                line_input
                    .chars()
                    .nth(min_digit_index)
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
            }
        }
        None => line_input
            .chars()
            .nth(min_digit_index)
            .unwrap()
            .to_digit(10)
            .unwrap(),
    };

    // find last num and or last digit
    let max_digit_index = line_input.rfind(|c: char| c.is_ascii_digit()).unwrap();
    let max_digit_from_str = str_map
        .keys()
        .map(|pat| match line_input.rfind(pat) {
            Some(num) => (num, pat, true),
            None => (usize::MIN, pat, false),
        })
        .filter(|(_, _, truth)| *truth)
        .max()
        .clone();

    let end: u32 = match max_digit_from_str {
        Some(result) => {
            if result.0 > max_digit_index {
                str_map.get(result.1).unwrap().clone()
            } else {
                line_input
                    .chars()
                    .nth(max_digit_index)
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
            }
        }
        None => line_input
            .chars()
            .nth(max_digit_index)
            .unwrap()
            .to_digit(10)
            .unwrap(),
    };

    10 * start + end
}
