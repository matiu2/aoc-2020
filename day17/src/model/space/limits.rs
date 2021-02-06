//! Helps us find the li min_x: (), max_x: (), min_y: (), max_y: (), min_z: (), max_z: () min_x: (), max_x: (), min_y: (), max_y: (), min_z: (), max_z: () min_x: (), max_x: (), min_y: (), max_y: (), min_z: (), max_z: ()mits of space

use super::Space;

#[derive(Debug, PartialEq, Eq)]
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
}

#[cfg(test)]
mod tests {
    use crate::model::Space;

    use super::Limits;

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
}
