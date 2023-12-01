use std::{collections::HashMap, env};

use advent_of_code_2023::utils::*;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

static NUMERICS: [char; 9] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

fn main() {
    // let input = read_in_data("01.1");

    // let strings: Vec<String> = get_strings(input);

    // let sums = get_sum(&strings);

    // println!("sum 01.1: {}", sums);

    let input = read_in_data("01.2");

    let mut strings: Vec<String> = get_strings(input);

    strings = replace_named_numbers(&strings);

    strings = remove_non_numerics(&strings);

    let sums = get_sum(&strings);

    println!("sum 01.2: {}", sums);
}

fn get_strings(input: String) -> Vec<String> {
    let mut strings: Vec<String> = vec![];
    for value in input.split('\n') {
        strings.push(value.to_string());
    }
    strings
}

fn remove_non_numerics(strings: &Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for value in strings {
        result.push(value.replace(ASCII_LOWER, ""));
    }
    result
}

fn replace_named_numbers(strings: &Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for value in strings {
        
        let numeric_string = find_and_replace_first_numeric(value);
        let numeric_string = find_and_replace_last_numeric(&numeric_string);

        println!("{}", numeric_string);

        result.push(numeric_string);
    }

    result
}

fn get_sum(strings: &Vec<String>) -> i64 {
    let mut sums: i64 = 0;

    for value in strings {
        let start = value.chars().nth(0).unwrap();

        let end = value.chars().nth(value.len() - 1).unwrap();

        let summable = format!("{}{}", start, end).parse::<i64>().unwrap();

        sums += summable;
    }

    sums
}

fn find_and_replace_first_numeric(value: &String) -> String {
    
    for scan in 0..value.len() {
        let string_scan = value[0..(scan + 1)].to_string();
        let result_string = replace_numerics(&string_scan);

        if string_scan.len() > result_string.len() {
            return value.clone().replace(&string_scan, &result_string);
        }

        let numeric_check = string_scan.replace(NUMERICS, "");

        if string_scan.len() > numeric_check.len() {
            return value.clone();
        }

    }

    value.clone()
}

fn find_and_replace_last_numeric(value: &String) -> String {
    
    for scan in 0..value.len() {
        let string_scan = value[(value.len() - scan - 1)..(value.len())].to_string();
        let result_string = replace_numerics(&string_scan);

        if string_scan.len() > result_string.len() {
            return value.clone().replace(&string_scan, &result_string);
        }

    }

    value.clone()
}

fn replace_numerics(value: &String) -> String {
    let mut result = value.clone();

    result = result.replace("one", "1");
    result = result.replace("two", "2");
    result = result.replace("three", "3");
    result = result.replace("four", "4");
    result = result.replace("five", "5");
    result = result.replace("six", "6");
    result = result.replace("seven", "7");
    result = result.replace("eight", "8");
    result = result.replace("nine", "9");

    result
}
