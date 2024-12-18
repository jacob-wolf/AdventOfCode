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

fn part1(data: &str) {}

fn part2(data: &str) {}
