//! part two of the problem requires the mask to mask the address being written to, instead of the value being written.
//! Also any bits in the mask that are `x` become wild bits in the addresses being written, eg.
//! mask: X1 - means write to 01 and 11
//!  * 0 bits in the mask, mean to leave tha address bit unchanged
//!  * 1 bits in the mask => write a 1 in the address field
//!  * X bits in the mask => handle all combinations of this bit (1 and 0)

use std::collections::HashMap;

use crate::model::{LocationMask, MemWriter};

pub struct AddressWriter {
    mask: LocationMask,
    instructions: Vec<MemWriter>,
}

impl AddressWriter {
    fn write_one(&self, location: usize, value: usize, memory: &mut HashMap<usize, usize>) {
        for address in self.mask.apply(location) {
            memory.insert(address, value);
        }
    }

    /// Write all the memory changes we have
    pub fn write(&self, memory: &mut HashMap<usize, usize>) {
        for instruction in &self.instructions {
            self.write_one(instruction.location, instruction.value, memory);
        }
    }
}
