use std::{collections::HashMap, fs::read_to_string};
fn main() {
    println!("{}", part1(&"input.txt"));
    println!("{}", part2(&"input.txt"));
}

fn part1(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();

    let mut total: usize = 0;

    file.split(',')
        .for_each(|step| total += apply_sequence_hash(&step));

    total
}

fn apply_sequence_hash(step: &str) -> usize {
    let mut total: usize = 0;
    step.chars()
        .for_each(|c| total = apply_char_hash(&total, &c));

    total
}

fn apply_char_hash(curr_total: &usize, c: &char) -> usize {
    let mut total = curr_total.clone();
    total += { *c } as usize;
    total *= 17;
    total %= 256;
    total
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.label.eq(&other.label)
    }
    fn ne(&self, other: &Self) -> bool {
        self.label.ne(&other.label)
    }
}

fn part2(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();
    let mut total: usize = 0;

    let mut map: HashMap<usize, Vec<Lens>> = HashMap::new();

    file.split(',')
        .for_each(|operator| handle_operator(&operator, &mut map));

    let mut keys = map.keys().cloned().collect::<Vec<usize>>();
    keys.sort();

    keys.iter()
        .for_each(|key| total += handle_box_counting_logic(&map.get(&key).unwrap(), &key));

    total
}

fn handle_operator(command: &str, map: &mut HashMap<usize, Vec<Lens>>) {
    let operator_index: usize = command.find(|c: char| c.eq(&'-') || c.eq(&'=')).unwrap();

    let label: String = command[0..operator_index].to_string();
    let operator: String = command[operator_index..].to_string();

    // run hash on the label (before operator) to get the box number
    let box_number = apply_sequence_hash(&label.as_str());

    // operator - or =
    match operator.chars().nth(0).unwrap() {
        '-' => {
            // - -> go to the box and remove the lens with the indicated label then move other lens forward
            let curr_box_contents = map.get(&box_number);
            if let Some(contents) = curr_box_contents {
                let mut lens_index: Option<usize> = None;
                for (index, lens) in contents.iter().enumerate() {
                    if lens.label.eq(&label) {
                        lens_index = Some(index);
                        break;
                    }
                }
                if let Some(index) = lens_index {
                    let mut new_contents = contents.clone();
                    new_contents.remove(index);
                    map.insert(box_number, new_contents);
                }
            }
        }
        '=' => {
            // =\d -> go to the box replace teh old lens with the same label else add the lens to the end of the box
            let curr_box_contents = map.get(&box_number);
            let focal_length = operator
                .chars()
                .last()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();
            match curr_box_contents {
                Some(contents) => {
                    let new_lens = Lens {
                        label,
                        focal_length,
                    };
                    let mut lens_index: Option<usize> = None;
                    for (index, lens) in contents.iter().enumerate() {
                        if lens.eq(&new_lens) {
                            lens_index = Some(index);
                            break;
                        }
                    }

                    if let Some(existing_index) = lens_index {
                        let mut new_contents = contents.clone();
                        new_contents.remove(existing_index);
                        new_contents.insert(existing_index, new_lens);
                        map.insert(box_number, new_contents);
                    } else {
                        let mut new_contents = contents.clone();
                        new_contents.push(new_lens);
                        map.insert(box_number, new_contents);
                    }
                }
                None => {
                    map.insert(
                        box_number,
                        vec![Lens {
                            label: label,
                            focal_length,
                        }],
                    );
                }
            }
        }
        _ => {
            println!("{:?}", operator.chars().nth(0));
            panic!("Parse err");
        }
    }
}

fn handle_box_counting_logic(contents: &Vec<Lens>, box_num_0_indexed: &usize) -> usize {
    let mut contents_total: usize = 0;

    contents
        .iter()
        .enumerate()
        .for_each(|(inner_box_order_index, lens_item)| {
            contents_total +=
                { 1 + box_num_0_indexed } * { 1 + inner_box_order_index } * lens_item.focal_length
        });

    contents_total
}
