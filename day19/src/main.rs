use std::str::FromStr;

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

mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::{anychar, digit1},
        combinator::map_res,
        multi::{separated_list0, separated_list1},
        sequence::{delimited, pair, terminated, tuple},
        IResult,
    };

    use crate::{NumberedRule, Rule};

    fn number(input: &str) -> IResult<&str, u32> {
        map_res(digit1, |digits: &str| digits.parse::<u32>())(input)
    }

    /// Parses a rule number:
    /// eg. 1:
    fn rule_number(input: &str) -> IResult<&str, u32> {
        terminated(number, tag(": "))(input)
    }

    fn simple_rule(input: &str) -> IResult<&str, Rule> {
        use nom::character::complete::char;
        let (rest, c) = delimited(char('"'), anychar, char('"'))(input)?;
        Ok((rest, Rule::Simple(c)))
    }

    fn chain(input: &str) -> IResult<&str, Vec<u32>> {
        use nom::character::complete::char;
        separated_list1(char(' '), number)(input)
    }

    fn choice(input: &str) -> IResult<&str, Rule> {
        let (rest, chains) = separated_list0(tag(" | "), chain)(input)?;
        Ok((rest, Rule::Choice(chains)))
    }

    fn rule(input: &str) -> IResult<&str, super::Rule> {
        use nom::Parser;
        simple_rule.or(choice).parse(input)
    }

    pub fn numbered_rule(input: &str) -> IResult<&str, NumberedRule> {
        let (rest, (number, rule)) = pair(rule_number, rule)(input)?;
        Ok((rest, NumberedRule { number, rule }))
    }
}

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
