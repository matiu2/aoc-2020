use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input.txt").expect("unable to read file");
    let part1_answer = sum_anyone(&input);
    println!("Part 1 answer: {}", part1_answer);
    let part2_answer = sum_everyone(&input);
    println!("Part 2 answer: {}", part2_answer);
}

/// Takes an input text
///  * Each person's answers are on a line of their own.
///  * If a person answerd yes to question `a`, they'll have an `a` on their line
///    + Each question is represented by a letter of the alphabet
///  * Each group of people is separated by a blank line ("\n\n")
fn sum_anyone(input: &str) -> usize {
    input
        // Split into groups of people
        .split("\n\n")
        .map(|group| {
            let mut answers = HashSet::new();
            // Split the group into people
            group.lines().for_each(|person| {
                // Gather the answers for each person
                person.chars().for_each(|answer| {
                    answers.insert(answer);
                })
            });
            answers.len()
        })
        .sum()
}

/// See @sum_anyone for input description
/// Returns the sum of groups, where in each group we count the questions to which *everyone* answered yes
fn sum_everyone(input: &str) -> usize {
    input
        // Split into groups
        .split("\n\n")
        .map(|group| {
            // Store how many people answered yes to each question
            let mut answers = HashMap::new();
            // Count all people
            let mut all_people_count = 0;
            group
                // Split the group into pepole
                .lines()
                .inspect(|_| all_people_count += 1)
                .for_each(|person| {
                    person
                        // Get the questions that this person answered yes to
                        .chars()
                        // Increase the count of people that answered yes to this question
                        .for_each(|answer| *answers.entry(answer).or_insert(0) += 1);
                });
            // Take the people count that answered yes for each question
            answers
                .iter()
                // We only care about questions where everyone in the group answered yes
                .filter(|(_answer, &count)| count == all_people_count)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    fn input() -> &'static str {
        r#"abc

a
b
c

ab
ac

a
a
a
a

b"#
    }

    #[test]
    fn test_anyone_sum() {
        assert_eq!(11, super::sum_anyone(input()));
    }

    #[test]
    fn test_everyone_sum() {
        assert_eq!(6, super::sum_everyone(input()));
    }
}
