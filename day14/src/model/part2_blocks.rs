//! Ties miltuple blocks together
mod parse;

use std::collections::HashMap;

use super::Part2Block;
#[derive(Debug, PartialEq, Eq)]
pub struct Part2Blocks {
    blocks: Vec<Part2Block>,
}

impl Part2Blocks {
    /// Execute all the instructions and write to `memory` (memory_location => value)
    /// Returns all the values written (location => value)
    pub fn write(&self) -> HashMap<usize, usize> {
        let mut memory = HashMap::new();
        for block in &self.blocks {
            block.write(&mut memory)
        }
        memory
    }
}

#[cfg(test)]
mod tests {

    use super::Part2Blocks;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_write() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let blocks: Part2Blocks = input.parse().unwrap();
        let mut got: Vec<(usize, usize)> = blocks.write().into_iter().collect();
        got.sort();
        let mut expected: Vec<(usize, usize)> = vec![
            (16, 1),
            (17, 1),
            (18, 1),
            (19, 1),
            (24, 1),
            (25, 1),
            (26, 1),
            (27, 1),
            // The below won't be there because first the 100 will be, written,
            // then 1 will overwrite it
            //(26, 100),
            // Also overwritten by 1
            //(27, 100),
            (58, 100),
            (59, 100),
        ];
        expected.sort();
        assert_eq!(got, expected);
    }
}
