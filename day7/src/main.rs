use std::fs::read_to_string;

use day7::{find_containers, rule_parser};

fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input.txt");
    let map = rule_parser(&input);
    let query = ("shiny", "gold");
    let possibilities = find_containers(query, &map);
    println!(
        "Part 1: Possible containers for {} {}: {}",
        &query.0,
        &query.1,
        possibilities.len()
    );
}
