use queues::*;
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    println!("part 1: {}", part1(&"test2.txt"));
    println!("part 2: {}", part2(&"test.txt"));
}
#[derive(Clone, Copy)]
enum Type {
    Flip,
    Conj,
    Broad,
}
#[derive(Clone, PartialEq, Eq)]
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
#[derive(Clone)]
struct Module {
    name: String,
    typ: Type,
    last_received: Option<HashMap<String, PulseType>>,
    switch: Option<bool>, // only for Flips
    targets: Vec<String>,
}

// only reacts to Lo pulse, starts off
// on receive lo it sends lo if on or hi if off and toggles its pulse state

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

    let mut module_map: HashMap<String, Module> = HashMap::new();
    module_map.insert(broadcast_module.name.clone(), broadcast_module);
    other_modules.iter().for_each(|module| {
        module_map.insert(module.name.clone(), module.clone());
    });
    // input low to other 1000 times
    // use the "state machine to determine how many pulses are sent"

    let mut lo_count: usize = 0;
    let mut hi_count: usize = 0;
    let mut press_count: usize = 0;

    while press_count < 1000 {
        press_count += 1;

        let mut queue: Queue<Pulse> = queue![Pulse {
            typ: PulseType::Lo,
            from: String::from("button"),
            to: String::from("broadcaster")
        }];

        while queue.size() > 0 {
            let curr_pulse = queue.remove().unwrap();
            match curr_pulse.typ {
                PulseType::Lo => lo_count += 1,
                PulseType::Hi => hi_count += 1,
            };
            let mut curr_rx = match module_map.get(&curr_pulse.to) {
                Some(rx) => rx,
                None => {
                    continue;
                }
            }
            .clone();
            match curr_rx.typ {
                Type::Flip => match curr_pulse.typ {
                    PulseType::Lo => {
                        // toggle the switch and send hi if on now and lo if off now
                        let new_switch_state = !curr_rx.switch.unwrap();
                        curr_rx.switch = Some(new_switch_state);
                        curr_rx.targets.iter().for_each(|target| {
                            queue
                                .add(Pulse {
                                    from: curr_rx.name.clone(),
                                    to: target.clone(),
                                    typ: match new_switch_state {
                                        true => PulseType::Hi,
                                        false => PulseType::Lo,
                                    },
                                })
                                .unwrap();
                        });
                        module_map.insert(curr_rx.name.clone(), curr_rx);
                    }
                    PulseType::Hi => {}
                },
                Type::Conj => {
                    // update last received then send hi unless last seen all hi then lo
                    let mut last_received = curr_rx.last_received.unwrap();
                    last_received.insert(curr_pulse.from, curr_pulse.typ);
                    curr_rx.last_received = Some(last_received.clone());
                    curr_rx.targets.iter().for_each(|target| {
                        queue
                            .add(Pulse {
                                from: curr_rx.name.clone(),
                                to: target.clone(),
                                typ: match curr_rx
                                    .last_received
                                    .clone()
                                    .unwrap()
                                    .values()
                                    .any(|pulse_type| pulse_type.eq(&PulseType::Lo))
                                {
                                    false => {
                                        // send lo if all last_rx are hi
                                        PulseType::Lo
                                    }
                                    true => {
                                        // send hi otherwise
                                        PulseType::Hi
                                    }
                                },
                            })
                            .unwrap();
                    });
                    module_map.insert(curr_rx.name.clone(), curr_rx);
                }
                Type::Broad => {
                    //forward whatever it gets to all targets
                    curr_rx.targets.iter().for_each(|target| {
                        queue
                            .add(Pulse {
                                from: curr_rx.name.clone(),
                                to: target.clone(),
                                typ: curr_pulse.typ.clone(),
                            })
                            .unwrap();
                    });
                }
            };
        }
    }
    println!("{lo_count} lows and {hi_count} his");
    let result = lo_count * hi_count;
    result
}
//11685999 1 too many low and 1 too few hi?
//11687500

fn part2(path: &str) -> usize {
    let data = read_to_string(&path).unwrap();
    todo!()
}
