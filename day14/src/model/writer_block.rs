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
}

impl WriterBlock {
    /// Writes our values to `out`
    pub fn write(&self, out: &mut HashMap<usize, usize>) {
        for writer in &self.writers {
            out.insert(writer.location, self.mask.apply_to(writer.value));
        }
    }
}
