use crate::model::Rule;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{map_res, recognize},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};

// Get the rule number at the start of a line (we later just throw it away anyway)
// Input example: '10: '
fn rule_number(input: &str) -> IResult<&str, &str> {
    recognize(digit1.and(tag(": ")))(input)
}

fn simple_char(input: &str) -> IResult<&str, char> {
    delimited(tag("\""), anychar, tag("\""))(input)
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |digits: &str| digits.parse::<usize>())(input)
}

fn simple_chain(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(" "), number)(input)
}

fn chains(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(tag(" | "), simple_chain)(input)
}

pub fn rule(input: &str) -> IResult<&str, Rule> {
    let char = simple_char.map(|c| Rule::Simple(c));
    let chains = chains.map(|nums| Rule::Chain(nums));
    let rule = alt((char, chains));
    preceded(rule_number, rule)(input)
}

#[cfg(test)]
mod test {
    use crate::model::Rule;

    #[test]
    fn test_parse_rule_number() -> anyhow::Result<()> {
        // Read a number
        let input = "12: ";
        let (rest, output) = super::rule_number(input)?;
        assert_eq!(output, input);
        assert_eq!(rest, "");
        Ok(())
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
2: 1 3 | 3 1
3: "b""#;
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
                Rule::Chain(vec![vec![1, 2]]),
                Rule::Simple('a'),
                Rule::Chain(vec![vec![1, 3], vec![3, 1]]),
                Rule::Simple('b')
            ]
        );
    }
}
