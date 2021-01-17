use std::{collections::HashSet, fs::read_to_string};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style = "snake_case")]
enum Instruction {
    #[display("acc {0}")]
    Acc(i64),
    #[display("nop {0}")]
    Nop(i64),
    #[display("jmp {0}")]
    Jmp(i64),
}

impl Instruction {
    /// Returns true if this is a jmp instruction
    fn is_jmp(&self) -> bool {
        if let Instruction::Jmp(_) = self {
            true
        } else {
            false
        }
    }
    /// Flips a jmp instruction to a nop instruction
    fn flip_to_nop(&mut self) {
        if let Instruction::Jmp(val) = *self {
            *self = Instruction::Nop(val)
        }
    }
    /// Flips a nop instruction to a jmp instruction
    fn flip_to_jmp(&mut self) {
        if let Instruction::Nop(val) = *self {
            *self = Instruction::Jmp(val)
        }
    }
}

/// Parses input rules (one per line) and returns the instructions, or panics
fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|i| -> Instruction {
            i.parse()
                .expect(&format!("Unable to parse instruction: {}", i))
        })
        .collect()
}

/// Executes the program, but stops as soon as we hit the same instruction twice
fn execute_part1(instructions: &[Instruction]) -> i64 {
    // Keep track of previous instructions
    let mut previous_instructions = HashSet::new();
    // Keep track of the accumulator
    let mut acc = 0;
    // Keep track of the instruction pointer
    let mut ip = 0i64;
    // loop until done
    while let Some(instruction) = instructions.get(ip as usize) {
        if previous_instructions.contains(&ip) {
            break;
        }
        previous_instructions.insert(ip);
        match instruction {
            Instruction::Acc(num) => {
                acc += num;
                ip += 1;
            }
            Instruction::Jmp(num) => {
                ip += num;
            }
            Instruction::Nop(_) => {
                ip += 1;
            }
        };
    }
    acc
}

/// Executes until the program is done.
/// If it gets to the end of the program it returs the accumulator value
/// If it infinite loops, it returns None
fn execute_part2(instructions: &[Instruction]) -> Option<i64> {
    // Keep track of previous instructions
    let mut previous_instructions = HashSet::new();
    // Keep track of the accumulator
    let mut acc = 0;
    // Keep track of the instruction pointer
    let mut ip = 0i64;
    // loop until done
    while let Some(instruction) = instructions.get(ip as usize) {
        if previous_instructions.contains(&ip) {
            return None;
        }
        previous_instructions.insert(ip);
        match instruction {
            Instruction::Acc(num) => {
                acc += num;
                ip += 1;
            }
            Instruction::Jmp(num) => {
                ip += num;
            }
            Instruction::Nop(_) => {
                ip += 1;
            }
        };
    }
    Some(acc)
}

fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input.txt");
    let mut instructions = parse(&input);
    let answer = execute_part1(&instructions);
    println!("Day 8 - Part 1 - Accumulator = {}", answer);

    // Part 2
    // Find all the jump statement locations
    let jump_indexes: Vec<usize> = instructions
        .iter()
        .enumerate()
        .filter(|(_i, instruction)| instruction.is_jmp())
        .map(|(i, _instruction)| i)
        .collect();
    // Try flipping each jmp location to a nop, until we find one that returns the accumulator
    let answer = jump_indexes
        .iter()
        .flat_map(|index| {
            instructions[*index].flip_to_nop();
            let out = execute_part2(&instructions);
            instructions[*index].flip_to_jmp();
            out
        })
        .next()
        .expect("Unable to find a non-infite loop program");
    println!("Day 8 - Part 2 - Accumulator = {}", answer);
}

#[cfg(test)]
mod tests {
    use super::Instruction;

    fn input() -> &'static str {
        r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#
    }

    #[test]
    fn test_parsing() {
        let expected = vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jmp(-4),
            Instruction::Acc(6),
        ];
        let got = super::parse(input());
        assert_eq!(got, expected);
    }

    #[test]
    fn test_execute_part1() {
        let instructions = super::parse(input());
        let got = super::execute_part1(&instructions);
        let expected = 5;
        assert_eq!(got, expected);
    }

    #[test]
    fn test_execute_part2() {
        let mut instructions = super::parse(input());
        let penultimate_index = instructions.len() - 2;
        instructions[penultimate_index] = Instruction::Nop(0);
        let got = super::execute_part2(&instructions);
        let expected = 8;
        assert_eq!(got, expected);
    }
}
