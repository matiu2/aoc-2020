use anyhow::{anyhow, Error};
use std::{collections::HashMap, str::FromStr};

use super::{BitValue, Part2Mask};

impl FromStr for Part2Mask {
    type Err = Error;

    /// Parses the location mask
    ///
    /// ```
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        match parts.as_slice() {
            ["mask", "=", mask] => {
                let bits: HashMap<usize, BitValue> = mask
                    .chars()
                    .rev()
                    .enumerate()
                    .flat_map(|(position, c)| match &c {
                        '0' => None,
                        '1' => Some((position, BitValue::On)),
                        _ => Some((position, BitValue::Wild)),
                    })
                    .collect();
                Ok(Part2Mask { bits })
            }
            _ => Err(anyhow!("Invalid mask line: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BitValue, Part2Mask};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        let mask: Part2Mask = "mask = 0X1X0011011111X1X000X11X11000X01XX11".parse()?;
        // The left most bit (35) is none
        assert_eq!(mask.bits.get(&35), None);
        // The 2nd from left is `X` wild
        assert_eq!(mask.bits.get(&34), Some(&BitValue::Wild));
        // The right most bit is `1` Set
        assert_eq!(mask.bits.get(&0), Some(&BitValue::On));
        Ok(())
    }
}
