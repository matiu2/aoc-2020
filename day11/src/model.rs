use parse_display::{Display, FromStr};
mod vision;

/// A grid space in the waiting room
#[derive(Display, FromStr, PartialEq, Eq, Debug, Clone, Copy)]
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
}

/// All the spaces in the waiting room
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Spaces {
    // Data is stored [y,y,y]
    // So to access coordinate (x,y), index is y * width + x
    // So to access coordinate (x, y), index is y * width + x
    // eg. x 3 y 0 = index 3
    // eg. x 3 y 2 = index 2 * width + 3
    data: Vec<Space>,
    width: usize,
    height: usize,
}

impl Spaces {
    /// Runs through one step of iteration

    /// Returns the new state
    pub fn step(&self) -> Self {
        let converter = |x, y, space| {
            match space {
                Space::EmptySeat => {
                    // If a seat is empty (L) and there are no occupied seats
                    // adjacent to it, the seat becomes occupied.
                    if self.adjacent(x, y).iter().all(|space| !space.is_occupied()) {
                        // Sit in the chair
                        Space::OccupiedSeat
                    } else {
                        space
                    }
                }
                Space::OccupiedSeat => {
                    // If a seat is occupied (#) and four or more seats
                    // adjacent to it are also occupied, the seat becomes
                    // empty.
                    if self
                        .adjacent(x, y)
                        .iter()
                        .filter(|space| space.is_occupied())
                        .take(4)
                        .count()
                        == 4
                    {
                        // Get out of the chair
                        Space::EmptySeat
                    } else {
                        space
                    }
                }
                // We don't care about floor - nothing changes there
                Space::Floor => space,
            }
        };
        self.step_generic(converter)
    }

    /// Returns the new state
    ///
    /// # Type Arguments
    ///
    /// C: Converter function, takes a (x, y, Space) and returns an Space
    fn step_generic<C>(&self, converter_func: C) -> Self
    where
        C: Fn(usize, usize, Space) -> Space,
    {
        let data: Vec<Space> = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .map(|(x, y)| {
                (
                    x,
                    y,
                    self.get(x, y)
                        .expect(&format!("Unabel to get cell at x: {} y: {}", x, y)),
                )
            })
            .map(|(x, y, space)| converter_func(x, y, *space))
            .collect();
        Spaces {
            data,
            height: self.height,
            width: self.width,
        }
    }

    /// Returns a vec of adjacent spaces to a given space
    /// Normal spaces have 8 adjacents (horizontal, vertical, and the two diagnals)
    /// Edge spaces, only get 3
    /// Corner spaces, only get 2
    pub fn adjacent(&self, x: usize, y: usize) -> Vec<&Space> {
        // All possible directions we can go
        let deltas = [-1, 0, 1];
        // Go every direction
        deltas
            .iter()
            .flat_map(|dx| deltas.iter().map(move |dy| (dx, dy)))
            // Don't include the current space
            .filter(|(&dx, &dy)| !(dx == 0 && dy == 0))
            // Do the math as i64 (rather than usize)
            // Don't go left if we're already at the left
            .filter(|(&dx, _dy)| !(x == 0 && dx == -1))
            // Don't go right if we're at the end
            .filter(|(&dx, _dy)| !(x == self.width - 1 && dx == 1))
            // Don't go up if we're at the top
            .filter(|(_dx, &dy)| !(y == 0 && dy == -1))
            // Don't go up if we're at the top
            .filter(|(_dx, &dy)| !(y == self.height - 1 && dy == 1))
            // Convert the deltas into coordinates
            .map(|(dx, dy)| (x as i64 + dx, y as i64 + dy))
            // i64 to usize
            .map(|(x, y)| (x as usize, y as usize))
            // coordinates to actual Spaces
            .flat_map(|(x, y)| self.get(x, y))
            .collect()
    }

    /// Returns the number of occupied seats
    pub fn count_occupied(&self) -> usize {
        self.data.iter().filter(|space| space.is_occupied()).count()
    }
}

impl std::fmt::Display for Spaces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let space = self
                    .get(x, y)
                    .expect(&format!("Unable to get a space for x: {} y: {}", x, y));
                write!(f, "{}", space)?;
            }
            if y != self.height - 1 {
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

    /// Retrieves the space at x and y (0,0 is top-left), if any
    pub fn get(&self, x: usize, y: usize) -> Option<&Space> {
        self.data.get(y * self.width + x)
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
        let step_2_expected = r#"#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##"#;
        let step_3_expected = r#"#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##"#;
        let step_4_expected = r#"#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##"#;
        let step_5_expected = r#"#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##"#;

        let spaces: super::Spaces = input.parse().unwrap();
        let spaces = spaces.step();
        println!("\nPrevious:\n{}", input);
        println!("\nExpected:\n{}", step_1_expected);
        println!("\nGot:\n{}", spaces);
        assert_eq!(step_1_expected, format!("{}", spaces));
        // Step 2
        let spaces = spaces.step();
        assert_eq!(step_2_expected, format!("{}", spaces));
        // Step 3
        let spaces = spaces.step();
        assert_eq!(step_3_expected, format!("{}", spaces));
        // Step 4
        let spaces = spaces.step();
        assert_eq!(step_4_expected, format!("{}", spaces));
        // Step 5
        let spaces = spaces.step();
        assert_eq!(step_5_expected, format!("{}", spaces));
        // Step 6 -- Should be the same as step 5
        let spaces = spaces.step();
        assert_eq!(step_5_expected, format!("{}", spaces));
    }
}
