//! Used to render space

use std::fmt::Display;

use super::{limits::Limits, Space};

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let limits = Limits::new(self).unwrap_or_default();
        for z in (limits.min_z + 1)..limits.max_z {
            writeln!(f, "z={}", z)?;
            for y in (limits.min_y + 1)..limits.max_y {
                for x in (limits.min_x + 1)..limits.max_x {
                    if self.is_active(x, y, z) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
