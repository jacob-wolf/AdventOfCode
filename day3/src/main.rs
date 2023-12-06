use std::fs::read_to_string;

fn main() {
    println!("part1: {}", part_1(&"input.txt"));
    println!("part2: {}", part_2(&"input.txt"));
}

fn part_1(filepath: &str) -> usize {
    let file: String = read_to_string(&filepath).unwrap();
    let char_matrix: Vec<Vec<char>> = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let dimensions: (usize, usize) = (char_matrix.len(), char_matrix[0].len());

    let mut accounted_numbers: Vec<Vec<bool>> = char_matrix
        .iter()
        .map(|_| vec![false; dimensions.1])
        .collect::<Vec<Vec<bool>>>();

    for (row_num, line) in char_matrix.iter().enumerate() {
        for (col_num, char) in line.iter().enumerate() {
            if !char.is_numeric() && char.ne(&'.') {
                check_symbol_neighbors(
                    &row_num,
                    &col_num,
                    &char_matrix,
                    &mut accounted_numbers,
                    &dimensions,
                );
            }
        }
    }

    parse_nums(&char_matrix, &accounted_numbers, &dimensions)
}

fn check_symbol_neighbors(
    r: &usize,
    c: &usize,
    chars: &Vec<Vec<char>>,
    nums: &mut Vec<Vec<bool>>,
    dims: &(usize, usize),
) {
    for i in r - 1..r + 2 {
        if i >= dims.0 {
            continue;
        }
        for j in c - 1..c + 2 {
            if j >= dims.1 {
                continue;
            }
            if chars[i][j].is_numeric() {
                nums[i][j] = true;
                identify_complete_number(&i, &j, &chars, nums, &dims);
            }
        }
    }
}

fn identify_complete_number(
    r: &usize,
    c: &usize,
    chars: &Vec<Vec<char>>,
    nums: &mut Vec<Vec<bool>>,
    dims: &(usize, usize),
) {
    let mut j = c.clone() + 1;
    while j < dims.1 && chars[*r][j].is_numeric() {
        nums[*r][j] = true;
        j += 1;
    }

    j = c.clone() - 1;
    while j < dims.1 && chars[*r][j].is_numeric() {
        nums[*r][j] = true;
        if j == 0 {
            break;
        }
        j -= 1;
    }
}

fn parse_nums(chars: &Vec<Vec<char>>, nums: &Vec<Vec<bool>>, dim: &(usize, usize)) -> usize {
    let mut total: usize = 0;
    for r in 0..dim.0 {
        let mut switch = false;
        let mut curr_str = String::from("");
        for c in 0..dim.1 {
            if nums[r][c] == false {
                if switch {
                    total += curr_str.parse::<usize>().unwrap();
                    switch = false;
                    curr_str = String::from("");
                }
                continue;
            }
            curr_str += chars[r][c].to_string().as_str();
            switch = true;
        }
        if switch {
            total += curr_str.parse::<usize>().unwrap();
        }
    }
    total
}

fn part_2(filepath: &str) -> usize {
    let file: String = read_to_string(&filepath).unwrap();
    let char_matrix: Vec<Vec<char>> = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let dimensions: (usize, usize) = (char_matrix.len(), char_matrix[0].len());
    let mut running_total = 0;
    for (row_num, line) in char_matrix.iter().enumerate() {
        for (col_num, char) in line.iter().enumerate() {
            if !char.is_numeric() && char.ne(&'.') {
                running_total +=
                    check_symbol_neighbors_2(&row_num, &col_num, &char_matrix, &dimensions);
            }
        }
    }
    running_total
}

fn check_symbol_neighbors_2(
    r: &usize,
    c: &usize,
    chars: &Vec<Vec<char>>,
    dims: &(usize, usize),
) -> usize {
    let mut found_nums: Vec<usize> = vec![];
    for i in r - 1..r + 2 {
        if i >= dims.0 {
            continue;
        }
        let mut is_curr_num = false;
        for j in c - 1..c + 2 {
            if j >= dims.1 {
                continue;
            }
            //if digit found, set switch to true
            if chars[i][j].is_numeric() {
                is_curr_num = true;
                continue;
            }
            //if not digit, but switch is true parse the num and add it to the list
            if is_curr_num {
                let num: usize = finish_num_validation(&chars, &dims, &i, &{ j - 1 });
                found_nums.push(num);
                is_curr_num = false;
            }
        }
        if is_curr_num {
            let num: usize = finish_num_validation(&chars, &dims, &i, &{ c + 1 });
            found_nums.push(num);
        }
    }

    if found_nums.len() != 2 {
        return 0;
    };

    found_nums[0] * found_nums[1]
}

fn finish_num_validation(
    chars: &Vec<Vec<char>>,
    dims: &(usize, usize),
    r: &usize,
    c: &usize,
) -> usize {
    let mut j = c.clone();
    while j < dims.1 && chars[*r][j].is_numeric() {
        j += 1;
    }
    let max_j_exc = j;

    j = c.clone();
    while j < dims.1 && chars[*r][j].is_numeric() {
        if j == 0 {
            break;
        }
        j -= 1;
    }
    let min_j_inc = if j == 0 && chars[*r][j].is_numeric() {
        0
    } else {
        j + 1
    };

    let mut num_string = String::from("");
    for col_index in min_j_inc..max_j_exc {
        num_string += chars[*r][col_index].to_string().as_str();
    }

    num_string.parse::<usize>().unwrap()
}
