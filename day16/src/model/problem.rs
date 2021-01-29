//! The whole problem to solve

use nom::{character::complete::newline, multi::separated_list1, sequence::tuple, IResult};

use super::{field::Field, tickets::Tickets};

pub struct Problem {
    pub fields: Vec<Field>,
    pub tickets: Tickets,
}

impl Problem {
    pub fn parse(input: &str) -> IResult<&str, Problem> {
        let fields = separated_list1(newline, Field::parse);
        let mut parse_all = tuple((fields, tuple((newline, newline)), Tickets::parse));
        let (input, (fields, _, tickets)) = parse_all(input)?;
        Ok((input, Problem { fields, tickets }))
    }
}

#[cfg(test)]
mod tests {
    use super::Problem;

    #[test]
    fn test_parse_problem() -> anyhow::Result<()> {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let (input, problem) = Problem::parse(input)?;
        assert_eq!(input, "");
        assert_eq!(problem.fields.len(), 3);
        Ok(())
    }
}
