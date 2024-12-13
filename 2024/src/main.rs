use advent_of_code_2024::{Part, Which};
use std::io;
mod p1;
mod p10;
mod p11;
mod p12;
mod p13;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;
mod p8;
mod p9;

fn main() {
    let number = get_problem_number().unwrap();
    let choice = get_data_selection().unwrap();
    let part = get_part_selection().unwrap();

    // Perform an action based on the choice
    match number {
        1 => p1::p1(choice, part),
        2 => p2::p2(choice, part),
        3 => p3::p3(choice, part),
        4 => p4::p4(choice, part),
        5 => p5::p5(choice, part),
        6 => p6::p6(choice, part),
        7 => p7::p7(choice, part),
        8 => p8::p8(choice, part),
        9 => p9::p9(choice, part),
        10 => p10::p10(choice, part),
        11 => p11::p11(choice, part),
        12 => p12::p12(choice, part),
        13 => p13::p13(choice, part),
        _ => {
            if number > 25 || number < 1 {
                panic!("Not a valid choice");
            }
            panic!("Don't have that solution implemented!")
        }
    }
}

fn get_problem_number() -> Option<u32> {
    // Prompt user to enter a number
    println!("Please enter a number between 1 and 25:");
    let mut num_input = String::new();

    io::stdin()
        .read_line(&mut num_input)
        .expect("Failed to read input");

    // Match the number to 1-25 and store it in a u32
    let number: u32 = match num_input.trim().parse::<u32>() {
        Ok(n) if n >= 1 && n <= 25 => n,
        _ => {
            println!("Invalid number! Please enter a number between 1 and 25.");
            return None;
        }
    };

    println!("You entered the number: {} \n", number);
    return Some(number);
}

fn get_data_selection() -> Option<Which> {
    // Prompt user to enter either 't' or 'r'
    println!("Please enter 't' to use Test data or 'r' for Real problem data:");
    let mut char_input = String::new();

    io::stdin()
        .read_line(&mut char_input)
        .expect("Failed to read input");

    // Match the input to the enum
    let choice: Which = match char_input.trim() {
        "t" => {
            println!("You selected Test.\n");
            Which::Test
        }
        "r" => {
            println!("You selected Real.\n");
            Which::Real
        }
        _ => {
            println!("Invalid choice! Please enter 't' or 'r'.");
            return None;
        }
    };
    return Some(choice);
}

fn get_part_selection() -> Option<Part> {
    println!("Please enter 1 or 2 to choose which part to run");
    let mut part_string = String::new();
    io::stdin()
        .read_line(&mut part_string)
        .expect("Failed to read input");

    let part: Part = match part_string.trim() {
        "1" => Part::One,
        "2" => Part::Two,
        _ => {
            println!("Invalid selection");
            return None;
        }
    };
    return Some(part);
}
