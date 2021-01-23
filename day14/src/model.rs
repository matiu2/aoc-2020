mod parse;

/// A bit in the bit modifier
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bit {
    /// Its position / index in the bitmask
    /// 0 means the least significant bit
    /// Max will be 36
    position: usize,
    /// Whether this bit should be turned on
    value: bool,
}

impl Bit {
    fn apply(self, input: &mut usize) {
        if self.value {
            // Set the bit
            *input |= 1 << self.position;
        } else {
            // Clear the bit
            *input &= !(1 << self.position);
        }
    }
}

#[derive(Default)]
pub struct BitMask {
    bits: Vec<Bit>,
}

impl BitMask {
    pub fn apply(self, input: &mut usize) {
        self.bits.iter().for_each(|bit| bit.apply(input));
    }
}
