//! The bit mask for part 2 is different

use std::collections::{HashMap, HashSet};

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
    pub fn iter(&self) -> Part2MaskIterator {
        Part2MaskIterator::new(&self.bits)
    }
}

/// This mask has only 0s and 1s in it
/// It's what you get after you process Wild bit Part2Mask's
#[derive(Debug, PartialEq, Eq)]
pub struct Part2MaskProcessed {
    /// All the bits that are turned on
    bits: HashSet<usize>,
}

impl Part2MaskProcessed {
    pub fn apply(&self, memory_address: usize) -> usize {
        let mut out = memory_address;
        for bit_offset in &self.bits {
            out |= 1 << bit_offset;
        }
        out
    }
}

/// Allows you to iterate every state of a wild mask
pub struct Part2MaskIterator {
    // The offset (in bits) of all the wild bits - sorted with smallest offset first
    wild_bits: Vec<usize>,
    // The offset (in bits) of all the normal (set to 1) bits
    normal_bits: HashSet<usize>,
    /// The maximum value we can get to having all wild bits set
    max: usize,
    /// Our current value
    state: usize,
}

impl Part2MaskIterator {
    fn new(bits: &HashMap<usize, BitValue>) -> Part2MaskIterator {
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
        let normal_bits: HashSet<usize> = bits
            .iter()
            .flat_map(|(offset, value)| {
                if *value == BitValue::On {
                    Some(*offset)
                } else {
                    None
                }
            })
            .collect();
        // Our maximum state will be;
        // eg. if there are 2 wild bits, to cover all values we need:
        // 00, 01, 10, 11 = 2 ^ 2 = 4 possible values for our state
        let max = 2usize.pow(wild_bits.len() as u32);
        Part2MaskIterator {
            wild_bits,
            normal_bits,
            max,
            state: 0,
        }
    }
}

impl Iterator for Part2MaskIterator {
    type Item = Part2MaskProcessed;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state == self.max {
            return None;
        }
        // Get all the bits that are always 1 anyway
        let mut bits: HashSet<usize> = self.normal_bits.clone();
        // Add our combination of wild bits
        if self.state > 0 {
            bits.extend(self.wild_bits.iter().enumerate().flat_map(|(idx, offset)| {
                if idx + 1 & self.state > 0 {
                    Some(*offset)
                } else {
                    None
                }
            }));
        }
        // Move on to the next state
        self.state += 1;
        Some(Part2MaskProcessed { bits })
    }
}

#[cfg(test)]
mod tests {

    use super::{Part2Mask, Part2MaskProcessed};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_mask_iterator() {
        let input = "mask = 0000000000000000000000000000000001XX";
        let input: Part2Mask = input.parse().unwrap();
        let got: Vec<Part2MaskProcessed> = input.iter().collect();
        let expected: Vec<Part2MaskProcessed> = vec![
            Part2MaskProcessed {
                // 100
                bits: vec![2].into_iter().collect(),
            },
            Part2MaskProcessed {
                // 101
                bits: vec![2, 0].into_iter().collect(),
            },
            Part2MaskProcessed {
                // 110
                bits: vec![2, 1].into_iter().collect(),
            },
            Part2MaskProcessed {
                // 101
                bits: vec![2, 0, 1].into_iter().collect(),
            },
        ]
        .into_iter()
        .collect();
        assert_eq!(got, expected);
    }
}
