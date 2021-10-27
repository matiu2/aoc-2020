use std::convert::TryFrom;

use nom::{error::Error, Err};

#[derive(Debug, PartialEq)]
pub enum Rule {
    // Looks like: "a"
    // Input must be 'a' for this to pass
    Simple(char),
    // Looks like: 1 3 | 3 1
    // Input must match rule 1, then the next char rule 3 ... or ... the other way around
    Chain(Vec<Vec<usize>>),
}

impl<'a> TryFrom<&'a str> for Rule {
    type Error = Err<Error<&'a str>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        crate::nom_parse::rule(value).map(|(_rest, output)| output)
    }
}
