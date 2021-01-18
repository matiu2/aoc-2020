/// Takes a step towards solving a combination. Recursive function
///
/// # Arguments
///
///  * adapters: The list of adapters we have available, sorted by joltage
///  * current_output: The output coming out of the socket (with all our adapters right now)
///  * target_output: The output we hope to reach to power our laptop
///
/// Returns the number of combinations found that add up to the target_output
fn step(adapters: &[usize], current_output: usize, target_output: usize) -> usize {
    if current_output == target_output {
        // We've reached a valid combination, exit, and count it
        // We've recursed down to the bottom
        1
    } else {
        let start = adapters
            // Start search where the first adapter should be
            // NOTE: If there are multiple adapters with the same joltage, it will
            //       choose an random one, but we don't expect that in the adapters
            .binary_search(&(current_output + 1))
            // If it can't find the exact place, it'll give us the place where it would be inserted
            .unwrap_or_else(|pos| pos);
        adapters[start..]
            .iter()
            .cloned()
            .take_while(|adapter_output| *adapter_output - current_output <= 3)
            .map(|next_step| step(&adapters, next_step, target_output))
            .sum()
    }
}

/// Returns the number of possible combinations that you can connect your
/// adapters to reach the target joltage
pub fn find_combinations(mut adapters: Vec<usize>) -> usize {
    adapters.sort();
    // We don't add the +3 because we know if we reach the maximum adapter's output we've made a combination
    let target = *adapters.last().unwrap_or(&0);
    // Count the combinations
    step(&adapters, 0, target)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_easy() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let got = super::find_combinations(adapters);
        let expected = 8;
        assert_eq!(expected, got)
    }

    #[test]
    fn test_hard() {
        let adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let got = super::find_combinations(adapters);
        let expected = 19208;
        assert_eq!(expected, got)
    }
}
