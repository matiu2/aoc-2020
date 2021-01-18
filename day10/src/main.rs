/*!
# Problem

 * You have a power outlet outputting 0 jolts
 * You have a bunch of power adapters
   + Each one can take in n-3..n jolts, where n is its output joltage
 * Your target joltage is n+3 - where n is your largest adaptor
 * You must figure out how to use all your adapters to get to the target joltage

*/

use std::fs::read_to_string;

use day10::find_combinations;

fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input.txt");
    let input: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    let answer = find_joltage(input.clone()).unwrap();
    println!("Day 10 - Part 1: {}", answer);

    // Part 2 - count combinations
    let combinations = find_combinations(input);
    println!("Day 10 - Part 2: {}", combinations);
}

/// Finds the answer to part1
/// The number of 1 volt step-ups * the number of 3 volt step-ups
/// Returns the number of 1 jolt jumps multiplied by the number of 3 jolt jumps
fn find_joltage(mut input: Vec<usize>) -> Option<usize> {
    input.sort();
    let mut current = 0;
    let target = *input.last()? + 3;
    // Number of 1 volt jumps
    let mut jumps_1 = 0;
    // Number of 3 volt jumps
    let mut jumps_3 = 0;
    // This is where we'll end up, so rather than doing an extra iteration at
    // the end of the loop, just throw it on now.
    input.push(target);
    input.into_iter().for_each(|jolt| {
        // How big is the step ?
        let jump_size = jolt - current;
        dbg!(current, jolt, jump_size, target);
        // Store the new current joltage
        current = jolt;
        // Handle the jump size
        match jump_size {
            1 => jumps_1 += 1,
            2 => (),
            3 => jumps_3 += 1,
            _ => panic!("Jump size too big! {} {}", current, jump_size),
        };
    });
    assert_eq!(current, target);
    Some(jumps_1 * jumps_3)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_easy() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let target_voltage = input.iter().max().unwrap() + 3;
        assert_eq!(22, target_voltage);
        let got = super::find_joltage(input).unwrap();
        let expected = 7 * 5;
        assert_eq!(expected, got);
    }

    #[test]
    fn test_longer() {
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let got = super::find_joltage(input).unwrap();
        let expected = 22 * 10;
        assert_eq!(expected, got);
    }
}
