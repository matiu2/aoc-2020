use anyhow::Error;
use std::str::FromStr;

use super::WriterBlocks;

impl FromStr for WriterBlocks {
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
    use crate::model::{Bit, BitMask, MemWriter, WriterBlock, WriterBlocks};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX00
mem[8] = 11
mem[7] = 123
mem[8] = 456";
        let got: WriterBlocks = input.parse().unwrap();
        let expected = WriterBlocks {
            blocks: vec![
                WriterBlock::new(
                    BitMask::new(vec![Bit::new(1, false), Bit::new(6, true)]),
                    vec![
                        MemWriter {
                            location: 8,
                            value: 11,
                        },
                        MemWriter {
                            location: 7,
                            value: 101,
                        },
                        MemWriter {
                            location: 8,
                            value: 0,
                        },
                    ],
                ),
                WriterBlock::new(
                    BitMask::new(vec![
                        Bit::new(0, false),
                        Bit::new(1, false),
                        Bit::new(6, true),
                    ]),
                    vec![
                        MemWriter {
                            location: 8,
                            value: 11,
                        },
                        MemWriter {
                            location: 7,
                            value: 123,
                        },
                        MemWriter {
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
