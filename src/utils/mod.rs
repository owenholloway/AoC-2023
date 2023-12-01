use std::{env, fs};

pub fn test() {
    println!("me hi");
}

pub fn read_in_data(day: &str) -> String {
    let path = format!(
        "{}/day_{}",
        env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .replace("bin", "inputs"),
        day
    );
    fs::read_to_string(path.clone()).expect(&format!("File not found at {}", path))
}
