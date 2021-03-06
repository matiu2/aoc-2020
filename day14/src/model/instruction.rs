mod parse;

/// An instruction to write a value to location in memory
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Instruction {
    pub location: usize,
    pub value: usize,
}
