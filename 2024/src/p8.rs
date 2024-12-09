use advent_of_code_2024::{read_file, Part, Which};
use std::collections::{HashMap, HashSet};

pub fn p8(choice: Which, part: Part) {
    let file_data: String = read_file(8, choice, None);
    let now = std::time::SystemTime::now();
    match part {
        Part::One => part1(&file_data),
        Part::Two => part2(&file_data),
    }
    match now.elapsed() {
        Ok(elapsed) => println!("Runtime: {} microseconds", elapsed.as_micros()),
        _ => panic!(),
    }
}

fn validate_coord(r: &isize, c: &isize, r_max: &isize, c_max: &isize) -> bool {
    r >= &0 && r <= r_max && c >= &0 && c <= c_max
}

fn part1(data: &str) {
    let mut coord_hashmap: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let data_map = data
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, c)| match c {
                    '.' => None,
                    _ => {
                        if let None = coord_hashmap.get(&c) {
                            coord_hashmap.insert(c, vec![]);
                        }
                        let mut vec = coord_hashmap.get(&c).unwrap().clone();
                        vec.push((row_idx, col_idx));
                        coord_hashmap.insert(c, vec);
                        Some(c)
                    }
                })
                .collect::<Vec<Option<char>>>()
        })
        .collect::<Vec<Vec<Option<char>>>>();

    let row_max = data_map.len() as isize - 1;
    let col_max = data_map[0].len() as isize - 1;
    let mut node_set: HashSet<(isize, isize)> = HashSet::new();
    coord_hashmap.keys().for_each(|key| {
        let transmitter_location_vec = coord_hashmap.get(&key).unwrap();
        let num_coords = transmitter_location_vec.len();
        if num_coords < 2 {
            return;
        }
        let mut combos = vec![];
        transmitter_location_vec
            .iter()
            .enumerate()
            .for_each(|(idx, coord)| {
                for next_cooord_idx in idx + 1..num_coords {
                    combos.push((
                        coord.clone(),
                        transmitter_location_vec[next_cooord_idx].clone(),
                    ));
                }
            });
        for ((l_r, l_c), (r_r, r_c)) in combos {
            let d_r = r_r as isize - l_r as isize;
            let d_c = r_c as isize - l_c as isize;
            let (a1r, a1c) = (r_r as isize + d_r, r_c as isize + d_c);
            let (a2r, a2c) = (l_r as isize - d_r, l_c as isize - d_c);

            if validate_coord(&a1r, &a1c, &row_max, &col_max) {
                node_set.insert((a1r, a1c));
            }
            if validate_coord(&a2r, &a2c, &row_max, &col_max) {
                node_set.insert((a2r, a2c));
            }
        }
    });
    let count = node_set.len();
    println!("{count}");
}

fn part2(data: &str) {
    let mut coord_hashmap: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let data_map = data
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, c)| match c {
                    '.' => None,
                    _ => {
                        if let None = coord_hashmap.get(&c) {
                            coord_hashmap.insert(c, vec![]);
                        }
                        let mut vec = coord_hashmap.get(&c).unwrap().clone();
                        vec.push((row_idx, col_idx));
                        coord_hashmap.insert(c, vec);
                        Some(c)
                    }
                })
                .collect::<Vec<Option<char>>>()
        })
        .collect::<Vec<Vec<Option<char>>>>();

    let row_max = data_map.len() as isize - 1;
    let col_max = data_map[0].len() as isize - 1;
    let mut node_set: HashSet<(isize, isize)> = HashSet::new();
    coord_hashmap.keys().for_each(|key| {
        let transmitter_location_vec = coord_hashmap.get(&key).unwrap();
        let num_coords = transmitter_location_vec.len();
        if num_coords < 2 {
            return;
        }
        let mut combos = vec![];
        transmitter_location_vec
            .iter()
            .enumerate()
            .for_each(|(idx, coord)| {
                for next_cooord_idx in idx + 1..num_coords {
                    combos.push((
                        coord.clone(),
                        transmitter_location_vec[next_cooord_idx].clone(),
                    ));
                }
            });
        for ((l_r, l_c), (r_r, r_c)) in combos {
            let d_r = r_r as isize - l_r as isize;
            let d_c = r_c as isize - l_c as isize;
            let mut new_r = l_r as isize;
            let mut new_c = l_c as isize;
            while validate_coord(&new_r, &new_c, &row_max, &col_max) {
                node_set.insert((new_r, new_c));
                new_r = new_r - d_r;
                new_c = new_c - d_c;
            }
            new_r = r_r as isize;
            new_c = r_c as isize;
            while validate_coord(&new_r, &new_c, &row_max, &col_max) {
                node_set.insert((new_r, new_c));
                new_r = new_r + d_r;
                new_c = new_c + d_c;
            }
        }
    });
    println!("{}", node_set.len());
}
