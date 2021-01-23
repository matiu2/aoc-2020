use anyhow::{anyhow, Error};
use std::str::FromStr;

use super::{Bit, BitMask};

impl FromStr for BitMask {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example line
        // mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        match parts.as_slice() {
            ["mask", "=", mask] => {
                let bits: Vec<Bit> = mask
                    .chars()
                    .rev()
                    .enumerate()
                    .flat_map(|(position, c)| match &c {
                        '0' => Some(Bit {
                            position,
                            value: false,
                        }),
                        '1' => Some(Bit {
                            position,
                            value: true,
                        }),
                        _ => None,
                    })
                    .collect();
                Ok(BitMask { bits })
            }
            _ => Err(anyhow!("Invalid mask line: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Bit, BitMask};

    #[test]
    fn test_parse() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let mask: BitMask = input.parse().unwrap();
        assert_eq!(
            &mask.bits,
            &vec![
                Bit {
                    position: 1,
                    value: false
                },
                Bit {
                    position: 6,
                    value: true
                }
            ]
        );
    }
}
