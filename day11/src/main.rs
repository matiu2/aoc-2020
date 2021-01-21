/*!
# Problem

It's like conway's game of life, but with waiting room seats.

Input example:

    L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL

 * L = a seat
 * . = floor
 * # = An occupied seat

Each turn:

 * If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
 * If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
 * Otherwise, the seat's state does not change.
 * Floor (.) never changes

Keep running turns / rounds until the state stabalizes. How many seats end up occupied ?
*/

use std::fs::read_to_string;

use day11::Spaces;

fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input.txt");
    let mut spaces: Spaces = input.parse().expect("Unable to read input");
    let mut next_step = spaces.step();
    while spaces != next_step {
        spaces = next_step;
        next_step = spaces.step();
    }
    // Count the occupied seats
    println!(
        "Day 11 - Part 1 - Occupied seats: {}",
        spaces.count_occupied()
    );
    // Part 2 - Using line if sight technique
    spaces = input.parse().unwrap();
    let mut next_step = spaces.step_part2();
    while spaces != next_step {
        spaces = next_step;
        next_step = spaces.step_part2();
    }
    // Count the occupied seats
    println!(
        "Day 11 - Part 2 - Occupied seats: {}",
        spaces.count_occupied()
    );
}
