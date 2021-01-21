use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
pub enum Direction {
    // Move (but don't turn North)
    #[display("N{0}")]
    North(i64),
    #[display("S{0}")]
    South(i64),
    #[display("E{0}")]
    East(i64),
    #[display("W{0}")]
    West(i64),
    // Turn left .0 degrees (always 90, 180, 270)
    #[display("L{0}")]
    Left(i64),
    // Turn right .0 degrees (always 90, 180, 270)
    #[display("R{0}")]
    Right(i64),
    // Move Forward `n` spaces
    #[display("F{0}")]
    Forward(i64),
}

#[cfg(test)]
mod tests {
    use super::Direction;

    #[test]
    fn test_parse() {
        let input = "F10
N3
F7
R90
F11";
        let got: Vec<Direction> = input.lines().map(|line| line.parse().unwrap()).collect();
        let expected = vec![
            Direction::Forward(10),
            Direction::North(3),
            Direction::Forward(7),
            Direction::Right(90),
            Direction::Forward(11),
        ];
        assert_eq!(expected, got);
    }
}
