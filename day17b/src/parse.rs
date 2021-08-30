//! Parse 3d and 4d inputs
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{eof, map, map_res},
    multi::many_till,
    sequence::{pair, preceded, separated_pair},
    IResult, Parser,
};

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

/// Returns true if a spot on a cube is active (#) or false if it's inactive (.)
fn active(input: &str) -> IResult<&str, bool> {
    map(tag("#"), |_| true)
        .or(map(tag("."), |_| false))
        .parse(input)
}

/// Returns a vec of 'x' values of active cells from a row
fn row(input: &str) -> IResult<&str, Vec<usize>> {
    map(
        many_till(active, tag("\n").or(eof)),
        |(bools, _new_line)| -> Vec<usize> {
            // Take a sequence of true/false, return the index of the 'true' values; these are the 'x' values
            bools
                .iter()
                .enumerate()
                .flat_map(|(x, val)| val.then(|| x))
                .collect()
        },
    )(input)
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

    #[test]
    fn test_active() {
        assert_eq!(super::active("#"), Ok(("", true)));
        assert_eq!(super::active("."), Ok(("", false)));
    }

    #[test]
    fn test_row() {
        assert_eq!(super::row(".#.#..##.").unwrap(), ("", vec![1, 3, 6, 7]));
        assert_eq!(super::row("#.#..##.\n").unwrap(), ("", vec![0, 2, 5, 6]));
    }
}
