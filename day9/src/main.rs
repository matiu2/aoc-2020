//! # Problem:
//!
//! ## Input
//!
//!  * preamble length: eg. 5
//!  * list of numbers: eg. 5
//!
//! ## Goal
//!
//! Find the first invalid number
//!
//! ## Rules
//!
//!  * All numbers in the preamble are valid
//!  * Any subsequent number must be the sum of 2 numbers `preamble length` in the past
//!    + eg. The 20th number, with a preamble length of 5, must be the sum of any two numbers in the range 14-19

use std::fs::read_to_string;

use day9::{contiguous_sum, parse, validate};

fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input.txt");
    let input = parse(&input);
    let part_1 = validate(&input, 25);
    if let Some(part_1) = part_1 {
        println!("Day 9 - Part 1: {:?}", part_1);
        // Part 2
        let part_2 = contiguous_sum(&input, part_1);
        println!(
            "Day 9 - Part 2: {}",
            part_2
                .map(|num| format!("{}", num))
                .unwrap_or_else(|| "No answer found".to_string())
        );
    } else {
        println!("No answers found")
    }
}
