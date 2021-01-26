use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};

use crate::model::{Instruction, Part2Mask};

use super::Part2Block;

impl FromStr for Part2Block {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let mask: Part2Mask = lines
            .first()
            .ok_or_else(|| anyhow!("No mask line!"))?
            .parse()
            .context("Reading mask")?;
        let instructions: Result<Vec<Instruction>, Error> = lines
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, line)| -> Result<Instruction, _> {
                line.parse().context(format!(
                    "Unable to parse Instruction on line {}: {}",
                    i, line
                ))
            })
            .collect();
        let instructions = instructions?;
        Ok(Part2Block { mask, instructions })
    }
}

#[cfg(test)]
mod tests {

    use crate::model::{BitValue, Instruction, Part2Block, Part2Mask};

    #[test]
    fn test_parse() {
        let input = "mask = 0000000000000000000000000000010000X0
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let got: Part2Block = input.parse().unwrap();
        assert_eq!(
            got,
            Part2Block {
                instructions: vec![
                    Instruction {
                        location: 8,
                        value: 11,
                    },
                    Instruction {
                        location: 7,
                        value: 101,
                    },
                    Instruction {
                        location: 8,
                        value: 0,
                    },
                ],
                mask: Part2Mask::new(
                    vec![(1usize, BitValue::Wild), (6, BitValue::On)]
                        .into_iter()
                        .collect()
                ),
            }
        )
    }
}
