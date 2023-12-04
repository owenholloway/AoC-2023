use std::{env, fs};

pub static NUMERICS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub static SYMBOLS: [char; 13] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '=', '+',
];

pub fn test() {
    println!("me hi");
}

pub fn read_in_data(day: &str) -> String {
    let path = format!(
        "{}/src/inputs/day_{}",
        env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .replace("/src", "")
            .replace("/bin", ""),
        day
    );
    fs::read_to_string(path.clone()).expect(&format!("File not found at {}", path))
}

pub fn lines_to_vector(input: String) -> Vec<String> {
    let mut strings: Vec<String> = vec![];
    for value in input.split('\n') {
        strings.push(value.to_string());
    }
    strings
}

pub fn lines_to_single(input: String) -> String {
    let mut result = String::new();
    for value in input.split('\n') {
        result = result + value;
    }
    result
}
