use std::{collections::HashMap, fs::read_to_string};
#[macro_use]
extern crate queues;

use queues::*;

fn main() {
    println!("part 1: {}", part1(&"input.txt"));
    println!("part 2: {}", part2(&"test.txt"));
}
#[derive(Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn sum(self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

struct Workflow {
    operations: Vec<Operation>,
    name: String,
}
#[derive(PartialEq, Eq, Clone)]
struct Operation {
    parent_workflow: String,
    attr: Option<char>,
    if_true: String,
    cmp: Option<char>,
    num: Option<usize>,
}

fn part1(path: &str) -> usize {
    // read in the parts and the workflows
    let data = read_to_string(&path).unwrap();
    let split_index = data
        .lines()
        .enumerate()
        .find(|(_, line)| line.trim().is_empty())
        .unwrap()
        .0;

    let parts = data
        .lines()
        .skip(split_index + 1)
        .map(|line| {
            let mut vals: [usize; 4] = [0, 0, 0, 0];
            line[1..line.len() - 1].split(',').for_each(|part_val_str| {
                let vals_index = match part_val_str.chars().nth(0).unwrap() {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => panic!(),
                };
                //println!("{part_val_str}");
                vals[vals_index] = part_val_str
                    .split('=')
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            });
            Part {
                x: vals[0],
                m: vals[1],
                a: vals[2],
                s: vals[3],
            }
        })
        .collect::<Vec<Part>>();

    let workflows = data
        .lines()
        .take(split_index)
        .map(|line| {
            let name = line.split('{').nth(0).unwrap().to_string();
            let mut operations_str = line.split('{').nth(1).unwrap();
            operations_str = &operations_str[0..operations_str.len() - 1];

            let operations = operations_str
                .split(',')
                .map(|operation| {
                    if !operation.contains(':') {
                        return Operation {
                            attr: None,
                            cmp: None,
                            num: None,
                            if_true: operation.to_string(),
                            parent_workflow: name.clone(),
                        };
                    }
                    // looks like part_attr<num:str
                    let part_attr = operation.chars().nth(0).unwrap();
                    let cmp = operation.chars().nth(1).unwrap();
                    //println!("{}", operation);
                    let num = operation[2..]
                        .split(':')
                        .nth(0)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    let if_true_str = operation[2..].split(':').nth(1).unwrap().to_string();

                    Operation {
                        attr: Some(part_attr),
                        cmp: Some(cmp),
                        num: Some(num),
                        if_true: if_true_str,
                        parent_workflow: name.clone(),
                    }
                })
                .collect::<Vec<Operation>>();

            Workflow { operations, name }
        })
        .collect::<Vec<Workflow>>();

    let mut workflow_map: HashMap<&String, &Workflow> = HashMap::new();

    workflows.iter().for_each(|workflow: &Workflow| {
        workflow_map.insert(&workflow.name, workflow);
    });

    let result = parts
        .iter()
        .map(|part| {
            let mut next_workflow = String::from("in");
            while next_workflow.ne(&"A") && next_workflow.ne(&"R") {
                let workflow = workflow_map.get(&next_workflow).unwrap();
                for operation in workflow.operations.iter() {
                    match operation.cmp {
                        Some('>') => match operation.attr {
                            Some('x') => {
                                if part.x > operation.num.unwrap() {
                                    next_workflow = operation.if_true.clone();
                                    break;
                                }
                            }
                            Some('m') => {
                                if part.m > operation.num.unwrap() {
                                    next_workflow = operation.if_true.clone();
                                    break;
                                }
                            }
                            Some('a') => {
                                if part.a > operation.num.unwrap() {
                                    next_workflow = operation.if_true.clone();
                                    break;
                                }
                            }
                            Some('s') => {
                                if part.s > operation.num.unwrap() {
                                    next_workflow = operation.if_true.clone();
                                    break;
                                }
                            }
                            _ => panic!(),
                        },
                        Some('<') => match operation.attr {
                            Some('x') => {
                                if part.x < operation.num.unwrap() {
                                    next_workflow = operation.if_true.clone();
                                    break;
                                }
                            }
                            Some('m') => {
                                if part.m < operation.num.unwrap() {
                                    next_workflow = operation.if_true.clone();
                                    break;
                                }
                            }
                            Some('a') => {
                                if part.a < operation.num.unwrap() {
                                    next_workflow = operation.if_true.clone();
                                    break;
                                }
                            }
                            Some('s') => {
                                if part.s < operation.num.unwrap() {
                                    next_workflow = operation.if_true.clone();
                                    break;
                                }
                            }
                            _ => panic!(),
                        },
                        None => next_workflow = operation.if_true.clone(),
                        _ => panic!(),
                    }
                }
            }

            match next_workflow.chars().nth(0).unwrap() {
                'A' => part.clone().sum(),
                'R' => 0,
                _ => panic!(),
            }
        })
        .sum::<usize>();
    result
}
//505416 too high
//421983 just right!
//268050 too low

fn part2(path: &str) -> usize {
    // how many combinations of ratings are valid?
    // find ranges of x, m, a, s that lead to an A!
    // range of 1-4000 for each

    // construct a Graph where each node is a workflow, A or R
    // edges are the unidirectional operations
    // traverse the graph bfs and store the edge paths in a vec
    //      if the same edge appears twice in the list discard the branch of the bfs
    // collect all the valid paths from in to A(no cycle)
    // for each valid path aggregate the constraints
    // count how many parts meet the constraints
    let data = read_to_string(&path).unwrap();
    let split_index = data
        .lines()
        .enumerate()
        .find(|(_, line)| line.trim().is_empty())
        .unwrap()
        .0;

    let workflows = data
        .lines()
        .take(split_index)
        .map(|line| {
            let name = line.split('{').nth(0).unwrap().to_string();
            let mut operations_str = line.split('{').nth(1).unwrap();
            operations_str = &operations_str[0..operations_str.len() - 1];

            let operations = operations_str
                .split(',')
                .map(|operation| {
                    if !operation.contains(':') {
                        return Operation {
                            attr: None,
                            cmp: None,
                            num: None,
                            if_true: operation.to_string(),
                            parent_workflow: name.clone(),
                        };
                    }
                    // looks like part_attr<num:str
                    let part_attr = operation.chars().nth(0).unwrap();
                    let cmp = operation.chars().nth(1).unwrap();
                    //println!("{}", operation);
                    let num = operation[2..]
                        .split(':')
                        .nth(0)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    let if_true_str = operation[2..].split(':').nth(1).unwrap().to_string();

                    Operation {
                        attr: Some(part_attr),
                        cmp: Some(cmp),
                        num: Some(num),
                        if_true: if_true_str,
                        parent_workflow: name.clone(),
                    }
                })
                .collect::<Vec<Operation>>();

            Workflow { operations, name }
        })
        .collect::<Vec<Workflow>>();

    let mut workflow_map: HashMap<&String, &Workflow> = HashMap::new();

    workflows.iter().for_each(|workflow: &Workflow| {
        workflow_map.insert(&workflow.name, workflow);
    });

    let first_check_item: (&Workflow, Vec<(Operation, bool)>) =
        (workflow_map.get(&String::from("in")).unwrap(), vec![]);

    let mut queue: Queue<(&Workflow, Vec<(Operation, bool)>)> = queue![];
    let _ = queue.add(first_check_item);
    let mut valid_paths: Vec<Vec<(Operation, bool)>> = vec![];

    while queue.size() != 0 {
        let (curr_workflow, curr_path) = queue.remove().unwrap();
        for (op_idx, operation) in curr_workflow.operations.iter().enumerate() {
            if operation.if_true.eq(&"A") {
                //in -> A
                let mut new_valid_path = curr_path.clone();
                for push_idx in 0..=op_idx {
                    new_valid_path.push(curr_workflow.operations[push_idx].clone())
                }
                valid_paths.push(new_valid_path);
                continue;
            }
            if operation.if_true.eq(&"R") {
                //in -> R
                continue;
            }
            // in -> ?
            if curr_path.contains(operation) {
                // cycle!
                continue;
            }
            // not a cycle or a dead end
            let mut new_path = curr_path.clone();
            new_path.push(operation.clone());
            let new_workflow = workflow_map.get(&operation.if_true).unwrap();
            let _ = queue.add((new_workflow, new_path));
        }
    }
    valid_paths.iter().enumerate().for_each(|(idx, path)| {
        println!("{idx}");
        path.iter()
            .for_each(|op| print!("{:?}{:?}{:?}->", op.attr, op.cmp, op.num));
        println!("");
    });
    println!("{}", valid_paths.len());
    todo!()
}
