use queues::*;
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    println!("part 1: {}", part1(&"test.txt"));
    println!("part 2: {}", part2(&"test.txt"));
}
#[derive(Clone, Copy)]
enum Type {
    Flip,
    Conj,
    Broad,
}
#[derive(Clone)]
enum PulseType {
    Lo,
    Hi,
}
#[derive(Clone)]
struct Pulse {
    typ: PulseType,
    from: String,
    to: String,
}

struct Module {
    name: String,
    typ: Type,
    last_received: Option<HashMap<String, PulseType>>,
    switch: Option<bool>, // only for Flips
    targets: Vec<String>,
}

// only reacts to Lo pulse, starts off
// on receive lo it sends lo if on or hi if off and toggles its pulse state
#[derive(Hash)]
struct Flip {
    switch: bool,
}

// conj remembers last pulse from every input, starts all lo
// on receive update then check if all hi send lo else send hi

// button sends 1 low pulse to broadcast

// broadcast sends whatever it receives to all destinations
// pulses are handled fifo (queue!)

fn part1(path: &str) -> usize {
    // Note moved broadcast line to top of puzzle input (should've viewed it first)
    let data = read_to_string(&path).unwrap();
    let broadcast_targets = data
        .lines()
        .nth(0)
        .unwrap()
        .split("->")
        .nth(1)
        .unwrap()
        .split(',')
        .map(|spl| spl.trim().to_string())
        .collect::<Vec<String>>();
    let broadcast_module = Module {
        name: String::from("broadcaster"),
        typ: Type::Broad,
        last_received: None,
        switch: None,
        targets: broadcast_targets,
    };
    let other_modules = data
        .lines()
        .skip(1)
        .map(|line| {
            let typ = match line.split("->").nth(0).unwrap().chars().nth(0).unwrap() {
                '&' => Type::Conj,
                '%' => Type::Flip,
                _ => panic!(),
            };
            let name = line.split("->").nth(0).unwrap().trim()[1..].to_string();
            let targets = line
                .split("->")
                .nth(1)
                .unwrap()
                .split(',')
                .map(|target| target.trim().to_string())
                .collect::<Vec<String>>();
            Module {
                name,
                typ,
                targets,
                last_received: match typ {
                    Type::Flip => None,
                    Type::Conj => Some(HashMap::new()),
                    Type::Broad => panic!(),
                },
                switch: match typ {
                    Type::Flip => Some(false),
                    Type::Conj => None,
                    Type::Broad => panic!(),
                },
            }
        })
        .collect::<Vec<Module>>();
    // input low to other 1000 times
    // use the "state machine to determine how many pulses are sent"

    let mut lo_count: usize = 0;
    let mut hi_count: usize = 0;
    let mut press_count: usize = 0;

    while press_count < 1000 {
        press_count += 1;

        let mut queue: Queue<Pulse> = queue![Pulse {
            typ: PulseType::Lo,
            from: String::from("Button"),
            to: String::from("Broadcast")
        }];
    }

    todo!()
}

fn part2(path: &str) -> usize {
    let data = read_to_string(&path).unwrap();
    todo!()
}
