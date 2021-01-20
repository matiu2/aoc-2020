use parse_display::{Display, FromStr};

/// A grid space in the waiting room
#[derive(Display, FromStr, PartialEq, Debug)]
pub enum Space {
    #[display("#")]
    OccupiedSeat,
    #[display("L")]
    EmptySeat,
    #[display(".")]
    Floor,
}

impl Space {
    /// Returns true if this seat is occupied
    fn is_occupied(&self) -> bool {
        if let Space::OccupiedSeat = self {
            true
        } else {
            false
        }
    }

    /// Occupies a seat - panics if you try to occupy the floor or an already occupied seat
    fn occupy(&mut self) {
        if let Space::EmptySeat = self {
            *self = Space::OccupiedSeat
        } else {
            panic!("Tried to occupy {:?}", self)
        }
    }

    /// Vacates a seat - panics if you try to occupy the floor or an already empty seat
    fn vacate(&mut self) {
        if let Space::OccupiedSeat = self {
            *self = Space::OccupiedSeat
        } else {
            panic!("Tried to occupy {:?}", self)
        }
    }
}

/// All the spaces in the waiting room
#[derive(Default, Debug)]
pub struct Spaces {
    // Data is stored [row,row,row]
    // So to access coordinate (x,y), index is y * width + x
    // So to access coordinate (col, row), index is row * width + col
    // eg. col 3 row 0 = index 3
    // eg. col 3 row 2 = index 2 * width + 3
    data: Vec<Space>,
    width: usize,
    height: usize,
}

impl Spaces {
    /// Runs through one step of iteration
    /// Returns true if it has arrived at a stable state
    pub fn step(&mut self) -> bool {
        let mut dirty = false;
        for row in 0..self.height {
            for col in 0..self.width {
                let space = self.get_mut(col, row).expect(&format!(
                    "Unable to get a space for row: {} col: {}",
                    row, col
                ));
                match space {
                    Space::EmptySeat => {
                        // If a seat is empty (L) and there are no occupied seats
                        // adjacent to it, the seat becomes occupied.
                        if self
                            .adjacent(row, col)
                            .iter()
                            .all(|space| !space.is_occupied())
                        {
                            dirty = true;
                            // Sit in the chair
                            self.occupy(row, col);
                        }
                    }
                    Space::OccupiedSeat => {
                        // If a seat is occupied (#) and four or more seats
                        // adjacent to it are also occupied, the seat becomes
                        // empty.
                        if self
                            .adjacent(row, col)
                            .iter()
                            .filter(|space| space.is_occupied())
                            .take(4)
                            .count()
                            == 4
                        {
                            dirty = true;
                            // Get out of the chair
                            self.vacate(row, col);
                        }
                    }
                    // We don't care about floor - nothing changes there
                    Space::Floor => (),
                };
            }
        }
        dirty
    }

    /// Occupies an empty seat - panics for anything else
    fn occupy(&mut self, row: usize, col: usize) {
        self.get_mut(col, row).map(|space| space.occupy());
    }

    /// Occupies an empty seat - panics for anything else
    fn vacate(&mut self, row: usize, col: usize) {
        self.get_mut(col, row).map(|space| space.vacate());
    }

    /// Returns a vec of adjacent spaces to a given space
    /// Normal spaces have 8 adjacents (horizontal, vertical, and the two diagnals)
    /// Edge spaces, only get 3
    /// Corner spaces, only get 2
    pub fn adjacent(&self, row: usize, col: usize) -> Vec<&Space> {
        // All possible directions we can go
        let deltas = [-1, 0, 1];
        // Go every direction
        deltas
            .iter()
            .flat_map(|dx| deltas.iter().map(move |dy| (dx, dy)))
            // Don't include the current space
            .filter(|(&dx, &dy)| dx != 0 && dy != 0)
            // Do the math as i64 (rather than usize)
            // Don't go left if we're already at the left
            .filter(|(&dx, _dy)| !(col == 0 && dx == -1))
            // Don't go right if we're at the end
            .filter(|(&dx, _dy)| !(col == self.width + 1 && dx == 1))
            // Don't go up if we're at the top
            .filter(|(_dx, &dy)| !(row == 0 && dy == -1))
            // Don't go up if we're at the top
            .filter(|(_dx, &dy)| !(row == self.height + 1 && dy == 1))
            // Convert the deltas into coordinates
            .map(|(dx, dy)| (row as i64 + dy, col as i64 + dx))
            // i64 to usize
            .map(|(x, y)| (x as usize, y as usize))
            // coordinates to actual Spaces
            .flat_map(|(x, y)| self.get(y, x))
            .collect()
    }
}

impl std::fmt::Display for Spaces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let space = self.get(col, row).expect(&format!(
                    "Unable to get a space for row: {} col: {}",
                    row, col
                ));
                write!(f, "{}", space)?;
            }
            if row != self.height - 1 {
                // Add a new line, except at the last line
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Spaces {
    pub fn push_row(&mut self, row: Vec<Space>) {
        if self.width == 0 && self.data.is_empty() {
            // If this is the first push, get our row size
            self.width = row.len();
        }
        self.height += 1;
        self.data.extend(row);
    }

    /// Retrieves the space at column x and row y, if any
    pub fn get(&self, x: usize, y: usize) -> Option<&Space> {
        self.data.get(y * self.width + x)
    }

    /// Allows us to change a seat
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Space> {
        self.data.get_mut(y * self.width + x)
    }

    /// Read-only width - set it using FromStr when you build this struct
    pub fn width(&self) -> usize {
        self.width
    }

    /// Read-only width - set it using FromStr when you build this struct
    pub fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_display_spaces() {
        let input = r#"L.LL.LL.LL
LLL#LLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
        let spaces: super::Spaces = input.parse().unwrap();
        let output = format!("{}", spaces);
        assert_eq!(input, output);
    }

    #[test]
    fn test_step() {
        let input = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
        let mut spaces: super::Spaces = input.parse().unwrap();
        spaces.step();
        let step_1_expected = r#"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"#;
        println!("{}", spaces);
        assert_eq!(step_1_expected, format!("{}", spaces));
    }
}
