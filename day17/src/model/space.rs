use std::collections::HashSet;

use limits::Limits;

mod limits;
mod parse;

#[derive(Debug, PartialEq, Eq)]
pub struct Space {
    // All the active blocks that we know about.
    // Anything not in here is an inactive block
    active_blocks: HashSet<(i64, i64, i64)>,
}

impl Space {
    /// Runs a part 1 cycle and returns a new state
    fn part1_cycle(&self) -> Space {
        // Find our limits
        // For piece of space
        todo!()
    }

    /// Iterate over every part of possibly affected space
    pub fn limits(&self) -> Limits {
        Limits::new(self).unwrap_or_default()
    }
}
