use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};
fn main() {
    println!("{:?}", part1(&"input.txt"));
    println!("{:?}", part2(&"input.txt"));
}

fn part1(filepath: &str) -> u32 {
    let file = read_to_string(&filepath).unwrap();

    file.lines()
        .map(|line| line.split(':').last().unwrap())
        .map(|mod_line| {
            mod_line
                .split('|')
                .map(|nums_string| nums_string.trim())
                .map(|trimmed_nums| {
                    Vec::from_iter(
                        trimmed_nums
                            .split_ascii_whitespace()
                            .map(|num| num.parse::<usize>().unwrap()),
                    )
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .map(|num_pair| {
            let winning_set: HashSet<&usize> = HashSet::from_iter(num_pair[0].iter());
            let mut winner_count: u32 = 0;
            for num in num_pair[1].clone() {
                if winning_set.contains(&num) {
                    winner_count += 1;
                }
            }
            if winner_count == 0 {
                return 0;
            }
            return 2_u32.pow(winner_count - 1);
        })
        .sum()
}

fn part2(filepath: &str) -> u32 {
    let file = read_to_string(&filepath).unwrap();
    let num_cards = file.lines().count();
    let mut card_copy_map: HashMap<usize, u32> = HashMap::new();

    file.lines()
        .map(|line| line.split(':').last().unwrap())
        .map(|mod_line| {
            mod_line
                .split('|')
                .map(|nums_string| nums_string.trim())
                .map(|trimmed_nums| {
                    Vec::from_iter(
                        trimmed_nums
                            .split_ascii_whitespace()
                            .map(|num| num.parse::<usize>().unwrap()),
                    )
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .enumerate()
        .for_each(|(card_num, num_pair)| {
            let winning_set: HashSet<&usize> = HashSet::from_iter(num_pair[0].iter());
            let mut winner_count: usize = 0;
            for num in num_pair[1].clone() {
                if winning_set.contains(&num) {
                    winner_count += 1;
                }
            }

            let previously_established_copies =
                card_copy_map.get(&{ card_num }).unwrap_or(&0_u32).clone();
            card_copy_map.insert(card_num, previously_established_copies + 1);

            if winner_count == 0 {
                return;
            }

            for card in card_num + 1..card_num + 1 + winner_count {
                if card > num_cards {
                    break;
                }
                let curr_winnings = card_copy_map.get(&card);
                if let None = curr_winnings {
                    card_copy_map.insert(card.clone(), previously_established_copies + 1);
                } else {
                    let curr_count = curr_winnings.unwrap();
                    card_copy_map
                        .insert(card.clone(), curr_count + previously_established_copies + 1);
                }
            }
        });
    card_copy_map
        .keys()
        .map(|key| card_copy_map.get(key).unwrap())
        .sum()
}
