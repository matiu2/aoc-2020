use std::{convert::TryFrom, fmt::Display};

use nom::{error::Error, Err};

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub number: usize,
    pub logic: RuleLogic,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RuleLogic {
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

impl Display for RuleLogic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleLogic::Simple(c) => write!(f, "\"{}\"", c),
            RuleLogic::Chain(chains) => {
                for chain in chains {
                    for rule in chain {
                        write!(f, "{} ", rule)?;
                    }
                    write!(f, " | ")?;
                }
                writeln!(f, "")
            }
        }
    }
}

impl RuleLogic {
    pub fn is_simple(&self) -> bool {
        if let RuleLogic::Simple(_) = self {
            true
        } else {
            false
        }
    }
}
