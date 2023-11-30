//https://adventofcode.com/2022/day/3

use std::collections::HashMap;

use advent_of_code_2023::utils::*;

fn main() {
    println!("2022-03");

    let mut rucksacks: HashMap<i32, RucksackItems> = HashMap::new();

    rucksacks.insert(00,RucksackItems("vJrwpWtwJgWrhcsFMMfFFhFp".to_string()));
    rucksacks.insert(01,RucksackItems("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()));
    rucksacks.insert(02,RucksackItems("PmmdzqPrVvPwwTWBwg".to_string()));
    rucksacks.insert(03,RucksackItems("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string()));
    rucksacks.insert(04,RucksackItems("ttgJtRGJQctTZtZT".to_string()));
    rucksacks.insert(05,RucksackItems("CrZsJsPPZsGzwwsLwLmpwMDw".to_string()));


    for (no, items) in rucksacks {
        let front_pocket = items.first_comparment();
        println!("{}: {}", no, front_pocket);
    }



}

struct RucksackItems(String);

impl RucksackItems {
    fn first_comparment(&self) -> String {
        let size = &self.0.len();
        self.0[0..size/2].to_string()
    }
}
