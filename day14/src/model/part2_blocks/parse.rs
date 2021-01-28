use anyhow::Error;
use std::str::FromStr;

use super::Part2Blocks;

impl FromStr for Part2Blocks {
    type Err = Error;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut blocks = Vec::new();
        while let Some(start) = s.find("mask") {
            let end = match s[1..].find("mask") {
                Some(end) => end + 1,
                None => s.len(),
            };
            let tmp = &s[start..end];
            blocks.push(tmp.parse()?);
            s = &s[end..]
        }
        Ok(Self { blocks })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{BitValue, Instruction, Part2Block, Part2Blocks, Part2Mask};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let input = "mask = 0000000000000000000000000000010000X0
mem[8] = 11
mem[7] = 101
mem[8] = 0
mask = 0000000000000000000000000000010000XX
mem[8] = 11
mem[7] = 123
mem[8] = 456";
        let got: Part2Blocks = input.parse().unwrap();
        let expected = Part2Blocks {
            blocks: vec![
                Part2Block::new(
                    Part2Mask::new(
                        vec![(1, BitValue::Wild), (6, BitValue::On)]
                            .into_iter()
                            .collect(),
                    ),
                    vec![
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
                ),
                Part2Block::new(
                    Part2Mask::new(
                        vec![(0, BitValue::Wild), (1, BitValue::Wild), (6, BitValue::On)]
                            .into_iter()
                            .collect(),
                    ),
                    vec![
                        Instruction {
                            location: 8,
                            value: 11,
                        },
                        Instruction {
                            location: 7,
                            value: 123,
                        },
                        Instruction {
                            location: 8,
                            value: 456,
                        },
                    ],
                ),
            ],
        };
        assert_eq!(got, expected);
    }
}
