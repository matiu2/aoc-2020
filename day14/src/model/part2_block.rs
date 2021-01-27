//! part two of the problem requires the mask to mask the address being written to, instead of the value being written.
//! Also any bits in the mask that are `x` become wild bits in the addresses being written, eg.
//! mask: X1 - means write to 01 and 11
//!  * 0 bits in the mask, mean to leave tha address bit unchanged
//!  * 1 bits in the mask => write a 1 in the address field
//!  * X bits in the mask => handle all combinations of this bit (1 and 0)

use std::collections::HashMap;
mod parse;

use crate::model::{Instruction, Part2Mask};

#[derive(PartialEq, Eq, Debug)]
pub struct Part2Block {
    mask: Part2Mask,
    instructions: Vec<Instruction>,
}

impl Part2Block {
    fn write_one(&self, location: usize, value: usize, memory: &mut HashMap<usize, usize>) {
        let addresses = self.mask.apply(location);
        for address in addresses {
            dbg!(address, value);
            memory.insert(address, value);
        }
    }

    /// Write all the memory changes we have
    pub fn write(&self, memory: &mut HashMap<usize, usize>) {
        for instruction in &self.instructions {
            dbg!(&instruction);
            self.write_one(instruction.location, instruction.value, memory);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Part2Block;

    #[test]
    fn test_write() -> anyhow::Result<()> {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100";
        let block: Part2Block = input.parse()?;
        let mut memory = HashMap::new();
        block.write(&mut memory);
        dbg!(&memory);
        Ok(())
    }
}
