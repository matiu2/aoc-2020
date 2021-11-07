/// Handles a simple rule: does this character match with the next character
/// Returns the rest of the input if successful, or nothing if it fails
pub fn simple_rule(rule_char: char, input: &str) -> Option<&str> {
    input
        // For every unicode character (ie. decode utf-8)
        .chars()
        // Only take the first one
        .next()
        // Only take this first one if it matches the character in our rule
        .filter(|c2| *c2 == rule_char)
        // If it matches, return the rest of the input
        .map(|_| &input[1..])
}
