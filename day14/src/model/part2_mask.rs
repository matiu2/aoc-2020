//! The bit mask for part 2 is different

use std::{collections::HashMap, fmt::Debug};

mod parse;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BitValue {
    // the corresponding memory address bit is overwritten with 1.
    On,
    // the corresponding memory address bit is floating.
    Wild,
}

/// A bitmask, but it's applied to memory locations
/// A wild bit `X` will write to many locations
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Part2Mask {
    /// All our bits, anything missing is just a 0
    bits: HashMap<usize, BitValue>,
}

impl Part2Mask {
    /// Only used so we can mock these up in tests. In production code, they'll only be created using &str::parse();
    #[cfg(test)]
    pub fn new(bits: HashMap<usize, BitValue>) -> Part2Mask {
        Part2Mask { bits }
    }

    /// Provide every real mask possible to be made from this mask
    /// by changing all of the `X` Wild bits into both a 0 and a 1
    pub fn apply(&self, value: usize) -> Part2MaskIterator {
        Part2MaskIterator::new(&self.bits, value)
    }
}

/// Allows you to iterate every state of a wild mask
pub struct Part2MaskIterator {
    // The offset (in bits) of all the wild bits - sorted with smallest offset first
    wild_bits: Vec<usize>,
    /// The maximum value we can get to having all wild bits set
    max: usize,
    /// Our current value
    state: usize,
    /// Our starting value (the | of value and the 1 bits)
    starting_value: usize,
}

impl Part2MaskIterator {
    fn new(bits: &HashMap<usize, BitValue>, value: usize) -> Part2MaskIterator {
        let mut wild_bits: Vec<usize> = bits
            .iter()
            .flat_map(|(offset, value)| {
                if *value == BitValue::Wild {
                    Some(*offset)
                } else {
                    None
                }
            })
            .collect();
        wild_bits.sort();
        // Apply the initial bits to the starting value straight away
        let mut starting_value = value;
        bits.iter()
            .flat_map(|(offset, value)| {
                if *value == BitValue::On {
                    Some(*offset)
                } else {
                    None
                }
            })
            .for_each(|offset| starting_value |= 1 << offset);
        // Our maximum state will be;
        // eg. if there are 2 wild bits, to cover all values we need:
        // 00, 01, 10, 11 = 2 ^ 2 = 4 possible values for our state
        let max = 2usize.pow(wild_bits.len() as u32);
        Part2MaskIterator {
            wild_bits,
            max,
            state: 0,
            starting_value,
        }
    }
}

impl Iterator for Part2MaskIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state == self.max {
            return None;
        }
        // Add our combination of wild bits
        let mut out = self.starting_value;
        self.wild_bits.iter().enumerate().for_each(|(idx, offset)| {
            if (1 << idx) & self.state > 0 {
                // Set this wild bit
                out |= 1 << *offset;
            } else {
                // Clear this wild bit
                out &= !(1 << *offset);
            }
        });
        // Move on to the next state
        self.state += 1;
        Some(out)
    }
}

#[cfg(test)]
mod tests {

    use super::Part2Mask;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_mask_apply() {
        let input = "mask = 000000000000000000000000000000X1001X";
        let input: Part2Mask = input.parse().unwrap();
        let got: Vec<usize> = input.apply(42).collect();
        let expected: Vec<usize> = vec![
            26, // 000000000000000000000000000000011010
            27, // 000000000000000000000000000000011011
            58, // 000000000000000000000000000000111010
            59, // 000000000000000000000000000000111011
        ]
        .into_iter()
        .collect();
        assert_eq!(got, expected);
    }
}
