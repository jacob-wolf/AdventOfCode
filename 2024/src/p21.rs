use advent_of_code_2024::{read_file, Part, Which};
use std::collections::HashMap;
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
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
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

fn get_numpad_directions(
    item: &NumPad,
    start_position: &(usize, usize),
    computed: &mut HashMap<((usize, usize), (usize, usize)), Vec<KeyPad>>,
) -> Vec<KeyPad> {
    const INVALID_POSITION: (usize, usize) = (3, 0);

    let target_position = get_num_pad_coord(&item);

    if let Some(val) = computed.get(&(*start_position, target_position)) {
        return val.clone();
    }

    let mut curr_position = start_position.clone();
    let mut key_pad_required = vec![];
    // loop to press arrow keys until target num is hovered
    while curr_position.ne(&target_position) {
        if curr_position.0 < target_position.0
            && INVALID_POSITION.ne(&(curr_position.0 + 1, curr_position.1))
        {
            curr_position = (curr_position.0 + 1, curr_position.1);
            key_pad_required.push(KeyPad::Down);
        } else if curr_position.0 > target_position.0 {
            curr_position = (curr_position.0 - 1, curr_position.1);
            key_pad_required.push(KeyPad::Up);
        } else if curr_position.1 > target_position.1
            && INVALID_POSITION.ne(&(curr_position.0, curr_position.1 - 1))
        {
            curr_position = (curr_position.0, curr_position.1 - 1);
            key_pad_required.push(KeyPad::Left);
        } else {
            curr_position = (curr_position.0, curr_position.1 + 1);
            key_pad_required.push(KeyPad::Right);
        }
    }
    // finally press A to enter the number
    key_pad_required.push(KeyPad::A);

    computed.insert((*start_position, curr_position), key_pad_required.clone());

    key_pad_required
}

fn get_keypad_directions(
    desired: &Vec<KeyPad>,
    computed: &mut HashMap<(KeyPad, KeyPad), Vec<KeyPad>>,
) -> Vec<KeyPad> {
    // want to minimize number of steps
    // if i last pressed left prio left aka nearby keys
    //const INVALID_POSITION: (usize, usize) = (0, 0);
    let mut higher_level_directions = vec![];
    let mut last_input_num = KeyPad::A;
    for desire in desired {
        if let Some(instructions) = computed.get(&(KeyPad::A, *desire)) {
            for item in instructions {
                higher_level_directions.push(item.clone());
            }
            last_input_num = desire.clone();
            continue;
        }

        let curr_position = get_key_pad_coord(&last_input_num); // always going to start from A
        let target_position = get_key_pad_coord(desire);

        let mut computed_instructions = vec![];

        let (dy, dx) = (
            target_position.0 as isize - curr_position.0 as isize,
            target_position.1 as isize - curr_position.1 as isize,
        );
        // prio > ^ v <
        if dx > 0 {
            for _ in 0..dx {
                computed_instructions.push(KeyPad::Right);
            }
        }

        if dy < 0 {
            for _ in 0..dy.abs() {
                computed_instructions.push(KeyPad::Up);
            }
        }

        if dy > 0 {
            for _ in 0..dy.abs() {
                computed_instructions.push(KeyPad::Down);
            }
        }

        if dx < 0 {
            for _ in 0..dx.abs() {
                computed_instructions.push(KeyPad::Left);
            }
        }

        computed_instructions.push(KeyPad::A);
        computed.insert((KeyPad::A, *desire), computed_instructions.clone());
        last_input_num = desire.clone();
    }

    higher_level_directions
}

fn part1(data: &str) {
    let mut computed_num_pads = HashMap::new();
    let mut computed_key_pads = HashMap::new();
    let mut ans: usize = 0;
    data.lines().for_each(|line| {
        let target_sequence = line
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

        let mut numpad_bot = get_num_pad_coord(&NumPad::A);
        let mut my_press_count: usize = 0;

        for target in target_sequence {
            // target is what first bot must press
            // second bot must press first bot directions
            let first_bot_directions =
                get_numpad_directions(&target, &numpad_bot, &mut computed_num_pads);
            // third bot must press second bot directions
            let second_bot_directions =
                get_keypad_directions(&first_bot_directions, &mut computed_key_pads);
            // i press third bot directions
            let what_i_press =
                get_keypad_directions(&second_bot_directions, &mut computed_key_pads);
            my_press_count += what_i_press.len();
            first_bot_directions.iter().for_each(|pre| {
                let v = match pre {
                    KeyPad::Left => '<',
                    KeyPad::Right => '>',
                    KeyPad::Up => '^',
                    KeyPad::Down => 'v',
                    KeyPad::A => 'A',
                };
                print!("{v}");
            });
            println!("");
            numpad_bot = get_num_pad_coord(&target);
        }

        let target_numeric = line.trim()[0..3].parse::<usize>().unwrap();
        println!("For {line}, {my_press_count} * {target_numeric}");

        ans += target_numeric * my_press_count;

        // what do I hit to get each number
    });
    println!("{ans}");
}
fn part2(data: &str) {}
