use super::{Direction, Pointing};

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

    use super::State;
    use crate::model::{Direction, Pointing};

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
