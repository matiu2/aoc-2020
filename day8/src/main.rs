use std::{
    collections::HashSet,
    fs::{read_link, read_to_string},
};

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

fn execute(instructions: &[Instruction]) -> i64 {
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

fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input.txt");
    let instructions = parse(&input);
    let answer = execute(&instructions);
    println!("Day 7 - Part 1 - Accumulator = {}", answer);
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
    fn test_execute() {
        let instructions = super::parse(input());
        let got = super::execute(&instructions);
        let expected = 5;
        assert_eq!(got, expected);
    }
}
