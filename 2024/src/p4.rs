use advent_of_code_2024::{read_file, Part, Which};
pub fn p4(choice: Which, part: Part) {
    let file_data: String = read_file(4, choice, None);
    match part {
        Part::One => part1(&file_data),
        Part::Two => part2(&file_data),
    }
}
enum Size {
    One,
    Two,
    Three,
}
#[derive(PartialEq, Eq)]
enum Direction {
    Px,
    Py,
    Nx,
    Ny,
    Pxpy,
    Pxny,
    Nxpy,
    Nxny,
}
struct Coord(usize, usize);

fn check_character(
    coord: &Coord,
    direction: &Direction,
    size: Size,
    data: &Vec<Vec<char>>,
) -> bool {
    let (character, size) = match size {
        Size::One => ('M', 1 as usize),
        Size::Two => ('A', 2 as usize),
        Size::Three => ('S', 3 as usize),
    };
    // Py points down
    // Px points right
    match direction {
        Direction::Py => data[coord.0 + size][coord.1] == character, // changing rows is Py
        Direction::Ny => data[coord.0 - size][coord.1] == character,
        Direction::Px => data[coord.0][coord.1 + size] == character,
        Direction::Nx => data[coord.0][coord.1 - size] == character,
        Direction::Pxpy => data[coord.0 + size][coord.1 + size] == character,
        Direction::Nxpy => data[coord.0 + size][coord.1 - size] == character,
        Direction::Pxny => data[coord.0 - size][coord.1 + size] == character,
        Direction::Nxny => data[coord.0 - size][coord.1 - size] == character,
    }
}

fn check_x(coord: &Coord, data: &Vec<Vec<char>>, max_x: &usize, max_y: &usize) -> usize {
    let x = coord.1; // col_idx defines the x coordinate
    let y = coord.0; // row_idx defines the y coordinate
    let mut directions_to_check: Vec<Direction> = vec![];
    //eliminate edges (need 3 more slots to fit mas)
    if x <= max_x - 3 && y <= max_y - 3 {
        directions_to_check.push(Direction::Px);
        directions_to_check.push(Direction::Py);
        directions_to_check.push(Direction::Pxpy);
    } else if x <= max_x - 3 {
        directions_to_check.push(Direction::Px);
    } else if y <= max_y - 3 {
        directions_to_check.push(Direction::Py);
    }

    if x >= 3 && y >= 3 {
        directions_to_check.push(Direction::Nx);
        directions_to_check.push(Direction::Ny);
        directions_to_check.push(Direction::Nxny);
    } else if x >= 3 {
        directions_to_check.push(Direction::Nx);
    } else if y >= 3 {
        directions_to_check.push(Direction::Ny)
    }

    if x <= max_x - 3 && y >= 3 {
        directions_to_check.push(Direction::Pxny);
    }
    if x >= 3 && y <= max_y - 3 {
        directions_to_check.push(Direction::Nxpy);
    }

    let mut curr_x_count: usize = 0;
    directions_to_check.iter().for_each(|direction| {
        //check m a s
        if check_character(&coord, &direction, Size::One, &data)
            && check_character(&coord, &direction, Size::Two, &data)
            && check_character(&coord, &direction, Size::Three, &data)
        {
            curr_x_count += 1;
        }
    });
    curr_x_count
}

fn part1(data: &str) {
    // find XMAS
    // find every x in the file
    // find m in 8 neighbors
    // find a in same direction
    // find s in same direction

    let mut x_coords: Vec<Coord> = vec![];

    let data_vec: Vec<Vec<char>> = data
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars().enumerate().for_each(|(col_idx, character)| {
                if character.eq(&'X') {
                    x_coords.push(Coord(row_idx, col_idx));
                }
            });
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut result: usize = 0;
    let x_dim = data_vec[0].len(); // number of cols defines the x size
    let y_dim = data_vec.len(); // number of rows defines the y size
    x_coords
        .iter()
        .for_each(|coord| result += check_x(&coord, &data_vec, &{ x_dim - 1 }, &{ y_dim - 1 })); //subtract one for true maximum

    println!("{result}");
}

fn check_a(coord: &Coord, data: &Vec<Vec<char>>, max_x: &usize, max_y: &usize) -> usize {
    let x = coord.1; // col_idx defines the x coordinate
    let y = coord.0; // row_idx defines the y coordinate

    // eliminate edges
    if x == *max_x || x == 0 || y == *max_y || y == 0 {
        return 0;
    }

    let lower_right = data[y + 1][x + 1];
    let upper_left = data[y - 1][x - 1];
    let lower_left = data[y + 1][x - 1];
    let upper_right = data[y - 1][x + 1];

    // check the four corners, should see two M and two S
    let mut m_count: usize = 0;
    let mut s_count: usize = 0;
    for char in [lower_left, lower_right, upper_left, upper_right] {
        match char {
            'M' => m_count += 1,
            'S' => s_count += 1,
            _ => continue,
        }
    }

    if s_count != 2 || m_count != 2 {
        return 0;
    }
    // now counts are 2 S and 2 M in the corners
    // 4 choose 2 = 6 possibilities (4 positions pick where 2Ms go)
    // first four valid
    //MM MS SS SM     MS SM
    //SS MS MM SM     SM MS

    // rule out the MAM SAS combos
    if lower_left == upper_right {
        return 0;
    }

    return 1;
}

fn part2(data: &str) {
    // Find all the A's, check if they create MAS MAS
    let mut a_coords: Vec<Coord> = vec![];
    let data_vec: Vec<Vec<char>> = data
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars().enumerate().for_each(|(col_idx, character)| {
                if character.eq(&'A') {
                    a_coords.push(Coord(row_idx, col_idx));
                }
            });
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut result: usize = 0;
    // subtract one due to zero indexing (x_max is largest allowed index)
    let x_max = data_vec[0].len() - 1; // number of cols defines the x size
    let y_max = data_vec.len() - 1; // number of rows defines the y size
    a_coords
        .iter()
        .for_each(|coord| result += check_a(&coord, &data_vec, &x_max, &y_max));
    println!("{result}");
}
