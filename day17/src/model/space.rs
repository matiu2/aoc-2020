use std::collections::HashSet;

use limits::Limits;

mod limits;
mod parse;
mod render;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Space {
    // All the active blocks that we know about.
    // Anything not in here is an inactive block
    active_blocks: HashSet<(i64, i64, i64)>,
}

impl Space {
    /// Runs a part 1 cycle and returns a new state
    pub fn part1_cycle(&self) -> Space {
        // Find our limits
        // For piece of space
        let active_blocks: HashSet<_> = self
            .iter()
            .filter(|(x, y, z)| {
                let count = self.check_neighbours(*x, *y, *z);
                match (self.is_active(*x, *y, *z), count) {
                    // If a cube is active and exactly 2 or 3 of its neighbors are also
                    // active, the cube remains active. Otherwise, the cube becomes
                    // inactive.
                    (true, 2) | (true, 3) => true,
                    (true, _) => false,
                    // If a cube is inactive but exactly 3 of its neighbors are
                    // active, the cube becomes active. Otherwise, the cube remains
                    // inactive.
                    (false, 3) => true,
                    (false, _) => false,
                }
            })
            .collect();
        Space { active_blocks }
    }

    /// Returns the count of active cubes
    pub fn active_count(&self) -> usize {
        self.active_blocks.len()
    }

    /// Returns the number of active neighbours for a cube, up to a maximum of 4
    /// ie. returns 0, 1, 2, 3 or 4 - even if it has 9 active neighbours
    pub fn check_neighbours(&self, x: i64, y: i64, z: i64) -> usize {
        self.get_neighbours(x, y, z)
            .filter(|(x, y, z)| self.is_active(*x, *y, *z))
            .take(4)
            .count()
    }

    /// Returns true if a cube is active
    pub fn is_active(&self, x: i64, y: i64, z: i64) -> bool {
        self.active_blocks.contains(&(x, y, z))
    }

    /// Iterate over every possibly affected space coordinate
    pub fn iter(&self) -> impl Iterator<Item = (i64, i64, i64)> {
        Limits::new(self).unwrap_or_default().into_iter()
    }

    /// Return the neighbour coordinates for a cube
    pub fn get_neighbours(&self, x: i64, y: i64, z: i64) -> impl Iterator<Item = (i64, i64, i64)> {
        ((z - 1)..=(z + 1))
            .flat_map(move |z| ((y - 1)..=(y + 1)).map(move |y| (y, z)))
            .flat_map(move |(y, z)| ((x - 1)..=(x + 1)).map(move |x| (x, y, z)))
            .filter(move |(x1, y1, z1)| (x, y, z) != (*x1, *y1, *z1))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use super::Space;

    #[test]
    fn test_get_neighbours() {
        let space: Space = Default::default();
        let neighbours: HashSet<_> = space.get_neighbours(0, 0, 0).collect();
        assert!(neighbours.contains(&(-1, -1, -1)));
        assert!(neighbours.contains(&(-1, 0, 0)));
        assert!(neighbours.contains(&(1, 0, 0)));
        // Shouldn't contain itself
        assert!(!neighbours.contains(&(0, 0, 0)));
        // Shouldn't contain anything 2 blocks away
        assert!(!neighbours.contains(&(2, 0, 0)));
    }

    #[test]
    fn test_stage1_cycle() {
        pretty_env_logger::try_init().ok();
        let input = "z=0
.#.
..#
###";
        let start = Space::parse(input);
        let step1 = start.part1_cycle();
        let input = "z=-1
#..
..#
.#.

z=0
#.#
.##
.#.

z=1
#..
..#
.#.";
        let expected = Space::parse(input);
        assert_eq!(format!("{}", &step1), format!("{}", &expected));
        let step2 = step1.part1_cycle();
        let expected = Space::parse(
            "z=-2
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
.###.

z=1
..#..
.#..#
....#
.#...
.....

z=2
.....
.....
..#..
.....
.....",
        );
        assert_eq!(format!("{}", &step2), format!("{}", &expected));
        let step3 = step2.part1_cycle();
        let expected = Space::parse(
            "z=-2
.......
.......
..##...
..###..
.......
.......
.......

z=-1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=0
...#...
.......
#......
.......
.....##
.##.#..
...#...

z=1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=2
.......
.......
..##...
..###..
.......
.......
.......",
        );
        assert_eq!(format!("{}", &step3), format!("{}", &expected));
    }
}
