//! Parse 3d and 4d inputs
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::map_res;
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::IResult;

/// Gets '9' from z=9
fn get_z(input: &str) -> IResult<&str, i64> {
    preceded(tag("z="), map_res(digit1, |s: &str| s.parse::<i64>()))(input)
}

/// Gets '42' from w=42
fn get_w(input: &str) -> IResult<&str, i64> {
    preceded(tag("w="), map_res(digit1, |s: &str| s.parse::<i64>()))(input)
}

/// Gets the 'z' and 'w' values from: z=1, w=2
fn get_z_and_w(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(get_z, pair(tag(","), space0), get_w)(input)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_z() {
        assert_eq!(super::get_z("z=42"), Ok(("", 42)));
    }

    #[test]
    fn test_get_w() {
        assert_eq!(super::get_w("w=99"), Ok(("", 99)));
    }

    #[test]
    fn test_get_z_and_w() {
        assert_eq!(super::get_z_and_w("z=420, w=9"), Ok(("", (420, 9))));
    }
}
