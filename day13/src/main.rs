use std::fs::read_to_string;
fn main() {
    println!("{}", part1(&"input.txt"));
    println!("{}", part2(&"input.txt"));
}
#[derive(Debug, Clone, PartialEq)]
enum Location {
    Ash,
    Rock,
}

fn part1(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();

    let games = file
        .split(&"\n\r\n")
        .map(|game| {
            game.to_string()
                .lines()
                .map(|line| {
                    line.trim()
                        .chars()
                        .map(|c| match c {
                            '.' => Location::Ash,
                            '#' => Location::Rock,
                            _ => panic!("parsing issue"),
                        })
                        .collect::<Vec<Location>>()
                })
                .collect::<Vec<Vec<Location>>>()
        })
        .map(|game| solve_game(&game))
        .sum::<usize>();

    games
}

fn solve_game(game: &Vec<Vec<Location>>) -> usize {
    for (index, _row) in game.iter().enumerate().skip(1) {
        if game[index].eq(&game[index - 1]) {
            let mut is_mirror = true;
            let range_ind = *[game.len() - 1 - index, index - 1].iter().min().unwrap();

            for i in 1 as usize..range_ind + 1 {
                for j in 0..game[index].len() {
                    if game[index + i][j] != game[index - 1 - i][j] {
                        is_mirror = false;
                        break;
                    }
                }
            }

            if is_mirror {
                return index * 100;
            }
        }
    }

    for col_index in 1..game[0].len() {
        let mut is_matching = true;
        for (row_index, _row) in game.iter().enumerate() {
            if game[row_index][col_index - 1].ne(&game[row_index][col_index]) {
                is_matching = false;
                break;
            }
        }
        if is_matching {
            let mut is_mirror = true;
            let range_ind = *[game[0].len() - 1 - col_index, col_index - 1]
                .iter()
                .min()
                .unwrap();
            for i in 1 as usize..range_ind + 1 {
                for j in 0..game.len() {
                    if game[j][col_index + i] != game[j][col_index - 1 - i] {
                        is_mirror = false;
                        break;
                    }
                }
            }
            if is_mirror {
                return col_index;
            }
        }
    }
    return 0;
}

fn part2(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();

    let games = file
        .split(&"\n\r\n")
        .map(|game| {
            game.to_string()
                .lines()
                .map(|line| {
                    line.trim()
                        .chars()
                        .map(|c| match c {
                            '.' => Location::Ash,
                            '#' => Location::Rock,
                            _ => panic!("parsing issue"),
                        })
                        .collect::<Vec<Location>>()
                })
                .collect::<Vec<Vec<Location>>>()
        })
        .map(|game| solve_game_for_1_missing(&game))
        .sum::<usize>();

    games
}

fn solve_game_for_1_missing(game: &Vec<Vec<Location>>) -> usize {
    // need to modify to count misalignments in row if > 1 break, then if ne 1 then continue
    // other wise this is the candidate

    for (row_index, _row) in game.iter().enumerate().skip(1) {
        let mut row_misalignment_count: usize = 0;
        for (col_index, _col) in game[0].iter().enumerate() {
            if game[row_index - 1][col_index].ne(&game[row_index][col_index]) {
                row_misalignment_count += 1;
            }
        }

        if row_misalignment_count <= 1 {
            let range_ind = *[game.len() - 1 - row_index, row_index - 1]
                .iter()
                .min()
                .unwrap();
            for i in 1 as usize..range_ind + 1 {
                for j in 0..game[0].len() {
                    if game[row_index + i][j] != game[row_index - 1 - i][j] {
                        row_misalignment_count += 1;
                    }
                    if row_misalignment_count > 1 {
                        break;
                    }
                }
                if row_misalignment_count > 1 {
                    break;
                }
            }
            if row_misalignment_count.eq(&1) {
                return 100 * row_index;
            }
        }
    }

    for col_index in 1..game[0].len() {
        let mut col_misalignment_count: usize = 0;
        for (row_index, _row) in game.iter().enumerate() {
            if game[row_index][col_index - 1].ne(&game[row_index][col_index]) {
                col_misalignment_count += 1;
            }
        }
        if col_misalignment_count <= 1 {
            let range_ind = *[game[0].len() - 1 - col_index, col_index - 1]
                .iter()
                .min()
                .unwrap();
            for i in 1 as usize..range_ind + 1 {
                for j in 0..game.len() {
                    if game[j][col_index + i] != game[j][col_index - 1 - i] {
                        col_misalignment_count += 1;
                    }
                    if col_misalignment_count > 1 {
                        break;
                    }
                }
                if col_misalignment_count > 1 {
                    break;
                }
            }
            if col_misalignment_count.eq(&1) {
                return col_index;
            }
        }
    }
    return 0;
}
