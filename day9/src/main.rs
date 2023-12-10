use std::fs::read_to_string;

fn main() {
    println!("{:?}", part1("input.txt"));
    println!("{:?}", part2("input.txt"));
}

fn part1(path: &str) -> isize {
    let file = read_to_string(&path).unwrap();
    let values_data = file
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.trim().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    let mut total: isize = 0;

    values_data
        .iter()
        .for_each(|values_list| total += calculate_next_value(values_list));

    total
}

fn calculate_next_value(values: &Vec<isize>) -> isize {
    let mut continuous_values = values.clone();
    let mut last_values: Vec<isize> = vec![values.iter().last().unwrap().clone()];

    while continuous_values.iter().any(|value| value.ne(&0)) {
        let new_continuous_values = continuous_values
            .iter()
            .enumerate()
            .skip(1)
            .map(|(index, _)| continuous_values[index] - continuous_values[index - 1])
            .collect::<Vec<isize>>();
        last_values.push(new_continuous_values.iter().last().unwrap().clone());
        continuous_values = new_continuous_values;
    }

    let mut final_value: isize = 0;

    for value in last_values.iter().rev() {
        final_value += value;
    }

    final_value
}

fn part2(path: &str) -> isize {
    let file = read_to_string(&path).unwrap();
    let values_data = file
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.trim().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    let mut total: isize = 0;

    values_data
        .iter()
        .for_each(|values_list| total += calculate_prev_value(values_list));

    total
}

fn calculate_prev_value(values: &Vec<isize>) -> isize {
    let mut continuous_values = values.clone();
    //now the first value
    let mut first_values: Vec<isize> = vec![values[0].clone()];

    while continuous_values.iter().any(|value| value.ne(&0)) {
        let new_continuous_values = continuous_values
            .iter()
            .enumerate()
            .skip(1)
            .map(|(index, _)| continuous_values[index] - continuous_values[index - 1])
            .collect::<Vec<isize>>();
        first_values.push(new_continuous_values[0]);
        continuous_values = new_continuous_values;
    }

    let mut final_value: isize = 0;

    for value in first_values.iter().rev() {
        final_value = value - final_value;
    }

    final_value
}
