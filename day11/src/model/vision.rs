//! Everything to do with people seeing other seats from seats
//! (Part 2 of the puzzle)

use crate::Space;

use super::Spaces;

impl Spaces {
    /// Starting at grid space x (col number) and y (row number),
    /// It'll return the number of occupied seats visible (along the 8 direction lines)
    /// An empty seat blocks the view of occupied seats
    /// If you only care that there are `any` occupied chairs visible, set `any`
    /// to true, and the function will exit as soon as it finds an occupied space
    pub fn count_visible_seat_searchs(&self, x: usize, y: usize, any: bool) -> usize {
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
            if any && count > 0 {
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
            if any && count > 0 {
                return count;
            }
        }
        // Look right
        if (x + 1..self.width)
            .find_map(|x| seat_search(x, y))
            .unwrap_or(false)
        {
            count += 1;
            if any && count > 0 {
                return count;
            }
        }
        // Look down
        if (y + 1..self.height)
            .find_map(|y| seat_search(x, y))
            .unwrap_or(false)
        {
            count += 1;
            if any && count > 0 {
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
            if any && count > 0 {
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
            if any && count > 0 {
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
            if any && count > 0 {
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
            if any && count > 0 {
                return count;
            }
        }
        count
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
        assert_eq!(8, spaces.count_visible_seat_searchs(3, 4, false));
        assert_eq!(1, spaces.count_visible_seat_searchs(3, 4, true));
    }

    #[test]
    fn test_count_seat_searchs_1() {
        let input = ".............
.L.L.#.#.#.#.
.............";
        let spaces: Spaces = input.parse().unwrap();
        assert_eq!(0, spaces.count_visible_seat_searchs(1, 1, true));
        assert_eq!(0, spaces.count_visible_seat_searchs(1, 1, false));
        assert_eq!(1, spaces.count_visible_seat_searchs(3, 1, true));
        assert_eq!(1, spaces.count_visible_seat_searchs(3, 1, false));
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
        assert_eq!(0, spaces.count_visible_seat_searchs(3, 3, true));
        assert_eq!(0, spaces.count_visible_seat_searchs(3, 3, false));
    }
}
