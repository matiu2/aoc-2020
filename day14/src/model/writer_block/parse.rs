use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};

use crate::model::{BitMask, Instruction};

use super::WriterBlock;

impl FromStr for WriterBlock {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let mask: BitMask = lines
            .first()
            .ok_or_else(|| anyhow!("No mask line!"))?
            .parse()
            .context("Reading mask")?;
        let writers: Result<Vec<Instruction>, Error> = lines
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
        let writers = writers?;
        Ok(WriterBlock { mask, writers })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{writer_block::WriterBlock, Bit, BitMask, Instruction};

    #[test]
    fn test_parse() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let got: WriterBlock = input.parse().unwrap();
        assert_eq!(
            got,
            WriterBlock {
                writers: vec![
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
                mask: BitMask::new(vec![Bit::new(1, false), Bit::new(6, true),]),
            }
        )
    }
}
