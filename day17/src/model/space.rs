use std::collections::HashSet;

use limits::Limits;

mod limits;
mod parse;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Space {
    // All the active blocks that we know about.
    // Anything not in here is an inactive block
    active_blocks: HashSet<(i64, i64, i64)>,
}

impl Space {
    /// Runs a part 1 cycle and returns a new state
    fn part1_cycle(&self) -> Space {
        // Find our limits
        // For piece of space
        todo!()
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
}
