use std::collections::{hash_set, HashSet};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style = "snake_case")]
enum Instruction {
    #[display("acc {0}")]
    Acc(i64),
    Nop,
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
    let previous_instructions = HashSet::new();
    // Keep track of the accumulator
    let mut acc = 0;
    // Keep track of the instruction pointer
    let mut ip = 0;
    // loop until done
    while Some(instruction) = instructions.get(ip) {
        if previous_instructions.contains(ip) {
            break;
        }
        match ip {
            Instruction::Acc(num) => acc + num,
            Instruction::Jmp(num) => ip + num,
            Instruction::Nop(_) => ip + num,
        }
    }
    acc
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Instruction;

    #[test]
    fn test_parsing() {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
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
        let got = super::parse(input);
        assert_eq!(got, expected);
    }
}
