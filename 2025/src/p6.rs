use advent_of_code_2025::{read_file, Part, Which};

pub fn p6(choice: Which, part: Part) {
    let file_data: String = read_file(6, choice, None);
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
    let line_count = data.lines().count();

    let mut problems = vec![];

    let operations = data
        .lines()
        .skip(line_count - 1)
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .map(|item| match item {
            "+" => false,
            "*" => true,
            _ => panic!("Not valid operation"),
        })
        .collect::<Vec<bool>>();
    let problem_count = operations.len();
    data.lines().take(line_count - 1).for_each(|line| {
        let mut line_i = vec![];
        line.split_ascii_whitespace().for_each(|num_str| {
            let num = num_str.parse::<usize>().unwrap();
            line_i.push(num);
        });

        problems.push(line_i);
    });
    let mut running_total = 0;
    for idx in 0..problem_count {
        let flag = operations[idx];

        let mut curr_problem_total = if flag { 1 } else { 0 };

        for jdx in 0..line_count - 1 {
            if flag {
                curr_problem_total *= problems[jdx][idx]
            } else {
                curr_problem_total += problems[jdx][idx]
            }
        }
        running_total += curr_problem_total;
    }

    println!("P1: {running_total}");
}
fn part2(data: &str) {
    let line_count = data.lines().count();
    let line_length = data.lines().nth(0).unwrap().len();
    let og_data = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let seed_vec: Vec<Option<char>> = vec![None; line_count];
    let mut data_transpose: Vec<Vec<Option<char>>> = vec![seed_vec; line_length];

    for i in 0..line_count {
        for j in 0..line_length {
            let item = if og_data[i][j].is_ascii_whitespace() {
                None
            } else {
                Some(og_data[i][j])
            };
            data_transpose[j][i] = item;
        }
    }
    let mut running_total = 0;
    let mut running_nums: Vec<usize> = vec![];
    let mut running_nums_operation = if data_transpose[0]
        .iter()
        .nth(data_transpose[0].len() - 1)
        .unwrap()
        .unwrap()
        == '*'
    {
        true
    } else {
        false
    };
    for (idx, new_line) in data_transpose.iter().enumerate() {
        let last_char = new_line.iter().nth(new_line.len() - 1).unwrap();
        if let Some(x) = last_char {
            if idx != 0 {
                let quantity_to_add = if running_nums_operation {
                    running_nums.iter().product::<usize>()
                } else {
                    running_nums.iter().sum::<usize>()
                };
                running_total += quantity_to_add;
            }
            running_nums_operation = if *x == '*' { true } else { false };
            running_nums = vec![];
        }

        let new_line_string = new_line
            .iter()
            .take(new_line.len() - 1)
            .map(|opt_chr| match opt_chr {
                Some(chr) => chr,
                _ => &' ',
            })
            .collect::<String>();
        let new_line_trim = new_line_string.trim();
        let new_num = if new_line_trim != String::from("") {
            new_line_trim.parse::<usize>().unwrap()
        } else {
            if running_nums_operation {
                1
            } else {
                0
            }
        };
        running_nums.push(new_num);
    }

    let quantity_to_add = if running_nums_operation {
        running_nums.iter().product::<usize>()
    } else {
        running_nums.iter().sum::<usize>()
    };
    running_total += quantity_to_add;

    println!("P2 {running_total:?}")
}
