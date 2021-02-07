//! Helps us find the li min_x: (), max_x: (), min_y: (), max_y: (), min_z: (), max_z: () min_x: (), max_x: (), min_y: (), max_y: (), min_z: (), max_z: () min_x: (), max_x: (), min_y: (), max_y: (), min_z: (), max_z: ()mits of space

use super::Space;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Limits {
    pub min_x: i64,
    pub max_x: i64,
    pub min_y: i64,
    pub max_y: i64,
    pub min_z: i64,
    pub max_z: i64,
}

impl Limits {
    /// Create a new set of limits
    pub fn new(space: &Space) -> Option<Limits> {
        let (x, y, z) = *space.active_blocks.iter().next()?;
        let min = [x, y, z];
        let max = [x, y, z];
        let (min, max) =
            space
                .active_blocks
                .iter()
                .fold((min, max), |(mut min, mut max), (x, y, z)| {
                    for i in 0..3 {
                        let input = [x, y, z];
                        if input[i] < &min[i] {
                            min[i] = *input[i];
                        }
                        if input[i] > &max[i] {
                            max[i] = *input[i];
                        }
                    }
                    (min, max)
                });
        // Add one, so that we count the neighbours of the edge active squares
        Some(Limits {
            min_x: min[0] - 1,
            max_x: max[0] + 1,
            min_y: min[1] - 1,
            max_y: max[1] + 1,
            min_z: min[2] - 1,
            max_z: max[2] + 1,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (i64, i64, i64)> + '_ {
        (self.min_z..=self.max_z)
            .flat_map(move |z| (self.min_y..=self.max_y).map(move |y| (y, z)))
            .flat_map(move |(y, z)| (self.min_x..=self.max_x).map(move |x| (x, y, z)))
    }

    pub fn into_iter(self) -> impl Iterator<Item = (i64, i64, i64)> {
        let z_range = self.min_z..=self.max_z;
        let y_range = self.min_y..=self.max_y;
        let x_range = self.min_x..=self.max_x;
        z_range
            .flat_map(move |z| y_range.clone().map(move |y| (y, z)))
            .flat_map(move |(y, z)| x_range.clone().map(move |x| (x, y, z)))
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Space;

    use super::Limits;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_limits() {
        let space = Space {
            active_blocks: vec![(-1, -2, -3), (1, 2, 3)].into_iter().collect(),
        };
        let limits = Limits::new(&space).unwrap();
        assert_eq!(
            limits,
            Limits {
                min_x: -2,
                max_x: 2,
                min_y: -3,
                max_y: 3,
                min_z: -4,
                max_z: 4,
            }
        );
    }

    #[test]
    fn test_iter() {
        let space = Space {
            active_blocks: vec![(-1, -2, -3), (1, 2, 3)].into_iter().collect(),
        };
        let limits = Limits::new(&space).unwrap();
        let out: Vec<_> = limits.iter().collect();
        let expected_start = vec![
            (-2, -3, -4),
            (-1, -3, -4),
            (0, -3, -4),
            (1, -3, -4),
            (2, -3, -4),
        ];
        assert_eq!(&out[0..expected_start.len()], &expected_start);
        let expected_end = vec![
            (2, 2, 4),
            (-2, 3, 4),
            (-1, 3, 4),
            (0, 3, 4),
            (1, 3, 4),
            (2, 3, 4),
        ];
        assert_eq!(
            &out[out.len() - expected_end.len()..out.len()],
            &expected_end
        );
    }

    #[test]
    fn test_into_iter() {
        let space = Space {
            active_blocks: vec![(-1, -2, -3), (1, 2, 3)].into_iter().collect(),
        };
        let limits = Limits::new(&space).unwrap();
        let out: Vec<_> = limits.into_iter().collect();
        let expected_start = vec![
            (-2, -3, -4),
            (-1, -3, -4),
            (0, -3, -4),
            (1, -3, -4),
            (2, -3, -4),
        ];
        assert_eq!(&out[0..expected_start.len()], &expected_start);
        let expected_end = vec![
            (2, 2, 4),
            (-2, 3, 4),
            (-1, 3, 4),
            (0, 3, 4),
            (1, 3, 4),
            (2, 3, 4),
        ];
        assert_eq!(
            &out[out.len() - expected_end.len()..out.len()],
            &expected_end
        );
    }
}
