use std::fs::read_to_string;

fn main() {
    println!("S{}", part1(&"input.txt"));
    println!("S{}", part2(&"test2.txt"));
}
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

fn part1(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();

    let lines = file
        .lines()
        .map(|line| {
            let spl = line.split_ascii_whitespace().collect::<Vec<&str>>();

            //can trim leading and trailing "operational" statuses b/c irrelevant
            let trimmed_statuses = spl[0]
                .trim_matches('.')
                .chars()
                .map(|status_char| match status_char {
                    '.' => Status::Operational,
                    '#' => Status::Damaged,
                    _ => Status::Unknown,
                })
                .collect::<Vec<Status>>();

            (
                trimmed_statuses,
                spl[1]
                    .split(',')
                    .map(|splitem| splitem.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect::<Vec<(Vec<Status>, Vec<usize>)>>();
    let mut total_configs: usize = 0;

    lines
        .iter()
        .for_each(|(positions, hint)| total_configs += solve_small_line(&positions, &hint));

    total_configs
}

fn solve_small_line(statuses: &Vec<Status>, hints: &Vec<usize>) -> usize {
    let mut raw_count: usize = 0;
    let mut queue_to_check: Vec<(Vec<Status>, Vec<usize>, Option<Status>)> =
        vec![(statuses.clone(), hints.clone(), None)];

    while !queue_to_check.is_empty() {
        let (curr_positions, curr_hints, invalid_status) = queue_to_check.pop().unwrap();
        if { curr_positions.len() as isize } < {
            curr_hints.iter().sum::<usize>() as isize + curr_hints.len() as isize - 1
        } {
            continue; //total broken + number of commas (separators) is minimum position count required
        } else if curr_positions
            .iter()
            .filter(|position| { *position }.ne(&Status::Operational))
            .count()
            < curr_hints.iter().sum::<usize>()
        {
            continue; //total possible remaining >= sum of broken
        }

        if curr_positions.is_empty() {
            if curr_hints.is_empty() {
                raw_count += 1;
            }
            continue;
        } else if curr_hints.is_empty() {
            if !curr_positions.contains(&Status::Damaged) {
                raw_count += 1;
            }
            continue;
        }

        let mut new_positions = curr_positions.clone();
        let curr_position = new_positions.remove(0);

        match curr_position {
            Status::Operational => {
                if invalid_status == Some(Status::Operational) {
                    continue;
                }
                queue_to_check.push((new_positions.clone(), curr_hints.clone(), None));
            }
            Status::Damaged => {
                //if the status is damaged, the next status must be damaged or operational
                //this depends on whether the current hint is exhausted
                if invalid_status == Some(Status::Damaged) {
                    continue;
                }

                let mut new_hints = curr_hints.clone();
                new_hints[0] -= 1;
                let mut next_invalid_status: Option<Status> = Some(Status::Operational);
                if new_hints[0].eq(&0) {
                    let removed_hint = new_hints.remove(0);
                    assert!(removed_hint.eq(&0));
                    next_invalid_status = Some(Status::Damaged);
                }
                queue_to_check.push((
                    new_positions.clone(),
                    new_hints.clone(),
                    next_invalid_status,
                ));
            }
            Status::Unknown => {
                //treat as operational -> No expected follow-up status
                if invalid_status != Some(Status::Operational) {
                    queue_to_check.push((new_positions.clone(), curr_hints.clone(), None));
                }

                //treat as damaged -> determine expected follow-up status
                if invalid_status != Some(Status::Damaged) {
                    let mut new_hints = curr_hints.clone();
                    new_hints[0] -= 1;
                    let mut must_be_op: Option<Status> = Some(Status::Operational);
                    if new_hints[0].eq(&0) {
                        let removed_hint = new_hints.remove(0);
                        assert!(removed_hint.eq(&0));
                        must_be_op = Some(Status::Damaged);
                    }

                    queue_to_check.push((new_positions.clone(), new_hints.clone(), must_be_op));
                }
            }
        };
    }

    raw_count
}

fn part2(path: &str) -> usize {
    let file = read_to_string(&path).unwrap();

    let lines = file
        .lines()
        .map(|line| {
            let spl = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let mut statuses = spl[0]
                .chars()
                .map(|status_char| match status_char {
                    '.' => Status::Operational,
                    '#' => Status::Damaged,
                    _ => Status::Unknown,
                })
                .collect::<Vec<Status>>();

            statuses.push(Status::Unknown);

            statuses = statuses
                .iter()
                .cloned()
                .cycle()
                .take(statuses.len() * 5 - 1)
                .collect::<Vec<Status>>();

            let first_non_op = statuses
                .iter()
                .enumerate()
                .find(|(_index, status)| { *status }.ne(&Status::Operational))
                .unwrap()
                .0;
            let last_non_op = statuses
                .iter()
                .enumerate()
                .rfind(|(_index, status)| { *status }.ne(&Status::Operational))
                .unwrap()
                .0;
            statuses = statuses[first_non_op..last_non_op + 1].to_vec();

            (
                statuses,
                spl[1]
                    .split(',')
                    .map(|splitem| splitem.parse::<usize>().unwrap())
                    .cycle()
                    .take(spl[1].split(',').count() * 5)
                    .collect::<Vec<usize>>(),
            )
        })
        .collect::<Vec<(Vec<Status>, Vec<usize>)>>();
    let mut total_configs: usize = 0;

    lines.iter().for_each(|(positions, hint)| {
        total_configs += solve_big_line(&positions, &hint);
    });

    total_configs
}

fn solve_big_line(statuses: &Vec<Status>, hints: &Vec<usize>) -> usize {
    let mut raw_count: usize = 0;
    let mut queue_to_check: Vec<(Vec<Status>, Vec<usize>, Option<Status>)> =
        vec![(statuses.clone(), hints.clone(), None)];

    while !queue_to_check.is_empty() {
        let (curr_positions, curr_hints, invalid_status) = queue_to_check.pop().unwrap();
        let sum = curr_hints.iter().sum::<usize>();

        if curr_positions
            .iter()
            .filter(|position| { *position }.ne(&Status::Operational))
            .count()
            < sum
        {
            println!("Filtered");
            continue; //total possible remaining >= sum of broken
        }

        if curr_positions.is_empty() {
            if curr_hints.is_empty() {
                raw_count += 1;
            }
            continue;
        } else if curr_hints.is_empty() {
            if !curr_positions.contains(&Status::Damaged) {
                raw_count += 1;
            }
            continue;
        }

        let mut new_positions = curr_positions.clone();
        let curr_position = new_positions.remove(0);

        match curr_position {
            Status::Operational => {
                if invalid_status == Some(Status::Operational) {
                    continue;
                }
                queue_to_check.push((new_positions.clone(), curr_hints.clone(), None));
            }
            Status::Damaged => {
                //if the status is damaged, the next status must be damaged or operational
                //this depends on whether the current hint is exhausted
                if invalid_status == Some(Status::Damaged) {
                    continue;
                }

                let mut new_hints = curr_hints.clone();
                new_hints[0] -= 1;
                let mut next_invalid_status: Option<Status> = Some(Status::Operational);
                if new_hints[0].eq(&0) {
                    let removed_hint = new_hints.remove(0);
                    assert!(removed_hint.eq(&0));
                    next_invalid_status = Some(Status::Damaged);
                }
                queue_to_check.push((
                    new_positions.clone(),
                    new_hints.clone(),
                    next_invalid_status,
                ));
            }
            Status::Unknown => {
                //treat as operational -> No expected follow-up status
                if invalid_status != Some(Status::Operational) {
                    queue_to_check.push((new_positions.clone(), curr_hints.clone(), None));
                }

                //treat as damaged -> determine expected follow-up status
                if invalid_status != Some(Status::Damaged) {
                    let mut new_hints = curr_hints.clone();
                    new_hints[0] -= 1;
                    let mut must_be_op: Option<Status> = Some(Status::Operational);
                    if new_hints[0].eq(&0) {
                        let removed_hint = new_hints.remove(0);
                        assert!(removed_hint.eq(&0));
                        must_be_op = Some(Status::Damaged);
                    }

                    queue_to_check.push((new_positions.clone(), new_hints.clone(), must_be_op));
                }
            }
        };
    }

    raw_count
}
