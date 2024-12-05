use advent_of_code_2024::{read_file, Part, Which};
use regex::Regex;

pub fn p3(choice: Which, part: Part) {
    let file_data: String = read_file(3, choice, Some(part));
    match part {
        Part::One => part1(&file_data),
        Part::Two => part2(&file_data),
    }
}

fn part1(data: &str) {
    let pattern = r#"mul\(\d{1,3},\d{1,3}\)"#;
    let re = Regex::new(pattern).unwrap();
    let result = re
        .find_iter(&data)
        .map(|hit| {
            hit.as_str()[4..hit.as_str().len() - 1]
                .split(',')
                .map(|num_str| num_str.parse::<isize>().unwrap())
                .product::<isize>()
        })
        .sum::<isize>();
    println!("{result}")
}
fn part2(data: &str) {
    let mul_pattern = r#"mul\(\d{1,3},\d{1,3}\)"#;
    let mul_re = Regex::new(mul_pattern).unwrap();
    let dont_pattern = r#"don't\(\)(.|\n)*?(do\(\)|\z)"#;
    let dont_re = Regex::new(dont_pattern).unwrap();

    let part_1_result: usize = mul_re
        .find_iter(&data)
        .map(|hit| {
            hit.as_str()[4..hit.as_str().len() - 1]
                .split(',')
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .product::<usize>()
        })
        .sum::<usize>();
    let part_2_donts = dont_re
        .find_iter(&data)
        .map(|hit| {
            mul_re
                .find_iter(hit.as_str())
                .map(|hit| {
                    hit.as_str()[4..hit.as_str().len() - 1]
                        .split(',')
                        .map(|num_str| num_str.parse::<usize>().unwrap())
                        .product::<usize>()
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    let result = part_1_result - part_2_donts;
    println!("{result}")
}
