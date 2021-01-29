use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Ticket {
    pub values: Vec<usize>,
}

impl Ticket {
    /// Only used to mock up tickets for testing
    pub fn parse(input: &str) -> IResult<&str, Ticket> {
        let number = map_res(digit1, |num: &str| num.parse::<usize>());
        let values = separated_list1(tag(","), number);
        let mut ticket = map(values, |values| Ticket { values });
        Ok(ticket(input)?)
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;

    #[test]
    fn test_parse_ticket() -> anyhow::Result<()> {
        let input = "7,1,14";
        let (input, ticket) = Ticket::parse(input)?;
        assert_eq!(input, "");
        assert_eq!(ticket.values, vec![7, 1, 14]);
        Ok(())
    }
}
