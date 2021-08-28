use parse_display::{Display, FromStr};
use std::fs::read_to_string;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{min}-{max} {letter}: {password}")]
struct Password {
    /// In part 1 - this is the minimum number of occurences for `letter`
    /// In part 2 - this is the first index to look for `letter`
    min: usize,
    /// In part 1 - this is the maximum number of occurences for `letter`
    /// In part 2 - this is the second index to look for `letter`
    max: usize,
    /// This is the letter used to validate the password
    letter: char,
    //// This is the password we will be validating
    password: String,
}

impl Password {
    /// Returns true if the password is valid
    /// (This is only for part 1)
    fn valid_part1(&self) -> bool {
        // Count the letter that we care about
        let count = self.password.chars().filter(|&c| c == self.letter).count();
        count >= self.min && count <= self.max
    }

    /// In part 2 min and max, actually become indexes (1 based index) a and b, and
    /// *exactly one* of those must hold the letter
    fn valid_part2(&self) -> bool {
        let count = self
            .password
            // Go through all the characters in the password
            .chars()
            // Give each character an index
            .enumerate()
            // The input is 1 based, but enumerate is 0 based, so add 1 to make `i` match the input
            .map(|(i, c)| (i + 1, c))
            // Only show us the characters at the indexes we care about
            .filter(|(i, _)| *i == self.min || *i == self.max)
            // If they match self.letter, collect it.
            .filter(|(_, c)| *c == self.letter)
            // If more than one matches, it's a fail
            .take(2)
            .count();
        count == 1
    }
}

fn main() -> anyhow::Result<()> {
    let passwords: Result<Vec<Password>, _> = read_to_string("input.txt")?
        .lines()
        .map(|line| line.parse())
        .collect();
    // Bail if any passwords are unreadable
    let passwords = passwords?;
    // Count the valid passwords
    let valid_count = passwords
        .iter()
        .filter(|password| password.valid_part1())
        .count();
    println!("Part 1: There are {} valid passwords", valid_count);

    // Part 2

    let valid_count = passwords
        .iter()
        .filter(|password| password.valid_part2())
        .count();
    println!("Part 2: There are {} valid passwords", valid_count);

    Ok(())
}
