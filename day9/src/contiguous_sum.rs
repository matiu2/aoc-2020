/// Searches through `input` to find a contiguous list of numbers that add up to `query`
/// Returns the *sum* of the *smallest* (min) and *largest* (max) of that contiguous list of numbers
/// If no such range is found, returns None
pub fn contiguous_sum(input: &[usize], query: usize) -> Option<usize> {
    (0..input.len())
        .flat_map(|start| {
            let mut sum = 0;
            let mut end = start;
            while let Some(num) = input.get(end) {
                sum += num;
                // We found our contiguous range, return the start and end of the range
                if sum == query {
                    return Some(start..end + 1);
                } else if sum > query {
                    // We went over the sum, this index doesn't work
                    return None;
                } else {
                    // Make the range a bit bigger
                    end += 1;
                }
            }
            // No ranges got >= query
            None
        })
        .flat_map(|range| {
            let min = input[range.clone()].iter().min().cloned();
            let max = input[range].iter().max().cloned();
            min.zip(max).map(|(a, b)| a + b)
        })
        .next()
}
