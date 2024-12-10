use queues::*;
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    println!("part 1: {}", part1(&"input.txt"));
    println!("part 2: {}", part2(&"input.txt"));
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

    // initialize the conjunction modules
    module_map.clone().keys().for_each(|module_name| {
        let starting_module = module_map.get(module_name).unwrap();
        let targets = starting_module.targets.clone();
        targets.iter().for_each(|target| {
            let mut mod_target = match module_map.get(target) {
                Some(target) => target,
                None => {
                    return;
                }
            }
            .clone();
            // get the target
            match mod_target.typ {
                Type::Flip => return,
                Type::Conj => {
                    // if the target is a conjunction get it's last received map
                    let mut recv_map = mod_target.last_received.unwrap().clone();
                    // ensure it remembers this tx as sending lo initially
                    recv_map.insert(module_name.clone(), PulseType::Lo);
                    // update the target object
                    mod_target.last_received = Some(recv_map);
                    // update the working module map
                    module_map.insert(mod_target.name.clone(), mod_target);
                }
                Type::Broad => return,
            };
        });
    });

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
                    PulseType::Hi => {
                        continue;
                    }
                },
                Type::Conj => {
                    // update last received then send hi unless last seen all hi then lo
                    let mut last_received = curr_rx.last_received.unwrap();
                    last_received.insert(curr_pulse.from, curr_pulse.typ);
                    curr_rx.last_received = Some(last_received.clone());
                    let pulse_typ_to_send = match last_received
                        .values()
                        .any(|pulse_type| pulse_type.eq(&PulseType::Lo))
                    {
                        true => PulseType::Hi,
                        false => PulseType::Lo,
                    };
                    curr_rx.targets.iter().for_each(|target| {
                        queue
                            .add(Pulse {
                                from: curr_rx.name.clone(),
                                to: target.clone(),
                                typ: pulse_typ_to_send.clone(),
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

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}
fn lcm(a: usize, b: usize) -> usize {
    b * a / gcd(a, b)
}

/// Some what hard coded -> i.e. code doesn't account for more compicated wirings to rx.
/// Know that a single conj module leads into rx
/// Can find when that conj module gets a single high last_recv from each
/// and lcm the press amounts when that happens
fn part2(path: &str) -> usize {
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

    let mut rx_input_name = other_modules
        .iter()
        .cloned()
        .find(|module| module.targets.contains(&String::from("rx")))
        .unwrap()
        .name;

    let mut module_map: HashMap<String, Module> = HashMap::new();
    module_map.insert(broadcast_module.name.clone(), broadcast_module);
    other_modules.iter().for_each(|module| {
        module_map.insert(module.name.clone(), module.clone());
    });

    // initialize the conjunction modules
    module_map.clone().keys().for_each(|module_name| {
        let starting_module = module_map.get(module_name).unwrap();
        let targets = starting_module.targets.clone();
        targets.iter().for_each(|target| {
            let mut mod_target = match module_map.get(target) {
                Some(target) => target,
                None => {
                    return;
                }
            }
            .clone();
            // get the target
            match mod_target.typ {
                Type::Flip => return,
                Type::Conj => {
                    // if the target is a conjunction get it's last received map
                    let mut recv_map = mod_target.last_received.unwrap().clone();
                    // ensure it remembers this tx as sending lo initially
                    recv_map.insert(module_name.clone(), PulseType::Lo);
                    // update the target object
                    mod_target.last_received = Some(recv_map);
                    // update the working module map
                    module_map.insert(mod_target.name.clone(), mod_target);
                }
                Type::Broad => return,
            };
        });
    });

    let mut press_count: usize = 0;
    let mut rx_inputs_presses: HashMap<String, usize> = HashMap::new();
    loop {
        press_count += 1;
        if press_count % 1_000_000 == 0 {
            println!("Press progress: {press_count}");
        }

        let mut queue: Queue<Pulse> = queue![Pulse {
            typ: PulseType::Lo,
            from: String::from("button"),
            to: String::from("broadcaster")
        }];

        while queue.size() > 0 {
            let curr_pulse = queue.remove().unwrap();
            if curr_pulse.to.eq(&rx_input_name) {
                if curr_pulse.typ.eq(&PulseType::Hi) {
                    if let None = rx_inputs_presses.get(&curr_pulse.from) {
                        rx_inputs_presses.insert(curr_pulse.from.clone(), press_count);
                    }
                }
                if rx_inputs_presses.len() >= 4 {
                    println!("{:?}", rx_inputs_presses);
                    break;
                }
            }

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
                    PulseType::Hi => {
                        continue;
                    }
                },
                Type::Conj => {
                    // update last received then send hi unless last seen all hi then lo
                    let mut last_received = curr_rx.last_received.unwrap();
                    last_received.insert(curr_pulse.from, curr_pulse.typ);
                    curr_rx.last_received = Some(last_received.clone());
                    let pulse_typ_to_send = match last_received
                        .values()
                        .any(|pulse_type| pulse_type.eq(&PulseType::Lo))
                    {
                        true => PulseType::Hi,
                        false => PulseType::Lo,
                    };
                    curr_rx.targets.iter().for_each(|target| {
                        queue
                            .add(Pulse {
                                from: curr_rx.name.clone(),
                                to: target.clone(),
                                typ: pulse_typ_to_send.clone(),
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
        if rx_inputs_presses.len() >= 4 {
            let press_counts = rx_inputs_presses.values().cloned().collect::<Vec<usize>>();
            let mut running_lcm: usize = press_counts[0];
            for i in 1..press_counts.len() {
                running_lcm = lcm(running_lcm, press_counts[i]);
            }
            return running_lcm;
        }
    }
}
