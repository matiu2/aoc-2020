//! A block with a mask and >= 1 writers
use std::collections::HashMap;

mod parse;

use super::{BitMask, MemWriter};

#[derive(Debug, PartialEq, Eq)]
pub struct WriterBlock {
    mask: BitMask,
    writers: Vec<MemWriter>,
}

impl WriterBlock {
    /// Writes our values to `out`
    pub fn write(&self, out: &mut HashMap<usize, usize>) {
        for writer in &self.writers {
            out.insert(writer.location, self.mask.apply_to(writer.value));
        }
    }
}
