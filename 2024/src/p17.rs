use advent_of_code_2024::{read_file, Part, Which};

pub fn p17(choice: Which, part: Part) {
    let file_data: String = read_file(17, choice, Some(part));
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
#[derive(Debug)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

fn get_combo_operand(opr: usize, a: usize, b: usize, c: usize) -> usize {
    match opr {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!(),
    }
}

fn compute(
    op: &Op,
    opr: &usize,
    a: &mut usize,
    b: &mut usize,
    c: &mut usize,
    pointer: &mut usize,
) -> Option<usize> {
    match op {
        Op::Adv => {
            let operand = get_combo_operand(*opr, *a, *b, *c);
            let num = *a;
            let denom = { 2 as usize }.pow(operand as u32);
            *a = num / denom;
            *pointer += 1;
        }
        Op::Bxl => {
            let operand = opr;
            *b = *b ^ *operand;
            *pointer += 1;
        }
        Op::Bst => {
            let operand = get_combo_operand(*opr, *a, *b, *c);
            *b = operand % 8;
            *pointer += 1;
        }
        Op::Jnz => {
            let operand = *opr;
            if *a != 0 {
                *pointer = operand as usize;
            } else {
                *pointer += 1;
            }
        }
        Op::Bxc => {
            // no operand
            *b = *b ^ *c;
            *pointer += 1;
        }
        Op::Out => {
            let operand = get_combo_operand(*opr, *a, *b, *c);
            *pointer += 1;
            return Some(operand % 8);
        }
        Op::Bdv => {
            let operand = get_combo_operand(*opr, *a, *b, *c);
            let num = *a;
            let denom = { 2 as usize }.pow(operand as u32);
            *b = num / denom;
            *pointer += 1;
        }
        Op::Cdv => {
            let operand = get_combo_operand(*opr, *a, *b, *c);
            let num = *a;
            let denom = { 2 as usize }.pow(operand as u32);
            *c = num / denom;
            *pointer += 1;
        }
    };
    return None;
}


fn part1(data: &str) {
    let mut a = data
        .lines()
        .nth(0)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut b: usize = data
        .lines()
        .nth(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut c: usize = data
        .lines()
        .nth(2)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut switch: bool = true;
    let mut ops = vec![];
    let mut oprs = vec![];

    data.split("\r\n\r\n")
        .nth(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .for_each(|num| {
            if switch {
                ops.push(match num.parse::<usize>().unwrap() {
                    0 => Op::Adv,
                    1 => Op::Bxl,
                    2 => Op::Bst,
                    3 => Op::Jnz,
                    4 => Op::Bxc,
                    5 => Op::Out,
                    6 => Op::Bdv,
                    7 => Op::Cdv,
                    _ => panic!(),
                });
            } else {
                oprs.push(num.parse::<usize>().unwrap())
            }
            switch = !switch;
        });

    let mut pointer: usize = 0;
    let mut output: Vec<usize> = vec![];
    while pointer < ops.len() {
        match compute(
            &ops[pointer as usize],
            &oprs[pointer as usize],
            &mut a,
            &mut b,
            &mut c,
            &mut pointer,
        ) {
            Some(val) => output.push(val),
            None => {}
        }
    }
    let last = output.len() - 1;
    output.iter().enumerate().for_each(|(idx, out)| {
        if idx != last {
            print!("{out},")
        } else {
            println!("{out}")
        }
    });
}

fn part2(data: &str) {
    let mut switch: bool = true;
    let mut ops = vec![];
    let mut oprs = vec![];
    let mut target_program = vec![];
    data.split("\r\n\r\n")
        .nth(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .for_each(|num_str| {
            let num = num_str.parse::<usize>().unwrap();
            if switch {
                ops.push(match num {
                    0 => Op::Adv,
                    1 => Op::Bxl,
                    2 => Op::Bst,
                    3 => Op::Jnz,
                    4 => Op::Bxc,
                    5 => Op::Out,
                    6 => Op::Bdv,
                    7 => Op::Cdv,
                    _ => panic!(),
                });
            } else {
                oprs.push(num)
            }
            target_program.push(num);
            switch = !switch;
        });

    // loop through the program backwards collecting sets of 3 bits for A
    // know that A must end at 0
    // find the necessary start_a s.t. the program outputs the correct value
    let mut solutions: Vec<usize> = vec![];
    let mut problems_to_check: Vec<(usize, usize)> = vec![(0, target_program.len() - 1)];
    while !problems_to_check.is_empty() {
        let (end_a, target_idx) = problems_to_check.pop().unwrap();
        let target = target_program[target_idx];
        for possible in 0..8 {
            let mut a = end_a * 8 + possible;
            let mut b = 0;
            let mut c = 0;
            let mut pointer = 0;
            loop {
                match compute(
                    &ops[pointer],
                    &oprs[pointer],
                    &mut a,
                    &mut b,
                    &mut c,
                    &mut pointer,
                ) {
                    Some(val) => {
                        if val.eq(&target) {
                            if target_idx > 0 {
                                // the next end_a is this iterations start a
                                problems_to_check.push((end_a * 8 + possible, target_idx - 1));
                            } else {
                                solutions.push(end_a * 8 + possible);
                            }
                        }
                        break;
                    }
                    None => {}
                }
            }
        }
    }
    solutions.sort();
    println!("{}", solutions[0]);
}
