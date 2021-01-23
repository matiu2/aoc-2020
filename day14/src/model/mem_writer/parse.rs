use anyhow::{anyhow, Context, Error};
use std::str::FromStr;

use super::MemWriter;

impl FromStr for super::MemWriter {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example string: mem[8] = 11
        let parts: Vec<&str> = s.split(|c| c == ' ' || c == '[' || c == ']').collect();
        match parts.as_slice() {
            ["mem", location, "", "=", value] => {
                let location = location.parse().context("Bad Location")?;
                let value = value.parse().context("bad value")?;
                Ok(MemWriter { location, value })
            }
            other => Err(anyhow!("Unable to parse {}. Got {:?}", s, other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::MemWriter;

    #[test]
    fn test_parse() {
        let input = "mem[8] = 11";
        let got: MemWriter = input.parse().unwrap();
        assert_eq!(
            got,
            MemWriter {
                location: 8,
                value: 11
            }
        );
    }
}
