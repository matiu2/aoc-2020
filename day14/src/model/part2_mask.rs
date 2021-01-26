//! The bit mask for part 2 is different

use std::collections::HashMap;
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
#[derive(Debug, Clone)]
pub struct Part2Mask {
    /// All our bits, anything missing is just a 0
    bits: HashMap<usize, BitValue>,
}

impl Part2Mask {
    /// Recursively clone masks until all the possible combinations of wild cards have been tried
    ///
    /// ## Arguments
    ///
    /// `location` is the memory location we're applying the mask to
    /// `mask` is the mask we ineherit from our parent caller
    pub fn apply_recursive(mask: &HashMap<usize, BitValue>, location: usize) -> Vec<usize> {
        if !mask.iter().any(|(_index, bit)| bit != &BitValue::Wild) {
            // If we have no more wild bits apply this mask to the value and return it
            let mut out = location;
            mask.iter().for_each(|(index, bit)| {
                assert_eq!(
                    bit,
                    &BitValue::On,
                    "There shouldn't be any wild bits any more"
                );
                out |= 1 << index;
            });
            vec![out]
        } else {
            let out = Vec::new();
            // Otherwise for each wild bit, recurse
            mask.iter()
                // Currently we only care about the wild bits
                .filter(|(_index, &bit)| bit == BitValue::Wild)
                .for_each(|(index, _bit)| {
                    let mut a = mask.clone();
                    let mut b = mask.clone();
                    // Recurse with the bit set to 1 (overwrite the wilde card in this copy)
                    a.insert(*index, BitValue::On);
                    b.remove(&index); // Basically a 0
                    let mut out = Vec::new();
                    out.extend(Part2Mask::apply_recursive(&a, location));
                    out.extend(Part2Mask::apply_recursive(&b, location));
                });
            out
        }
    }

    /// Apply the mask to the memory address (location)
    /// As there are wild bits, we may return more locations
    pub fn apply(&self, location: usize) -> Vec<usize> {
        Part2Mask::apply_recursive(&self.bits, location)
    }
}
