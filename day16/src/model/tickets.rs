use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list0, sequence::tuple,
    IResult,
};

use super::ticket::Ticket;
#[derive(Debug, PartialEq, Eq)]
pub struct Tickets {
    pub your_ticket: Ticket,
    pub nearby_tickets: Vec<Ticket>,
}

impl Tickets {
    pub fn parse(input: &str) -> IResult<&str, Tickets> {
        let your_ticket_tag = tuple((tag("your ticket:"), newline));
        let your_ticket = Ticket::parse;
        let nearby_tickets_tag = tuple((newline, newline, tag("nearby tickets:"), newline));
        let nearby_tickets = separated_list0(newline, Ticket::parse);
        let mut parse_all = tuple((
            your_ticket_tag,
            your_ticket,
            nearby_tickets_tag,
            nearby_tickets,
        ));
        let (input, (_, your_ticket, _, nearby_tickets)) = parse_all(input)?;
        Ok((
            input,
            Tickets {
                your_ticket,
                nearby_tickets,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::model::ticket::Ticket;

    use super::Tickets;

    #[test]
    fn test_parse_tickets() -> anyhow::Result<()> {
        let input = "your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let (input, tickets) = Tickets::parse(input)?;
        assert_eq!(input, "");
        let expected = Tickets {
            your_ticket: Ticket {
                values: vec![7, 1, 14],
            },
            nearby_tickets: vec![
                Ticket {
                    values: vec![7, 3, 47],
                },
                Ticket {
                    values: vec![40, 4, 50],
                },
                Ticket {
                    values: vec![55, 2, 20],
                },
                Ticket {
                    values: vec![38, 6, 12],
                },
            ],
        };
        assert_eq!(tickets, expected);
        Ok(())
    }
}
