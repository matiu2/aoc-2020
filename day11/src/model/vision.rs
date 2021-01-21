//! Everything to do with people seeing other seats from seats
//! (Part 2 of the puzzle)

use crate::Space;

use super::Spaces;

impl Spaces {
    /// Starting at grid space x (col number) and y (row number),
    /// It'll return the number of occupied seats visible (along the 8 direction lines)
    /// An empty seat blocks the view of occupied seats
    /// Once the count hits `max` the function will return early. `max` can be
    /// from 1-8 because we search in 8 directions
    pub fn count_visible_occupied_seats(&self, x: usize, y: usize, max: usize) -> usize {
        let mut count = 0;
        // Closure used to see if a space matches our search
        let seat_search = |x, y| -> Option<bool> {
            match self.get(x, y) {
                // An occupied seat adds to the count
                Some(Space::OccupiedSeat) => Some(true),
                // An empty seat blocks the rest of the view, so stop searching
                Some(Space::EmptySeat) => Some(false),
                // Anything else means, keep searching
                _ => None,
            }
        };
        // Look left
        if (0..x)
            .rev()
            .find_map(|x| seat_search(x, y))
            .unwrap_or(false)
        {
            count += 1;
            if count >= max {
                return count;
            }
        }
        // Look up
        if (0..y)
            .rev()
            .find_map(|y| seat_search(x, y))
            .unwrap_or(false)
        {
            count += 1;
            if count >= max {
                return count;
            }
        }
        // Look right
        if (x + 1..self.width)
            .find_map(|x| seat_search(x, y))
            .unwrap_or(false)
        {
            count += 1;
            if count >= max {
                return count;
            }
        }
        // Look down
        if (y + 1..self.height)
            .find_map(|y| seat_search(x, y))
            .unwrap_or(false)
        {
            count += 1;
            if count >= max {
                return count;
            }
        }
        // Look left and up diagnally
        if (0..x)
            .rev()
            .zip((0..y).rev())
            .find_map(|(x, y)| seat_search(x, y))
            .unwrap_or(false)
        {
            count += 1;
            if count >= max {
                return count;
            }
        }
        // Look right and up diagnally
        if (x + 1..self.width)
            .zip((0..y).rev())
            .find_map(|(x, y)| seat_search(x, y))
            .unwrap_or(false)
        {
            count += 1;
            if count >= max {
                return count;
            }
        }
        // Look right and down diagnally
        if (x + 1..self.width)
            .zip(y + 1..self.height)
            .find_map(|(x, y)| seat_search(x, y))
            .unwrap_or(false)
        {
            count += 1;
            if count >= max {
                return count;
            }
        }
        // Look left and down diagnally
        if ((0..x).rev())
            .zip(y + 1..self.height)
            .find_map(|(x, y)| seat_search(x, y))
            .unwrap_or(false)
        {
            count += 1;
            if count >= max {
                return count;
            }
        }
        count
    }

    /// Takes a step forward in the animation, returning the new state
    pub fn step_part2(&self) -> Self {
        let converter = |x, y, space| {
            match space {
                Space::EmptySeat => {
                    // If a seat is empty (L) and there are no occupied seats
                    // in line of sight, the seat becomes occupied.
                    if self.count_visible_occupied_seats(x, y, 1) == 0 {
                        // Sit in the chair
                        Space::OccupiedSeat
                    } else {
                        space
                    }
                }
                Space::OccupiedSeat => {
                    // If a seat is occupied (#) and five or more seats
                    // adjacent to it are also occupied, the seat becomes
                    // empty.
                    if self.count_visible_occupied_seats(x, y, 5) == 5 {
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
}

#[cfg(test)]
mod tests {
    use crate::Spaces;

    #[test]
    fn test_count_seat_searchs_8() {
        let input = r#".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#....."#;
        let spaces: Spaces = input.parse().unwrap();
        assert_eq!(8, spaces.count_visible_occupied_seats(3, 4, 8));
        assert_eq!(4, spaces.count_visible_occupied_seats(3, 4, 4));
        assert_eq!(1, spaces.count_visible_occupied_seats(3, 4, 1));
    }

    #[test]
    fn test_count_seat_searchs_1() {
        let input = ".............
.L.L.#.#.#.#.
.............";
        let spaces: Spaces = input.parse().unwrap();
        assert_eq!(0, spaces.count_visible_occupied_seats(1, 1, 1));
        assert_eq!(0, spaces.count_visible_occupied_seats(1, 1, 8));
        assert_eq!(1, spaces.count_visible_occupied_seats(3, 1, 1));
        assert_eq!(1, spaces.count_visible_occupied_seats(3, 1, 8));
    }

    #[test]
    fn test_count_seat_searchs_0() {
        let input = ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.";
        let spaces: Spaces = input.parse().unwrap();
        assert_eq!(0, spaces.count_visible_occupied_seats(3, 3, 1));
        assert_eq!(0, spaces.count_visible_occupied_seats(3, 3, 8));
    }

    #[test]
    fn test_step_part_2() {
        let expected_steps = vec![
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
            "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#",
            "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#",
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#",
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
        ];
        let mut spaces: Spaces = expected_steps[0].parse().unwrap();
        assert_eq!(expected_steps[0], &format!("{}", &spaces));
        for (i, expected) in expected_steps.iter().enumerate().skip(1) {
            spaces = spaces.step_part2();
            let got = format!("{}", &spaces);
            if expected != &got {
                println!(
                    "Step {}\nPrevious:\n{}\n\nExpected:\n{}\n\nGot:\n{}",
                    i,
                    expected_steps[i - 1],
                    expected,
                    got
                );
            }
            assert_eq!(expected, &got, "step {} doesn't match", i);
        }
    }
}
