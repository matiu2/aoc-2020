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

/// Which way the ship is currently pointing
#[derive(Display, PartialEq, Debug)]
pub enum Pointing {
    North,
    South,
    East,
    West,
}

impl Pointing {
    /// Turn right
    fn right(&mut self) {
        use Pointing::*;
        *self = match self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }
    /// Turn left
    fn left(&mut self) {
        use Pointing::*;
        *self = match self {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }
    /// 180 degrees
    fn reverse(&mut self) {
        use Pointing::*;
        *self = match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

impl Default for Pointing {
    fn default() -> Self {
        Pointing::East
    }
}

#[derive(Default, PartialEq, Debug)]
/// The state of the ship
pub struct State {
    pointing: Pointing,
    /// Horizontal offset from 0
    x: i64,
    /// Vertical offset from 0
    y: i64,
}

impl State {
    /// manhattan distance from our origin
    pub fn distance(&self) -> i64 {
        self.x + self.y
    }

    /// Update our state
    pub fn direction(&mut self, direction: Direction) {
        match direction {
            Direction::North(n) => self.y -= n,
            Direction::South(n) => self.y += n,
            Direction::East(n) => self.x += n,
            Direction::West(n) => self.x -= n,
            Direction::Left(90) => self.pointing.left(),
            Direction::Left(180) => self.pointing.reverse(),
            Direction::Left(270) => self.pointing.right(),
            Direction::Right(90) => self.pointing.right(),
            Direction::Right(180) => self.pointing.reverse(),
            Direction::Right(270) => self.pointing.left(),
            Direction::Forward(n) => match self.pointing {
                Pointing::North => self.direction(Direction::North(n)),
                Pointing::South => self.direction(Direction::South(n)),
                Pointing::East => self.direction(Direction::East(n)),
                Pointing::West => self.direction(Direction::West(n)),
            },
            other => unreachable!("Unexpected instruction: {:?}", other),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{Direction, Pointing, State};

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

    #[test]
    fn test_steering() {
        let mut ship: State = Default::default();
        assert_eq!(0, ship.x);
        assert_eq!(0, ship.y);
        assert_eq!(Pointing::East, ship.pointing);
        // Check each direction
        // Forward 10
        ship.direction(Direction::Forward(10));
        assert_eq!(10, ship.x);
        assert_eq!(0, ship.y);
        assert_eq!(Pointing::East, ship.pointing);
        // North 3
        ship.direction(Direction::North(3));
        assert_eq!(10, ship.x);
        assert_eq!(-3, ship.y);
        assert_eq!(Pointing::East, ship.pointing);
        // Forward 7 (still facing east)
        ship.direction(Direction::Forward(7));
        assert_eq!(17, ship.x);
        assert_eq!(-3, ship.y);
        assert_eq!(Pointing::East, ship.pointing);
        // Right 90 (now facing south)
        ship.direction(Direction::Right(90));
        assert_eq!(17, ship.x);
        assert_eq!(-3, ship.y);
        assert_eq!(Pointing::South, ship.pointing);
        // Forward 11 (now facing south)
        ship.direction(Direction::Forward(11));
        assert_eq!(17, ship.x);
        assert_eq!(8, ship.y);
        assert_eq!(Pointing::South, ship.pointing);
    }
}
