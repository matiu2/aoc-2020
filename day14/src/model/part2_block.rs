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
    /// Only used so we can build these in tests
    #[cfg(test)]
    pub fn new(mask: Part2Mask, instructions: Vec<Instruction>) -> Part2Block {
        Part2Block { mask, instructions }
    }
    /// Write all the memory changes we have
    pub fn write(&self, memory: &mut HashMap<usize, usize>) {
        self.instructions
            .iter()
            .flat_map(|instruction| {
                // For every possible real mask in our mask, apply it to the location fields
                self.mask
                    .iter()
                    .map(move |mask| (mask.apply(instruction.location), instruction.value))
            })
            // Execute each instruction and write the masked values to memory
            .for_each(|(location, value)| {
                memory.insert(location, value);
            });
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    use super::Part2Block;

    #[test]
    fn test_write() -> anyhow::Result<()> {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mem[64] = 20
";
        // 42 in binary: 0101010
        // 64 in binary: 1000000
        // Base Mask: X1001X
        // Masks generated (and their destination addresses)
        // 010010 & 0101010 = 58
        // 010011 & 0101010 = 59
        // 110010 & 0101010 = 58
        // 110011 & 0101010 = 59
        // 010010 & 1000000 = 82
        // 010011 & 1000000 = 83
        // 110010 & 1000000 = 114
        // 110011 & 1000000 = 115
        let block: Part2Block = input.parse()?;
        let mut memory = HashMap::new();
        block.write(&mut memory);
        let expected: HashMap<usize, usize> = vec![
            (58, 100),
            (59, 100),
            (82, 20),
            (83, 20),
            (114, 20),
            (115, 20),
        ]
        .into_iter()
        .collect();
        assert_eq!(memory, expected);
        Ok(())
    }
}
