use super::{limits::Limits, Space};

/// Returns every possible combination of x,y,z from Space
pub struct SpaceIterator<'a> {
    space: &'a Space,
    limits: Limits,
    x: i64,
    y: i64,
    z: i64,
}

impl<'a> SpaceIterator<'a> {
    pub fn new(space: &'a Space) -> Option<SpaceIterator> {
        let limits = Limits::new(space)?;
        Some(SpaceIterator {
            space,
            x: limits.min_x,
            y: limits.min_y,
            z: limits.min_z,
            limits,
        })
    }
}

impl<'a> Iterator for SpaceIterator<'a> {
    // x,y,z
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
