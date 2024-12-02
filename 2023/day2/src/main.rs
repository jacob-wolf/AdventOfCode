use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};
fn main() {
    println!("part 1: {}", sum_feasible_games(&"input.txt"));
    println!("part 2: {}", sum_minimum_multiples(&"input.txt"));
}

fn sum_feasible_games(input: &str) -> usize {
    let file = read_to_string(input).unwrap();

    let rules: [(&char, usize); 3] = [(&'r', 12), (&'g', 13), (&'b', 14)];
    let rules_map: HashMap<&char, usize> = rules.iter().cloned().collect::<HashMap<&char, usize>>();

    let mut running_total: usize = 0;
    file.lines().enumerate().for_each(|(index, line)| {
        running_total += get_game_feasibility(line, &{ index + 1 }, &rules_map);
    });
    running_total
}

/// Returns 0 if infeasible, returns game number if feasible
fn get_game_feasibility(line: &str, game_num: &usize, rules: &HashMap<&char, usize>) -> usize {
    let re = Regex::new(r"([0-9]+ [rgb])").unwrap();

    let mut results: Vec<&str> = vec![];
    for (_, [a]) in re.captures_iter(&line).map(|cap| cap.extract()) {
        results.push(a);
    }

    let invalidity = results
        .iter()
        .map(|res| {
            let allowed_count = rules.get(&res.chars().last().unwrap()).unwrap();
            let found_count = res
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            if found_count > *allowed_count {
                false
            } else {
                true
            }
        })
        .any(|truth| !truth);

    if !invalidity {
        game_num.clone()
    } else {
        0
    }
}

fn sum_minimum_multiples(input: &str) -> usize {
    let file = read_to_string(input).unwrap();
    let mut running_total: usize = 0;

    file.lines()
        .for_each(|line| running_total += calculate_minimum_multiplicity(&line));

    running_total
}

fn calculate_minimum_multiplicity(line: &str) -> usize {
    let re = Regex::new(r"([0-9]+ [rgb])").unwrap();

    let mut results: Vec<&str> = vec![];
    for (_, [a]) in re.captures_iter(&line).map(|cap| cap.extract()) {
        results.push(a);
    }

    let mut min_r: usize = 0;
    let mut min_g: usize = 0;
    let mut min_b: usize = 0;

    for res in results {
        let found_value = res
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let color = res.chars().last().unwrap().to_string();
        if color.eq_ignore_ascii_case(&"r") {
            min_r = if min_r < found_value {
                found_value
            } else {
                min_r
            };
        } else if color.eq_ignore_ascii_case(&"g") {
            min_g = if min_g < found_value {
                found_value
            } else {
                min_g
            };
        } else if color.eq_ignore_ascii_case(&"b") {
            min_b = if min_b < found_value {
                found_value
            } else {
                min_b
            };
        }
    }
    min_r * min_g * min_b
}
