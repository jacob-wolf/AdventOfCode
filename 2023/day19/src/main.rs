use core::panic;
use std::{collections::HashMap, fs::read_to_string};
fn main() {
    println!("{}", part1(&"input.txt"));
}
#[derive(Debug, Clone, Copy, Hash)]
enum CompAttr {
    X,
    M,
    A,
    S,
}
#[derive(Debug, Clone, Copy, Hash)]
enum OperationType {
    Greater,
    Smaller,
}
#[derive(Debug, Clone, Hash)]
enum NextStep {
    Operation,
    Status,
}
#[derive(Debug, Clone, Hash)]
struct Operation {
    value: usize,
    operator: OperationType,
    comparing: CompAttr,
    if_true: NextStep,
    if_true_name: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Status {
    A,
    R,
}
#[derive(Debug, Clone, Hash)]
struct WorkFlow {
    ops: Vec<Operation>,
    final_false: NextStep,
    final_false_string: String,
}
#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn sum_nums(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn part1(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();

    let workflows_str = file.split("\n\r\n").nth(0).unwrap();
    let workflows = workflows_str
        .lines()
        .map(|line| {
            let name = line.split('{').nth(0).unwrap().to_string();
            let mut operations: Vec<Operation> = vec![];
            let mut final_if_false: String = String::from("");
            let mut final_step: NextStep = NextStep::Status;
            line.split('{')
                .nth(1)
                .unwrap()
                .to_string()
                .trim_end_matches('}')
                .split(',')
                .for_each(|spl_item| {
                    println!("{spl_item}");
                    if spl_item.contains(':') {
                        let op_if_true = spl_item.split(':').collect::<Vec<&str>>();
                        let value: usize = op_if_true[0]
                            .chars()
                            .skip(2)
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap();

                        let if_true = if op_if_true[1].len() > 1 {
                            NextStep::Operation
                        } else {
                            NextStep::Status
                        };
                        let if_true_name = op_if_true[1].to_string();

                        let operator = if op_if_true[0].contains('<') {
                            OperationType::Smaller
                        } else {
                            OperationType::Greater
                        };

                        let comparing = match op_if_true[0].chars().nth(0).unwrap() {
                            'x' => CompAttr::X,
                            'm' => CompAttr::M,
                            'a' => CompAttr::A,
                            's' => CompAttr::S,
                            _ => panic!("Invalid attribute"),
                        };

                        operations.push(Operation {
                            comparing,
                            if_true,
                            if_true_name,
                            operator,
                            value,
                        });
                    } else {
                        final_if_false = spl_item.to_string();
                        if final_if_false.len() > 1 {
                            final_step = NextStep::Operation
                        }
                    }
                });
            (
                name.clone(),
                WorkFlow {
                    //name,
                    ops: operations,
                    final_false: final_step,
                    final_false_string: final_if_false,
                },
            )
        })
        .collect::<HashMap<String, WorkFlow>>();

    let parts_str = file.split("\n\r\n").nth(1).unwrap();
    let parts = parts_str
        .lines()
        .map(|line| line.trim_matches(|c: char| c.eq(&'{') || c.eq(&'}')))
        .map(|line_no_bracket| {
            let pairs = line_no_bracket
                .split(',')
                .map(|part_enum| {
                    let mut x = part_enum.split('=');
                    let attr = x.next().unwrap();
                    let num = x.next().unwrap();
                    (attr, num)
                })
                .collect::<Vec<(&str, &str)>>();
            Part {
                x: pairs
                    .iter()
                    .find(|pair| pair.0.eq("x"))
                    .unwrap()
                    .1
                    .parse::<usize>()
                    .unwrap(),
                m: pairs
                    .iter()
                    .find(|pair| pair.0.eq("m"))
                    .unwrap()
                    .1
                    .parse::<usize>()
                    .unwrap(),
                a: pairs
                    .iter()
                    .find(|pair| pair.0.eq("a"))
                    .unwrap()
                    .1
                    .parse::<usize>()
                    .unwrap(),
                s: pairs
                    .iter()
                    .find(|pair| pair.0.eq("s"))
                    .unwrap()
                    .1
                    .parse::<usize>()
                    .unwrap(),
            }
        })
        .collect::<Vec<Part>>();
    println!("{parts:?}");
    println!("{workflows:?}");

    parts
        .iter()
        .map(|part| determine_part_feasibility(&part, &workflows))
        .filter(|(status, _score)| status.eq(&Status::A))
        .map(|(_status, score)| score)
        .sum::<usize>()
}

fn determine_part_feasibility(
    part: &Part,
    workflows: &HashMap<String, WorkFlow>,
) -> (Status, usize) {
    let mut status: Option<Status> = None;
    let mut curr_workflow = workflows.get("in").unwrap();

    while let None = status {
        for op in curr_workflow.ops.clone() {
            let part_comparison_value = match op.comparing {
                CompAttr::X => part.x,
                CompAttr::M => part.m,
                CompAttr::A => part.a,
                CompAttr::S => part.s,
            };
            let comp_success = match op.operator {
                OperationType::Greater => part_comparison_value > op.value,
                OperationType::Smaller => part_comparison_value < op.value,
            };
            if !comp_success {
                continue;
            }
            match op.if_true {
                NextStep::Operation => {
                    curr_workflow = workflows.get(&op.if_true_name).unwrap();
                    break;
                }
                NextStep::Status => {
                    status = if op.if_true_name.eq(&String::from("A")) {
                        Some(Status::A)
                    } else {
                        Some(Status::R)
                    };
                    return (
                        status.clone().unwrap(),
                        if status.unwrap().eq(&Status::A) {
                            part.sum_nums()
                        } else {
                            0
                        },
                    );
                }
            };
        }
        match curr_workflow.final_false {
            NextStep::Operation => {
                curr_workflow = workflows.get(&curr_workflow.final_false_string).unwrap();
            }
            NextStep::Status => {
                status = if curr_workflow.final_false_string.eq(&String::from("A")) {
                    Some(Status::A)
                } else {
                    Some(Status::R)
                };
                break;
            }
        }
    }
    if let Some(result) = status {
        return (
            result.clone(),
            if result.eq(&Status::A) {
                part.sum_nums()
            } else {
                0
            },
        );
    } else {
        panic!("part didn't reach a final status");
    }
}
//505416 too high
//268050 too low
