use anyhow::anyhow;
use day15::NumberGenerator;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let input: Result<Vec<usize>, _> = read_to_string("input.txt")?
        .split(",")
        .map(str::trim)
        .map(|line| line.parse())
        .collect();
    let input = input?;
    let generator = NumberGenerator::new(&input);
    let answer = generator
        .skip(2019)
        .next()
        .ok_or_else(|| anyhow!("Unable to find number 2020"))?;
    println!("Day 15 - Part 1 = {}", answer);
    Ok(())
}
