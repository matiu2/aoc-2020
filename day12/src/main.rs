use std::fs::read_to_string;

use day12::Direction;

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    let directions: Vec<Direction> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut ship: day12::part1::State = Default::default();
    for direction in directions {
        ship.direction(direction)
    }
    println!("Day 12 - Part 1 - Distance: {}", ship.distance());
}
