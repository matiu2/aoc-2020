use std::fs::read_to_string;

use day14::model::{Part2Blocks, WriterBlocks};

fn main() -> anyhow::Result<()> {
    let input = read_to_string("input.txt")?;
    let blocks: WriterBlocks = input.parse()?;
    let values = blocks.write();
    let answer: usize = values.values().sum();
    println!("Day 14 - Part 1 - answer: {}", answer);
    // Part 2
    let blocks: Part2Blocks = input.parse()?;
    let values = blocks.write();
    let answer: usize = values.values().sum();
    println!("Day 14 - Part 2 - answer: {}", answer);
    Ok(())
}
