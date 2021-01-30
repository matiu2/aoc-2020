use std::ops::RangeInclusive;

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::{separated_pair, terminated},
    IResult,
};

/// Represents a field declaration of a ticket
#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub range_1: RangeInclusive<usize>,
    pub range_2: RangeInclusive<usize>,
}

impl Field {
    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, Field> {
        // Define how to read a range
        let number = |s: &'a str| map_res(digit1, |s: &'a str| s.parse::<usize>())(s);
        let range_raw = |s: &'a str| separated_pair(number, tag("-"), number)(s);
        let range = |s: &'a str| map(range_raw, |(a, b): (usize, usize)| a..=b)(s);

        // Read the name, up until the ':'
        let (input, name) =
            terminated(map(is_not(":"), |s: &str| s.to_string()), tag(": "))(input)?;
        let (input, (range_1, range_2)) = separated_pair(range, tag(" or "), range)(input)?;

        Ok((
            input,
            Field {
                name,
                range_1,
                range_2,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::Field;

    #[test]
    fn test_parse_field() -> anyhow::Result<()> {
        let (_rest, field): (&str, Field) = Field::parse("class: 1-3 or 5-7")?;
        assert_eq!(&field.name, "class");
        assert_eq!(field.range_1, 1..=3);
        assert_eq!(field.range_2, 5..=7);
        Ok(())
    }
}
