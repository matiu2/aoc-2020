use anyhow::Result;
use itertools::iproduct;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let input: std::result::Result<Vec<usize>, _> =
        input.lines().map(|line| line.parse()).collect();
    let input = input?;

    // Part 1 - Find 2 numbers that add up to 2020
    let entries = iproduct!(&input, &input).find(|(&a, &b)| a + b == 2020);
    if let Some((a, b)) = entries {
        println!("Part 1: The entries are {} and {}", a, b);
        println!("Part 1: The answer is {}", a * b);
    } else {
        println!("Part 1: Answer not found");
    }

    // Part 2 - find three numbers that add up to 2020
    let entries = iproduct!(&input, &input, &input).find(|(&i, &j, &k)| i + j + k == 2020);

    if let Some((a, b, c)) = entries {
        println!("Part 2: The entries are {} and {} and {}", a, b, c);
        println!("Part 2: The answer is {}", a * b * c);
    } else {
        println!("Part 2: Answer not found");
    }

    Ok(())
}
