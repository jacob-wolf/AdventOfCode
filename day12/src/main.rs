use std::{collections::HashMap, fs::read_to_string, time::Instant};

fn main() {
    let timer = Instant::now();
    println!("{}", part1(&"input.txt"));
    println!("{}", part2(&"input.txt"));
    println!("{}", timer.elapsed().as_millis());
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
        .for_each(|(positions, hint)| total_configs += solve_small_line(positions, hint));

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

    lines.iter().cloned().for_each(|(positions, hint)| {
        total_configs += solve_big_line_wrapper(positions, hint);
    });

    total_configs
}

fn solve_big_line_wrapper(statuses: Vec<Status>, hints: Vec<usize>) -> usize {
    let mut cache: HashMap<(Vec<Status>, Vec<usize>, Option<Status>), usize> = HashMap::new();

    solve_big_line(statuses, hints, None, &mut cache)
}

fn solve_big_line(
    statuses: Vec<Status>,
    hints: Vec<usize>,
    forbidden: Option<Status>,
    cache: &mut HashMap<(Vec<Status>, Vec<usize>, Option<Status>), usize>,
) -> usize {
    if let Some(value) = cache.get(&(statuses.clone(), hints.clone(), forbidden.clone())) {
        return *value;
    }
    // not cached check if empty
    if statuses.is_empty() {
        if hints.is_empty() {
            cache.insert((statuses, hints, forbidden), 1);
            return 1;
        } else {
            cache.insert((statuses, hints, forbidden), 0);
            return 0;
        }
    } else if hints.is_empty() {
        if !statuses.contains(&Status::Damaged) {
            cache.insert((statuses, hints, forbidden), 1);
            return 1;
        } else {
            cache.insert((statuses, hints, forbidden), 0);
            return 0;
        }
    }

    //not cached and have both hints and statuses
    let mut num_matches: usize = 0;

    let mut new_statuses: Vec<Status> = statuses.clone();
    let current_position: Status = new_statuses.remove(0);

    num_matches += match current_position {
        Status::Operational => {
            //if operational call the same thing on the next function
            if forbidden.eq(&Some(Status::Operational)) {
                0
            } else {
                solve_big_line(new_statuses, hints.clone(), None, cache)
            }
        }
        Status::Damaged => {
            if forbidden.eq(&Some(Status::Damaged)) {
                0
            } else {
                let mut new_hints = hints.clone();
                new_hints[0] -= 1;
                let mut next_forbidden = Some(Status::Operational);
                if new_hints[0].eq(&0) {
                    new_hints.remove(0);
                    next_forbidden = Some(Status::Damaged);
                }

                solve_big_line(new_statuses, new_hints, next_forbidden, cache)
            }
        }
        Status::Unknown => {
            //treat both branches (damaged or operational)
            let mut unknown_addition: usize = 0;
            if forbidden.ne(&Some(Status::Operational)) {
                unknown_addition +=
                    solve_big_line(new_statuses.clone(), hints.clone(), None, cache);
            }

            if forbidden.ne(&Some(Status::Damaged)) {
                let mut new_hints = hints.clone();
                new_hints[0] -= 1;
                let mut next_forbidden = Some(Status::Operational);
                if new_hints[0].eq(&0) {
                    new_hints.remove(0);
                    next_forbidden = Some(Status::Damaged);
                }

                unknown_addition += solve_big_line(new_statuses, new_hints, next_forbidden, cache);
            }

            unknown_addition
        }
    };

    cache.insert((statuses, hints, forbidden), num_matches);
    num_matches
}
