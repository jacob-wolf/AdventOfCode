use advent_of_code_2024::{read_file, Part, Which};
pub fn p13(choice: Which, part: Part) {
    let file_data: String = read_file(13, choice, None);
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
fn part1(data: &str) {
    let mut ans: usize = 0;
    data.split("\r\n\r\n").for_each(|claw_machine| {
        let a_line = claw_machine
            .lines()
            .nth(0)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(',')
            .map(|varplusnum| {
                varplusnum
                    .split('+')
                    .nth(1)
                    .unwrap()
                    .parse::<isize>()
                    .unwrap()
            })
            .collect::<Vec<isize>>();
        let b_line = claw_machine
            .lines()
            .nth(1)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(',')
            .map(|varplusnum| {
                varplusnum
                    .split('+')
                    .nth(1)
                    .unwrap()
                    .parse::<isize>()
                    .unwrap()
            })
            .collect::<Vec<isize>>();
        let ans_line = claw_machine
            .lines()
            .nth(2)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(',')
            .map(|vareqnum| {
                vareqnum
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .parse::<isize>()
                    .unwrap()
            })
            .collect::<Vec<isize>>();

        // equation 1: aline[0]*A_press + bline[0] * B_press = ans_line[0]
        // eqation 2 aline[1] * A_press + bline[1] * B_press = ans_line[1]

        let num = ans_line[0] as f64 - { ans_line[1] as f64 * a_line[0] as f64 / a_line[1] as f64 };
        let denom = b_line[0] as f64 - { b_line[1] as f64 * a_line[0] as f64 / a_line[1] as f64 };
        let b_press = num / denom;
        let a_press = { ans_line[1] as f64 - { b_press * b_line[1] as f64 } } / a_line[1] as f64;

        let a_fpe = a_press - { a_press as usize } as f64;
        let b_fpe = b_press - { b_press as usize } as f64;

        if { a_fpe > 0.00001 && a_fpe < 0.99999 } || { b_fpe > 0.00001 && b_fpe < 0.99999 } {
            return;
        }

        let a_int = if a_fpe > 0.99999 {
            a_press as usize + 1
        } else {
            a_press as usize
        };
        let b_int = if b_fpe > 0.99999 {
            b_press as usize + 1
        } else {
            b_press as usize
        };

        ans += 3 * a_int + b_int;
    });
    println!("{ans}");
}

fn part2(data: &str) {
    let mut ans: usize = 0;
    data.split("\r\n\r\n").for_each(|claw_machine| {
        let a_line = claw_machine
            .lines()
            .nth(0)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(',')
            .map(|varplusnum| {
                varplusnum
                    .split('+')
                    .nth(1)
                    .unwrap()
                    .parse::<isize>()
                    .unwrap()
            })
            .collect::<Vec<isize>>();
        let b_line = claw_machine
            .lines()
            .nth(1)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(',')
            .map(|varplusnum| {
                varplusnum
                    .split('+')
                    .nth(1)
                    .unwrap()
                    .parse::<isize>()
                    .unwrap()
            })
            .collect::<Vec<isize>>();
        let ans_line = claw_machine
            .lines()
            .nth(2)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(',')
            .map(|vareqnum| {
                let p1_num = vareqnum
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .parse::<isize>()
                    .unwrap();
                p1_num + 10000000000000
            })
            .collect::<Vec<isize>>();

        // equation 1: aline[0]*A_press + bline[0] * B_press = ans_line[0]
        // eqation 2 aline[1] * A_press + bline[1] * B_press = ans_line[1]

        let num = ans_line[0] as f64 - { ans_line[1] as f64 * a_line[0] as f64 / a_line[1] as f64 };
        let denom = b_line[0] as f64 - { b_line[1] as f64 * a_line[0] as f64 / a_line[1] as f64 };
        let b_press = num / denom;
        let a_press = { ans_line[1] as f64 - { b_press * b_line[1] as f64 } } / a_line[1] as f64;

        let a_fpe = a_press - { a_press as usize } as f64;
        let b_fpe = b_press - { b_press as usize } as f64;

        if { a_fpe > 0.001 && a_fpe < 0.999 } || { b_fpe > 0.001 && b_fpe < 0.999 } {
            return;
        }

        let a_int = if a_fpe > 0.999 {
            a_press as usize + 1
        } else {
            a_press as usize
        };
        let b_int = if b_fpe > 0.999 {
            b_press as usize + 1
        } else {
            b_press as usize
        };

        ans += 3 * a_int + b_int;
    });
    println!("{ans}");
}
