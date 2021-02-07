use std::fs::read_to_string;

use day17::model::Space;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("input.txt")?;
    let mut space = Space::parse(&input);
    for i in 0..6 {
        println!("Step {}:\n{}", i, &space);
        space = space.part1_cycle();
    }
    println!("Step {}:\n{}", 6, &space);
    println!("\nDay 17 - Part 1: {}", space.active_count());
    Ok(())
}
