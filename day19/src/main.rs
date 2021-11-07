use std::fs::read_to_string;

use crate::{logic::check_input, model::RuleLogic};

mod logic;
pub mod model;
pub mod nom_parse;

fn main() {
    pretty_env_logger::init();
    let data = read_to_string("input.txt").unwrap();
    // Read until the first empty line to get the rules
    let rules = data.lines().take_while(|line| !line.is_empty());
    let mut rules = nom_parse::rules(rules).unwrap();
    // Now count how many rules pass the test
    let input: Vec<&str> = data
        .lines()
        // Skip down to the second part of the input
        .skip_while(|line| !line.is_empty())
        // Ignore empty lines
        .filter(|line| !line.is_empty())
        .collect();
    let count = input
        .iter()
        // See if each line passes
        .filter(|input| check_input(&rules, input))
        .count();
    println!("Day 19 part 1: {}", count);

    // Part 2:
    rules.insert(8, RuleLogic::Chain(vec![vec![42], vec![42, 8]]));
    rules.insert(11, RuleLogic::Chain(vec![vec![42, 31], vec![42, 11, 31]]));
    let count = input
        .iter()
        // See if each line passes
        .filter(|input| check_input(&rules, input))
        .count();
    println!("Day 19 part 2: {}", count);
}
