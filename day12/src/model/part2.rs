use std::mem::swap;

use super::{Direction, Pointing};

#[derive(PartialEq, Debug)]
/// The state of the ship
pub struct State {
    pointing: Pointing,
    /// Waypoint / multiplier / direction finder relative to ship horizontal offset
    way_x: i64,
    /// Waypoint / multiplier / direction finder relative to ship vertical offset
    way_y: i64,
    /// Horizontal offset from 0
    x: i64,
    /// Vertical offset from 0
    y: i64,
}

impl Default for State {
    fn default() -> Self {
        State {
            pointing: Pointing::East,
            way_x: 10,
            way_y: -1,
            x: 0,
            y: 0,
        }
    }
}

impl State {
    /// Manhattan distance from origin
    pub fn distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    /// Process one instruction and move the ship or the waypoint
    pub fn process_instruction(&mut self, direction: Direction) {
        match direction {
            // Move the waypoint / direction finder North
            Direction::North(n) => self.way_y -= n,
            Direction::South(n) => self.way_y += n,
            Direction::East(n) => self.way_x += n,
            Direction::West(n) => self.way_x -= n,
            // Rotate the waypoint around the ship
            Direction::Left(270) | Direction::Right(90) => {
                swap(&mut self.way_x, &mut self.way_y);
                self.way_x = -self.way_x;
            }
            Direction::Left(180) | Direction::Right(180) => {
                self.way_x = -self.way_x;
                self.way_y = -self.way_y;
            }
            Direction::Left(90) | Direction::Right(270) => {
                swap(&mut self.way_x, &mut self.way_y);
                self.way_y = -self.way_y;
            }
            Direction::Forward(n) => {
                // Move forward multiples of the waypoint
                self.x += self.way_x * n;
                self.y += self.way_y * n;
            }
            other => unreachable!("Unexpected instruction: {:?}", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::default;

    use crate::Direction;

    use super::State;

    #[test]
    fn test_instruction_processing() {
        // The waypoint starts 10 units east and 1 unit north relative to the ship
        let mut ship: State = Default::default();
        // F10 moves the ship to the waypoint 10 times (a total of 100 units east
        // and 10 units north), leaving the ship at east 100, north 10. The
        // waypoint stays 10 units east and 1 unit north of the ship.
        ship.process_instruction(Direction::Forward(10));
        assert_eq!(ship.x, 100);
        assert_eq!(ship.y, -10);
        // N3 moves the waypoint 3 units north to 10 units east and 4 units north
        // of the ship. The ship remains at east 100, north 10.
        ship.process_instruction(Direction::North(3));
        assert_eq!(ship.x, 100);
        assert_eq!(ship.y, -10);
        assert_eq!(ship.way_x, 10);
        assert_eq!(ship.way_y, -4);
        // F7 moves the ship to the waypoint 7 times (a total of 70 units east and
        // 28 units north), leaving the ship at east 170, north 38. The waypoint
        // stays 10 units east and 4 units north of the ship.
        ship.process_instruction(Direction::Forward(7));
        assert_eq!(ship.x, 170);
        assert_eq!(ship.y, -38);
        assert_eq!(ship.way_x, 10);
        assert_eq!(ship.way_y, -4);
        // R90 rotates the waypoint around the ship clockwise 90 degrees, moving
        // it to 4 units east and 10 units south of the ship. The ship remains at
        // east 170, north 38.
        ship.process_instruction(Direction::Right(90));
        assert_eq!(ship.x, 170);
        assert_eq!(ship.y, -38);
        assert_eq!(ship.way_x, 4);
        assert_eq!(ship.way_y, 10);
        // F11 moves the ship to the waypoint 11 times (a total of 44 units east
        // and 110 units south), leaving the ship at east 214, south 72. The
        // waypoint stays 4 units east and 10 units south of the ship.
        ship.process_instruction(Direction::Forward(11));
        assert_eq!(ship.x, 214);
        assert_eq!(ship.y, 72);
        assert_eq!(ship.way_x, 4);
        assert_eq!(ship.way_y, 10);
        // After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.
        assert_eq!(286, ship.distance());
    }
}
