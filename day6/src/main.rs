use std::fs::read_to_string;
fn main() {
    println!("{}", part1(&"input.txt"));
    println!("{}", part2(&"input.txt"));
}
#[derive(Debug, Clone)]
struct Race {
    time: i64,
    dist: i64,
}

fn part1(filepath: &str) -> i64 {
    let file = read_to_string(&filepath).unwrap();
    let mut races: Vec<Race> = vec![];
    let time_distances = file
        .lines()
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|num_str| num_str.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    for index in 0..time_distances[0].len() {
        races.push(Race {
            time: time_distances[0][index],
            dist: time_distances[1][index],
        });
    }
    // quadratic inequality in free param Ψ (the hold time)
    // 0 > Ψ^2 - T*Ψ + D
    // if two solutions the whole range
    // if one integer solution then that one
    // else no winning hold times
    let mut product = 1;
    for race in races {
        if race.time * race.time < 4 * race.dist {
            //yields complex solutions immediately zeros everything
            return 0;
        }
        let plus_minus = f64::sqrt({ race.time * race.time - 4 * race.dist } as f64);

        let top_solution = 0.5 * { race.time as f64 + plus_minus };
        let bottom_solution = 0.5 * { race.time as f64 - plus_minus };

        let mut bottom = bottom_solution.ceil() as i64;
        let mut top = top_solution.floor() as i64;

        if compute_inequality_rhs(&bottom, &race.time, &race.dist) >= 0 {
            bottom += 1;
        }

        if compute_inequality_rhs(&top, &race.time, &race.dist) >= 0 {
            top -= 1;
        }

        product *= top - bottom + 1;
    }

    product
}

fn compute_inequality_rhs(soln: &i64, t: &i64, d: &i64) -> i64 {
    soln * soln - *t * soln + *d
}

fn part2(filepath: &str) -> i64 {
    let file = read_to_string(&filepath).unwrap();
    let time_distances = file
        .lines()
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .trim()
                .replace(|c: char| c.is_ascii_whitespace(), &"")
                .as_str()
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<i64>>();
    let race = Race {
        time: time_distances[0],
        dist: time_distances[1],
    };

    if race.time * race.time < 4 * race.dist {
        return 0;
    }
    let plus_minus = f64::sqrt({ race.time * race.time - 4 * race.dist } as f64);

    let top_solution = 0.5 * { race.time as f64 + plus_minus };
    let bottom_solution = 0.5 * { race.time as f64 - plus_minus };

    let mut bottom = bottom_solution.ceil() as i64;
    let mut top = top_solution.floor() as i64;

    if compute_inequality_rhs(&bottom, &race.time, &race.dist) >= 0 {
        bottom += 1;
    }
    if compute_inequality_rhs(&top, &race.time, &race.dist) >= 0 {
        top -= 1;
    }

    top - bottom + 1
}
