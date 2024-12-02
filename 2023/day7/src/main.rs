use day7::{p1::Hand as p1Hand, p2::Hand as p2Hand};
use std::fs::read_to_string;
fn main() {
    println!("{:?}", part1(&"input.txt"));
    println!("{:?}", part2(&"input.txt"));
}

fn part1(filepath: &str) -> usize {
    let file = read_to_string(&filepath).unwrap();

    let mut hands = file
        .lines()
        .map(|line| {
            let mut spl_iter = line.split(|c: char| c.is_ascii_whitespace());
            let cards = spl_iter.next().clone().unwrap();
            let bid = spl_iter.next().clone().unwrap().parse::<usize>().unwrap();
            p1Hand::parse_hand(cards, &bid)
        })
        .collect::<Vec<p1Hand>>();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| { index + 1 } * hand.bid)
        .sum::<usize>()
}

fn part2(filepath: &str) -> usize {
    let file = read_to_string(&filepath).unwrap();

    let mut hands = file
        .lines()
        .map(|line| {
            let mut spl_iter = line.split(|c: char| c.is_ascii_whitespace());
            let cards = spl_iter.next().clone().unwrap();
            let bid = spl_iter.next().clone().unwrap().parse::<usize>().unwrap();
            p2Hand::parse_hand(cards, &bid)
        })
        .collect::<Vec<p2Hand>>();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| { index + 1 } * hand.bid)
        .sum::<usize>()
}
