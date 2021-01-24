use std::collections::HashMap;

use crate::model::WriterBlock;

mod parse;

#[derive(PartialEq, Eq, Debug)]
pub struct WriterBlocks {
    blocks: Vec<WriterBlock>,
}

impl WriterBlocks {
    /// Writes all the values from all the blocks
    /// Returns the values written as a HasmMap<memory_location, value>
    pub fn write(&self) -> HashMap<usize, usize> {
        let mut out = HashMap::new();
        self.blocks.iter().for_each(|block| block.write(&mut out));
        out
    }
}
