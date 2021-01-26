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
    /// Only used so we can create some mockups for testing
    /// To actuall generate one of these just parse the input
    #[cfg(test)]
    pub fn new(position: usize, value: bool) -> Bit {
        Bit { position, value }
    }
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

#[derive(Default, Debug, PartialEq, Eq)]
pub struct BitMask {
    bits: Vec<Bit>,
}

impl BitMask {
    /// Only used in testing to make mockups
    #[cfg(test)]
    pub fn new(bits: Vec<Bit>) -> BitMask {
        BitMask { bits }
    }
    /// Applies the bit mask to `input` and returns the new value
    pub fn apply_to(&self, mut input: usize) -> usize {
        self.bits.iter().for_each(|bit| bit.apply(&mut input));
        input
    }
}
