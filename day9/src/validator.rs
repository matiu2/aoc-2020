//! Validates a list of numbers

use std::collections::VecDeque;

/// Returns the value of the first invalid number if any.
///
/// Each number after the `n`'th should be the sum of 2 numbers up to `n` back to be valid
///
/// ## Arguments
///
///  * input: the list of numbers to validate
///  * n: the number/length of the preamble
pub fn validate(input: &[usize], n: usize) -> Option<usize> {
    // Our ring buffer of previous `n` entries
    let mut preamble: VecDeque<usize> = input.iter().take(n).cloned().collect();
    input.iter().skip(n).cloned().find(|num| {
        // Check if this number is a combination of two entries in our preamble
        let is_valid = (0..n)
            // Go through every combination of entries
            .flat_map(|x| (0..n).map(move |y| (x, y)))
            // Exclude duplicate entries
            .filter(|(x, y)| x != y)
            // Get the sum of the two numbers from the preamble
            .map(|(x, y)| preamble[x] + preamble[y])
            // If any sum matches, the number is valid
            .any(|sum| sum == *num);
        // But we're looking for the first *invalid* number
        if is_valid {
            // Update preamble and continue
            preamble.pop_front();
            preamble.push_back(*num);
            false
        } else {
            // We found the bad number - exit
            true
        }
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_validate() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let expected = Some(127);
        let got = super::validate(&input, 5);
        assert_eq!(expected, got);
    }
}
