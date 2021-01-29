use std::collections::HashMap;

#[derive(Clone)]
pub struct NumberGenerator<'a> {
    input: &'a [usize],
    l2_cache: HashMap<usize, usize>,
    // The last number (from turn - 1)
    l1_cache: usize,
    // The current turn (starting at 1)
    turn: usize,
}

impl<'a> NumberGenerator<'a> {
    pub fn new(input: &'a [usize]) -> NumberGenerator {
        NumberGenerator {
            input,
            l2_cache: HashMap::new(),
            l1_cache: 0,
            turn: 1,
        }
    }
}

impl<'a> Iterator for NumberGenerator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.turn == 1 {
            // This is the first turn - nothing is cached yet
            // A special case
            if let Some(number) = self.input.first() {
                self.l1_cache = *number;
                self.turn += 1;
                Some(*number)
            } else {
                None
            }
        } else {
            self.do_next().map(|out| {
                // Promote all the caches
                self.l2_cache.insert(self.l1_cache, self.turn - 1);
                self.l1_cache = out;
                self.turn += 1;
                // Return the same number (not really a map sorry)
                out
            })
        }
    }
}

impl<'a> NumberGenerator<'a> {
    fn do_next(&self) -> Option<usize> {
        // In all other cases, we need to increment the turn and promote l1 cache to l2 cache
        // TurnEnder does that for us when it is dropped
        if self.turn <= self.input.len() {
            // We're still in the input list, just return the number
            self.input.get(self.turn - 1).cloned()
        } else {
            // Generate the next number
            // Check our l2_cache for the last number l1_cache
            // If the last number has been seen before,
            self.l2_cache
                .get(&self.l1_cache)
                // Return the difference between turn - 1 and the last time this number was seen
                .map(|last_turn| self.turn - 1 - last_turn)
                // If we've never seen it before, return 1
                .or(Some(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::NumberGenerator;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_number_generator_v1() {
        let input = vec![0, 3, 6];
        let generator = NumberGenerator::new(&input);
        // Check the first 10
        let got: Vec<usize> = generator.clone().take(10).collect();
        let expected = vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0];
        assert_eq!(got, expected);
        // Check number 2020
        let got = generator.skip(2019).next();
        let expected = Some(436);
        assert_eq!(got, expected);
    }
}
