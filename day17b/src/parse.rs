//! Parse 3d and 4d inputs
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{eof, map, map_res, opt, recognize},
    multi::{many0, many_till},
    sequence::{pair, preceded, separated_pair},
    IResult, Parser,
};

use crate::Point;

/// Reads a possibly signed integer
fn number(input: &str) -> IResult<&str, i64> {
    map_res(
        recognize(pair(opt(tag("+").or(tag("-"))), digit1)),
        |number: &str| number.parse::<i64>(),
    )(input)
}

/// Gets '9' from z=9
fn get_z(input: &str) -> IResult<&str, i64> {
    preceded(tag("z="), number)(input)
}

/// Gets '42' from w=42
fn get_w(input: &str) -> IResult<&str, i64> {
    preceded(tag("w="), number)(input)
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
fn row(input: &str) -> IResult<&str, Vec<i64>> {
    map(
        many_till(active, tag("\n").or(eof)),
        |(bools, _new_line)| -> Vec<i64> {
            // Take a sequence of true/false, return the index of the 'true' values; these are the 'x' values
            bools
                .iter()
                .enumerate()
                .flat_map(|(x, val)| val.then(|| x as i64))
                .collect()
        },
    )(input)
}

/// Parses a paragraph of rows, returns (x, y) coords
fn block(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    map(
        many_till(row, tag("\n").or(eof)),
        |(rows, _end)| -> Vec<(i64, i64)> {
            rows.iter()
                .enumerate()
                .flat_map(|(y, row)| row.iter().map(move |x| (*x, y as i64)))
                .collect()
        },
    )(input)
}

/// Parses 1 block of data into a 3 dimensional space
fn space_block_3d(input: &str) -> IResult<&str, Vec<Point<3>>> {
    map(separated_pair(get_z, tag("\n"), block), |(z, xys)| {
        xys.into_iter()
            .map(|(x, y)| Point::new([x, y, z]))
            .collect::<Vec<Point<3>>>()
    })(input)
}

/// Parses 1 block of data into a 4 dimensional space
fn space_block_4d(input: &str) -> IResult<&str, Vec<Point<4>>> {
    map(
        separated_pair(get_z_and_w, tag("\n"), block),
        |((z, w), xys)| {
            xys.into_iter()
                .map(|(x, y)| Point::new([x, y, z, w]))
                .collect::<Vec<Point<4>>>()
        },
    )(input)
}

/// Parses a bunch of 3D space blocks and gives you all the points
pub fn space_3d(input: &str) -> IResult<&str, Vec<Point<3>>> {
    map(many0(space_block_3d), |xys| {
        xys.into_iter().flatten().collect::<Vec<Point<3>>>()
    })(input)
}

/// Parses a bunch of 4D space blocks and gives you all the points
pub fn space_4d(input: &str) -> IResult<&str, Vec<Point<4>>> {
    map(many0(space_block_4d), |xys| {
        xys.into_iter().flatten().collect::<Vec<Point<4>>>()
    })(input)
}

#[cfg(test)]
mod tests {
    use crate::Point;

    #[test]
    fn test_get_z() {
        assert_eq!(super::get_z("z=42"), Ok(("", 42)));
        assert_eq!(super::get_z("z=-2"), Ok(("", -2)));
    }

    #[test]
    fn test_get_w() {
        assert_eq!(super::get_w("w=99"), Ok(("", 99)));
        assert_eq!(super::get_w("w=-2"), Ok(("", -2)));
    }

    #[test]
    fn test_get_z_and_w() {
        assert_eq!(super::get_z_and_w("z=420, w=9"), Ok(("", (420, 9))));
        assert_eq!(super::get_z_and_w("z=+1, w=-9"), Ok(("", (1, -9))));
        assert_eq!(super::get_z_and_w("z=-1, w=+9"), Ok(("", (-1, 9))));
        assert_eq!(super::get_z_and_w("z=-1, w=-4"), Ok(("", (-1, -4))));
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

    #[test]
    fn test_block() {
        assert_eq!(
            super::block(".#.\n..#\n###").unwrap(),
            ("", vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)])
        );
        assert_eq!(
            super::block(".#.\n..#\n###\n\nremainder").unwrap(),
            ("remainder", vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)])
        );
    }

    #[test]
    fn test_space_block_3d() {
        use crate::Point;
        let (remainder, space) = super::space_block_3d("z=4\n.#.\n..#\n###\n\nremainder").unwrap();
        assert_eq!(remainder, "remainder");
        let expected_space = vec![
            Point::new([1, 0, 4]),
            Point::new([2, 1, 4]),
            Point::new([0, 2, 4]),
            Point::new([1, 2, 4]),
            Point::new([2, 2, 4]),
        ];
        assert_eq!(&space, &expected_space);
    }

    #[test]
    fn test_space_block_4d() {
        let (remainder, space) =
            super::space_block_4d("z=4, w=-1\n.#.\n..#\n###\n\nremainder").unwrap();
        assert_eq!(remainder, "remainder");
        let expected_space = vec![
            Point::new([1, 0, 4, -1]),
            Point::new([2, 1, 4, -1]),
            Point::new([0, 2, 4, -1]),
            Point::new([1, 2, 4, -1]),
            Point::new([2, 2, 4, -1]),
        ];
        assert_eq!(&space, &expected_space);
    }

    #[test]
    fn test_space_3d() {
        let input = r#"z=-2
.....
.....
..#..
.....
.....

z=-1
..#..
.#..#
....#
.#...
.....

z=0
##...
##...
#....
....#
.###."#;
        let (remainder, space) = super::space_3d(input).unwrap();
        assert_eq!(remainder, "");
        let expected_space = vec![
            Point::new([2, 2, -2]),
            Point::new([2, 0, -1]),
            Point::new([1, 1, -1]),
            Point::new([4, 1, -1]),
            Point::new([4, 2, -1]),
            Point::new([1, 3, -1]),
            Point::new([0, 0, 0]),
            Point::new([1, 0, 0]),
            Point::new([0, 1, 0]),
            Point::new([1, 1, 0]),
            Point::new([0, 2, 0]),
            Point::new([4, 3, 0]),
            Point::new([1, 4, 0]),
            Point::new([2, 4, 0]),
            Point::new([3, 4, 0]),
        ];
        assert_eq!(space, expected_space);
    }

    fn test_space_4d() {
        let input = r#"z=-2, w=10
.....
.....
..#..
.....
.....

z=-1, w=-4
..#..
.#..#
....#
.#...
.....

z=0, w=42
##...
##...
#....
....#
.###."#;
        let (remainder, space) = super::space_4d(input).unwrap();
        assert_eq!(remainder, "");
        let expected_space = vec![
            Point::new([2, 2, -2, 10]),
            Point::new([2, 0, -1, -4]),
            Point::new([1, 1, -1, -4]),
            Point::new([4, 1, -1, -4]),
            Point::new([4, 2, -1, -4]),
            Point::new([1, 3, -1, -4]),
            Point::new([0, 0, 0, 42]),
            Point::new([1, 0, 0, 42]),
            Point::new([0, 1, 0, 42]),
            Point::new([1, 1, 0, 42]),
            Point::new([0, 2, 0, 42]),
            Point::new([4, 3, 0, 42]),
            Point::new([1, 4, 0, 42]),
            Point::new([2, 4, 0, 42]),
            Point::new([3, 4, 0, 42]),
        ];
        assert_eq!(space, expected_space);
    }
}
