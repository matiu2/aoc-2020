//! A block with a mask and >= 1 writers
use std::{collections::HashMap, fmt::Write};

mod parse;

use super::{BitMask, MemWriter, WriterBlocks};

#[derive(Debug, PartialEq, Eq)]
pub struct WriterBlock {
    mask: BitMask,
    writers: Vec<MemWriter>,
}

impl WriterBlock {
    /// A test only enabled test function so we can mock up blocks to test with
    #[cfg(test)]
    pub fn new(mask: BitMask, writers: Vec<MemWriter>) -> WriterBlock {
        WriterBlock { mask, writers }
    }

    /// Writes our values to `out`
    pub fn write(&self, out: &mut HashMap<usize, usize>) {
        for writer in &self.writers {
            out.insert(writer.location, self.mask.apply_to(writer.value));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::WriterBlock;

    #[test]
    fn test_write() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let block: WriterBlock = input.parse().unwrap();
        let mut values = HashMap::new();
        block.write(&mut values);
        assert_eq!(values.get(&8), Some(&64));
        assert_eq!(values.get(&7), Some(&101));
        assert_eq!(values.len(), 2);
    }
}
