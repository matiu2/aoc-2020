#[derive(Debug, PartialEq)]
pub enum Rule {
    // Looks like: "a"
    // Input must be 'a' for this to pass
    Simple(char),
    // Looks like: 1 3 | 3 1
    // Input must match rule 1, then the next char rule 3 ... or ... the other way around
    Chain(Vec<Vec<usize>>),
}
