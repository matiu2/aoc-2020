use std::convert::TryInto;
use std::fs::read_to_string;

mod logic;
pub mod model;
pub mod nom_parse;

fn main() {
    pretty_env_logger::init();
    let data = read_to_string("input.txt").unwrap();
    // Read until the first empty line to get the rules
    let rules: Vec<model::Rule> = data
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.try_into().unwrap())
        .collect();
    // Now count how many rules pass the test
    let count = data
        .lines()
        // Skip down to the second part of the input
        .skip_while(|line| !line.is_empty())
        // Ignore empty lines
        .filter(|line| !line.is_empty())
        // See if each line passes
        .filter(|input| logic::check_input(&rules, input))
        .count();
    println!("Day 19 part 1: {}", count);
}
