use advent_of_code_2024::{read_file, Part, Which};
pub fn p5(choice: Which, part: Part) {
    let file_data: String = read_file(5, choice, None);
    match part {
        Part::One => part1(&file_data),
        Part::Two => part2(&file_data),
    }
}

fn part1(data: &str) {}
fn part2(data: &str) {}
