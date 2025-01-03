use advent_of_code_2024::{read_file, Part, Which};
use std::{collections::HashMap, hash::Hash};

pub fn p21(choice: Which, part: Part) {
    let file_data: String = read_file(21, choice, None);
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

enum NumPad {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    A,
}
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
enum KeyPad {
    Left,
    Right,
    Up,
    Down,
    A,
}

fn get_num_pad_coord(target: &NumPad) -> (usize, usize) {
    match target {
        NumPad::One => (2, 0),
        NumPad::Two => (2, 1),
        NumPad::Three => (2, 2),
        NumPad::Four => (1, 0),
        NumPad::Five => (1, 1),
        NumPad::Six => (1, 2),
        NumPad::Seven => (0, 0),
        NumPad::Eight => (0, 1),
        NumPad::Nine => (0, 2),
        NumPad::Zero => (3, 1),
        NumPad::A => (3, 2),
    }
}

fn get_key_pad_coord(target: &KeyPad) -> (usize, usize) {
    match target {
        KeyPad::Left => (1, 0),
        KeyPad::Right => (1, 2),
        KeyPad::Up => (0, 1),
        KeyPad::Down => (1, 1),
        KeyPad::A => (0, 2),
    }
}

fn numpad_instructions(targets: &Vec<NumPad>) -> Vec<KeyPad> {
    let mut curr_position = get_num_pad_coord(&NumPad::A);
    let mut dir_pad = vec![];
    targets.iter().for_each(|target| {
        // priority check
        let target_position = get_num_pad_coord(&target);
        let mut dr = if target_position.0 > curr_position.0 {
            (0..target_position.0 - curr_position.0)
                .map(|_| KeyPad::Down)
                .collect::<Vec<KeyPad>>()
        } else {
            (0..curr_position.0 - target_position.0)
                .map(|_| KeyPad::Up)
                .collect::<Vec<KeyPad>>()
        };
        let mut dc = if target_position.1 > curr_position.1 {
            (0..target_position.1 - curr_position.1)
                .map(|_| KeyPad::Right)
                .collect::<Vec<KeyPad>>()
        } else {
            (0..curr_position.1 - target_position.1)
                .map(|_| KeyPad::Left)
                .collect::<Vec<KeyPad>>()
        };
        if target_position.1 > curr_position.1 && (target_position.0, curr_position.1) != (3, 0) {
            // going to move right and can move vertically first
            dir_pad.append(&mut dr);
            dir_pad.append(&mut dc);
        } else if (curr_position.0, target_position.1) != (3, 0) {
            dir_pad.append(&mut dc);
            dir_pad.append(&mut dr);
        } else {
            dir_pad.append(&mut dr);
            dir_pad.append(&mut dc);
        }
        dir_pad.push(KeyPad::A);
        curr_position = target_position;
    });
    dir_pad
}

fn keypad_instructions(
    targets: &Vec<KeyPad>,
    cache: &mut HashMap<((usize, usize), (usize, usize)), Vec<KeyPad>>,
) -> Vec<KeyPad> {
    let mut curr_position = get_key_pad_coord(&KeyPad::A);
    let mut dir_pad = vec![];
    targets.iter().for_each(|target| {
        let target_position = get_key_pad_coord(&target);
        if let Some(cached) = cache.get(&(curr_position, target_position)) {
            dir_pad.append(&mut cached.clone());
            curr_position = target_position;
            //println!("Cache Hit! {curr_position:?} -> {target_position:?} = {cached:?}");
            return;
        }

        let mut dr = if target_position.0 > curr_position.0 {
            (0..target_position.0 - curr_position.0)
                .map(|_| KeyPad::Down)
                .collect::<Vec<KeyPad>>()
        } else {
            (0..curr_position.0 - target_position.0)
                .map(|_| KeyPad::Up)
                .collect::<Vec<KeyPad>>()
        };
        let mut dc = if target_position.1 > curr_position.1 {
            (0..target_position.1 - curr_position.1)
                .map(|_| KeyPad::Right)
                .collect::<Vec<KeyPad>>()
        } else {
            (0..curr_position.1 - target_position.1)
                .map(|_| KeyPad::Left)
                .collect::<Vec<KeyPad>>()
        };
        let mut to_cache = vec![];
        if target_position.1 > curr_position.1 && (target_position.0, curr_position.1) != (0, 0) {
            // going to move right and can move vertically first
            dir_pad.append(&mut dr.clone());
            to_cache.append(&mut dr);

            dir_pad.append(&mut dc.clone());
            to_cache.append(&mut dc);
        } else if (curr_position.0, target_position.1) != (0, 0) {
            // can move horizontally first
            dir_pad.append(&mut dc.clone());
            to_cache.append(&mut dc);

            dir_pad.append(&mut dr.clone());
            to_cache.append(&mut dr);
        } else {
            // moving left and can move vertically first
            dir_pad.append(&mut dr.clone());
            to_cache.append(&mut dr);

            dir_pad.append(&mut dc.clone());
            to_cache.append(&mut dc);
        }
        dir_pad.push(KeyPad::A);
        to_cache.push(KeyPad::A);
        cache.insert((curr_position, target_position), to_cache);
        curr_position = target_position;
    });

    dir_pad
}

fn part1(data: &str) {
    let mut ans: usize = 0;
    data.lines().for_each(|line| {
        let num_bot = line
            .trim()
            .chars()
            .map(|chr| match chr {
                '1' => NumPad::One,
                '2' => NumPad::Two,
                '3' => NumPad::Three,
                '4' => NumPad::Four,
                '5' => NumPad::Five,
                '6' => NumPad::Six,
                '7' => NumPad::Seven,
                '8' => NumPad::Eight,
                '9' => NumPad::Nine,
                '0' => NumPad::Zero,
                'A' => NumPad::A,
                _ => panic!(),
            })
            .collect::<Vec<NumPad>>();

        let keypad_bot_one = numpad_instructions(&num_bot);
        let mut cache: HashMap<((usize, usize), (usize, usize)), Vec<KeyPad>> = HashMap::new();
        let keypad_bot_two = keypad_instructions(&keypad_bot_one, &mut cache);
        let keypad_bot_three = keypad_instructions(&keypad_bot_two, &mut cache);
        let my_press_count: usize = keypad_bot_three.len();

        let target_numeric = line.trim()[0..3].parse::<usize>().unwrap();
        println!("For {line}, {my_press_count} * {target_numeric}");

        ans += target_numeric * my_press_count;
    });
    println!("{ans}");
}

fn part2_get_keypad(start: &KeyPad, end: &KeyPad) -> Vec<KeyPad> {
    let mut res = vec![];
    let curr_position = get_key_pad_coord(&start);
    let target_position = get_key_pad_coord(&end);
    let dr = if target_position.0 > curr_position.0 {
        (0..target_position.0 - curr_position.0)
            .map(|_| KeyPad::Down)
            .collect::<Vec<KeyPad>>()
    } else {
        (0..curr_position.0 - target_position.0)
            .map(|_| KeyPad::Up)
            .collect::<Vec<KeyPad>>()
    };
    let dc = if target_position.1 > curr_position.1 {
        (0..target_position.1 - curr_position.1)
            .map(|_| KeyPad::Right)
            .collect::<Vec<KeyPad>>()
    } else {
        (0..curr_position.1 - target_position.1)
            .map(|_| KeyPad::Left)
            .collect::<Vec<KeyPad>>()
    };
    if target_position.1 > curr_position.1 && (target_position.0, curr_position.1) != (0, 0) {
        // going to move right and can move vertically first
        res.append(&mut dr.clone());
        res.append(&mut dc.clone());
    } else if (curr_position.0, target_position.1) != (0, 0) {
        // can move horizontally first
        res.append(&mut dc.clone());
        res.append(&mut dr.clone());
    } else {
        // moving left and can move vertically first
        res.append(&mut dr.clone());
        res.append(&mut dc.clone());
    }
    res.push(KeyPad::A);
    res
}

fn part2(data: &str) {
    let mut ans: usize = 0;
    data.lines().for_each(|line| {
        let num_bot = line
            .trim()
            .chars()
            .map(|chr| match chr {
                '1' => NumPad::One,
                '2' => NumPad::Two,
                '3' => NumPad::Three,
                '4' => NumPad::Four,
                '5' => NumPad::Five,
                '6' => NumPad::Six,
                '7' => NumPad::Seven,
                '8' => NumPad::Eight,
                '9' => NumPad::Nine,
                '0' => NumPad::Zero,
                'A' => NumPad::A,
                _ => panic!(),
            })
            .collect::<Vec<NumPad>>();

        // I don't actually care what the sequence is...
        // store counts for each sequence char to char
        // sequence always gets expanded by the char to char step
        let first_keypad = numpad_instructions(&num_bot);

        let mut modifiable_counts: HashMap<(KeyPad, KeyPad), usize> = HashMap::new();
        first_keypad.windows(2).for_each(|pair| {
            let count = modifiable_counts.get(&(pair[0], pair[1])).unwrap_or(&0);
            modifiable_counts.insert((pair[0], pair[1]), count + 1);
        });
        // don't skip the first item!
        let start_count = modifiable_counts
            .get(&(KeyPad::A, first_keypad[0]))
            .unwrap_or(&0);
        modifiable_counts.insert((KeyPad::A, first_keypad[0]), start_count + 1);

        let mut cache: HashMap<(KeyPad, KeyPad), Vec<KeyPad>> = HashMap::new();

        for _ in 0..25 {
            let mut next_modifiable_counts: HashMap<(KeyPad, KeyPad), usize> = HashMap::new();
            modifiable_counts.iter().for_each(|((start, end), count)| {
                let next_iter_vec = match cache.get(&(*start, *end)) {
                    Some(val) => val.clone(),
                    None => {
                        let x = part2_get_keypad(&start, &end);
                        cache.insert((*start, *end), x.clone());
                        x
                    }
                };
                next_iter_vec.windows(2).for_each(|pair_arr| {
                    let prev_count = next_modifiable_counts
                        .get(&(pair_arr[0], pair_arr[1]))
                        .unwrap_or(&0);
                    next_modifiable_counts.insert((pair_arr[0], pair_arr[1]), prev_count + count);
                });
                // don't skip the first item!
                let prev_count_start = next_modifiable_counts
                    .get(&(KeyPad::A, next_iter_vec[0]))
                    .unwrap_or(&0);
                next_modifiable_counts
                    .insert((KeyPad::A, next_iter_vec[0]), prev_count_start + count);
            });
            modifiable_counts = next_modifiable_counts;
        }
        let my_press_count = modifiable_counts.values().sum::<usize>();

        let target_numeric = line.trim()[0..3].parse::<usize>().unwrap();
        println!("For {line}, {my_press_count} * {target_numeric}");

        ans += target_numeric * my_press_count;
    });
    println!("{ans}");
}
