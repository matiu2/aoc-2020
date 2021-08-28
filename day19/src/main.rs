#[derive(PartialEq, Eq, Debug)]
pub enum Rule {
    /// A Simple rule: the letter referenced must be `char`
    Simple(char),
    /// Choice of chains: The letter sequence must match one of the chains
    Choice(Vec<Vec<u32>>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct NumberedRule {
    number: u32,
    rule: Rule,
}

pub mod parse;

/// Enumerated list of rule
struct Rules {
    rules: Vec<Rule>,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{parse::numbered_rule, NumberedRule, Rule};

    #[test]
    fn test_parsing() {
        let input = "0: \"a\"";
        let expected = NumberedRule {
            number: 0,
            rule: Rule::Simple('a'),
        };
        let (rest, got) = numbered_rule(input).unwrap();
        assert!(rest.is_empty());
        assert_eq!(got, expected);
    }
}
