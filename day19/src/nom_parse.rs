use std::collections::HashMap;

use crate::model::{Rule, RuleLogic};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, pair, terminated},
    IResult, Parser,
};

// Input example: '10: '
fn rule_number(input: &str) -> IResult<&str, usize> {
    terminated(number, tag(": "))(input)
}

/// Matches a character between two double quotes: eg. '"a"' -> 'a'
fn simple_char(input: &str) -> IResult<&str, char> {
    delimited(tag("\""), anychar, tag("\""))(input)
}

/// Matches any group of digits and returns it as a usize
/// eg. '1234' -> 1234
fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |digits: &str| digits.parse::<usize>())(input)
}

/// Grabs a single chain of rules
/// eg. '1 2 3' -> vec![1, 2, 3]
fn simple_chain(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(" "), number)(input)
}

/// Grabs optional chains
/// eg. '1 2 3 | 4 5 6' vec![vec![1, 2, 3], vec![4, 5, 6]]
fn chains(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(tag(" | "), simple_chain)(input)
}

/// Parses a whole rule including its number
/// eg. 10: "a" -> Rule{number: 10, RuleLogic::Simple('a')}
/// eg. 1: 1 4 | 4 5 -> Rule{number: 1, RuleLogic::Chain(vec![vec![1, 4], vec![4, 5]]) }
pub fn rule(input: &str) -> IResult<&str, Rule> {
    let char = simple_char.map(RuleLogic::Simple);
    let chains = chains.map(RuleLogic::Chain);
    let rule_logic = alt((char, chains));
    let (rest, (number, logic)) = pair(rule_number, rule_logic)(input)?;
    Ok((rest, Rule { number, logic }))
}

/// Parses a bunch of rules and returns their logic in order
pub fn rules<'a>(
    lines: impl Iterator<Item = &'a str>,
) -> Result<HashMap<usize, RuleLogic>, nom::Err<nom::error::Error<&'a str>>> {
    let rules: Result<Vec<Rule>, nom::Err<nom::error::Error<&str>>> = lines
        .map(|line| rule(line).map(|(_rest, rule)| rule))
        .collect();
    Ok(rules?
        .into_iter()
        .map(|rule| (rule.number, rule.logic))
        .collect())
}

#[cfg(test)]
mod test {
    use crate::model::{Rule, RuleLogic};

    #[test]
    fn test_parse_rule_number() {
        // Read a number
        let input = "12: ";
        let (rest, output) = super::rule_number(input).unwrap();
        assert_eq!(output, 12);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_rule_number_fail() {
        // Suppose the space is missing
        let input = "12:X";
        match super::rule_number(input) {
            Ok((rest, output)) => panic!(
                "It should have failed but instead: rest: {} output: {}",
                rest, output
            ),
            // Perfect it should have failed
            Err(_err) => (),
        };
    }

    #[test]
    fn test_read_char() {
        let input = "\"a\" rest";
        let (rest, output) = super::simple_char(input).unwrap();
        assert_eq!(rest, " rest");
        assert_eq!(output, 'a');
    }

    #[test]
    fn test_rule() {
        let input = r#"0: 1 2
1: "a"
3: "b"
2: 1 3 | 3 1"#;
        let out: Vec<Rule> = input
            .lines()
            .map(|line| {
                let (rest, out) = super::rule(line)
                    .unwrap_or_else(|err| panic!("Unable to parse line: '{}' - {:?}", line, err));
                assert!(
                    rest.is_empty(),
                    "There should be no leftovers after parsing line '{}', but I got '{}'",
                    line,
                    rest
                );
                out
            })
            .collect();
        assert_eq!(
            out,
            vec![
                Rule {
                    number: 0,
                    logic: RuleLogic::Chain(vec![vec![1, 2]])
                },
                Rule {
                    number: 1,
                    logic: RuleLogic::Simple('a')
                },
                Rule {
                    number: 3,
                    logic: RuleLogic::Simple('b')
                },
                Rule {
                    number: 2,
                    logic: RuleLogic::Chain(vec![vec![1, 3], vec![3, 1]])
                },
            ]
        );
    }

    #[test]
    fn test_advanced() {
        let rules = r#"0: 4 1 5
1: 2 3 | 3 2
3: 4 5 | 5 4
2: 4 4 | 5 5
4: "a"
5: "b""#;
        let rules: Vec<Rule> = rules
            .lines()
            .map(|line| super::rule(line).unwrap().1)
            .collect();
        assert_eq!(
            rules,
            vec![
                Rule {
                    number: 0,
                    logic: RuleLogic::Chain(vec![vec![4, 1, 5]])
                },
                Rule {
                    number: 1,
                    logic: RuleLogic::Chain(vec![vec![2, 3], vec![3, 2]])
                },
                Rule {
                    number: 3,
                    logic: RuleLogic::Chain(vec![vec![4, 5], vec![5, 4]])
                },
                Rule {
                    number: 2,
                    logic: RuleLogic::Chain(vec![vec![4, 4], vec![5, 5]])
                },
                Rule {
                    number: 4,
                    logic: RuleLogic::Simple('a')
                },
                Rule {
                    number: 5,
                    logic: RuleLogic::Simple('b')
                },
            ]
        );
    }
}
