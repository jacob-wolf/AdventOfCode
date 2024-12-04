use env_file_reader;
use std::fs::read_to_string;
pub enum Which {
    Test,
    Real,
}
#[derive(Clone, Copy, Debug)]
pub enum Part {
    One,
    Two,
}
pub fn read_env_path() -> String {
    let env_variables = env_file_reader::read_file(".env").unwrap();
    let path = &env_variables["FILE_DATA_PATH"];
    path.clone()
}

pub fn read_file(problem: u32, choice: Which, part: Option<Part>) -> String {
    let part_str = match part {
        // since real input never changes always call it "1"
        // since test input sometimes changes let this value help here.
        Some(val) => match val {
            Part::One => "1",
            Part::Two => match choice {
                Which::Real => "1",
                Which::Test => "2",
            },
        },
        None => "1",
    };
    let problem_str = problem.to_string();
    let type_str = match choice {
        Which::Real => "real",
        Which::Test => "test",
    };
    let mut file_name = type_str.to_owned();
    file_name.push_str(&problem_str);
    file_name.push_str(&"_");
    file_name.push_str(&part_str);
    file_name.push_str(&".txt");

    let mut full_file_path = read_env_path();
    full_file_path.push_str(&file_name);
    println!("Reading data from: {full_file_path}");
    if let Ok(file_data) = read_to_string(full_file_path.clone()) {
        return file_data;
    } else {
        panic!("\n Failed to read file at: {full_file_path}")
    }
}
