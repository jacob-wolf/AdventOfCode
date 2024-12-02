use std::collections::HashSet;
use std::fs::read_to_string;
fn main() {
    println!("{}", part1(&"input.txt"));
    println!("{}", part2(&"input.txt"));
}

fn part1(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();

    let original_lines = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut empty_row_indices: HashSet<usize> = HashSet::new();
    let mut empty_col_indices: HashSet<usize> = HashSet::from_iter(
        original_lines[0]
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, _)| index),
    );
    let mut star_initial_locations: Vec<(usize, usize)> = vec![];

    for (row_index, row) in original_lines.iter().enumerate() {
        if !row.contains(&'#') {
            empty_row_indices.insert(row_index);
        }
        for (col_index, item) in row.iter().enumerate() {
            if item.eq(&'#') {
                empty_col_indices.remove(&col_index);
                star_initial_locations.push((row_index, col_index));
            }
        }
    }

    let num_stars = star_initial_locations.len();
    let mut distance: usize = 0;
    for i in 0..num_stars - 1 {
        for j in i..num_stars {
            let s1 = star_initial_locations[i];
            let s2 = star_initial_locations[j];

            let min_col = [s1.1, s2.1].iter().cloned().min().unwrap();
            let max_col = [s1.1, s2.1].iter().cloned().max().unwrap();
            let min_row = [s1.0, s2.0].iter().cloned().min().unwrap();
            let max_row = [s1.0, s2.0].iter().cloned().max().unwrap();

            let num_cols_to_add = empty_col_indices
                .iter()
                .cloned()
                .filter(|col_index| col_index > &min_col && col_index < &max_col)
                .collect::<Vec<usize>>()
                .len();
            let num_rows_to_add = empty_row_indices
                .iter()
                .cloned()
                .filter(|row_index| row_index > &min_row && row_index < &max_row)
                .collect::<Vec<usize>>()
                .len();

            distance += max_col - min_col + max_row - min_row + num_cols_to_add + num_rows_to_add
        }
    }

    distance
}

fn part2(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();

    let original_lines = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut empty_row_indices: HashSet<usize> = HashSet::new();
    let mut empty_col_indices: HashSet<usize> = HashSet::from_iter(
        original_lines[0]
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, _)| index),
    );
    let mut star_initial_locations: Vec<(usize, usize)> = vec![];

    for (row_index, row) in original_lines.iter().enumerate() {
        if !row.contains(&'#') {
            empty_row_indices.insert(row_index);
        }
        for (col_index, item) in row.iter().enumerate() {
            if item.eq(&'#') {
                empty_col_indices.remove(&col_index);
                star_initial_locations.push((row_index, col_index));
            }
        }
    }

    let num_stars = star_initial_locations.len();
    let mut distance: usize = 0;
    for i in 0..num_stars - 1 {
        for j in i..num_stars {
            let s1 = star_initial_locations[i];
            let s2 = star_initial_locations[j];

            let min_col = [s1.1, s2.1].iter().cloned().min().unwrap();
            let max_col = [s1.1, s2.1].iter().cloned().max().unwrap();
            let min_row = [s1.0, s2.0].iter().cloned().min().unwrap();
            let max_row = [s1.0, s2.0].iter().cloned().max().unwrap();

            //twice as big add 1
            //1,000,000 as big add 999_999
            let num_cols_to_add = 999_999
                * empty_col_indices
                    .iter()
                    .cloned()
                    .filter(|col_index| col_index > &min_col && col_index < &max_col)
                    .collect::<Vec<usize>>()
                    .len();
            let num_rows_to_add = 999_999
                * empty_row_indices
                    .iter()
                    .cloned()
                    .filter(|row_index| row_index > &min_row && row_index < &max_row)
                    .collect::<Vec<usize>>()
                    .len();

            distance += max_col - min_col + max_row - min_row + num_cols_to_add + num_rows_to_add
        }
    }

    distance
}
