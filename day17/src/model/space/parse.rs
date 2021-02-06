use std::collections::HashSet;

use super::Space;

impl Space {
    fn parse_block<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Option<Space> {
        // Read `z=0`
        let z_line = lines.next()?;
        let parts: Vec<&str> = z_line.split('=').collect();
        let z = match parts.as_slice() {
            ["z", z] => z.parse().ok(),
            _other => None,
        }?;
        // Read each line of the block
        let mut active_blocks = HashSet::new();
        let mut y = 0;
        while let Some(row) = lines.next() {
            if row.is_empty() {
                break;
            }
            // Record the coords of the active cubes
            row.chars()
                .enumerate()
                .flat_map(|(x, c)| if c == '#' { Some(x as i64) } else { None })
                .for_each(|x| {
                    active_blocks.insert((x, y, z));
                });
            y += 1;
        }
        Some(Space { active_blocks })
    }

    pub fn parse(input: &str) -> Space {
        let mut lines = input.lines();
        let mut active_blocks = HashSet::new();
        while let Some(block) = Space::parse_block(&mut lines) {
            block.active_blocks.into_iter().for_each(|active_block| {
                active_blocks.insert(active_block);
            });
        }
        Space { active_blocks }
    }
}

#[cfg(test)]
mod tests {
    use super::Space;

    #[test]
    fn test_parse_block() {
        let input = "z=0
.#.
..#
###";
        let block = Space::parse_block(&mut input.lines());
        assert_eq!(
            block,
            Some(Space {
                active_blocks: vec![(1, 0, 0), (2, 1, 0), (0, 2, 0), (1, 2, 0), (2, 2, 0)]
                    .into_iter()
                    .collect()
            })
        )
    }

    #[test]
    fn test_parse() {
        let input = "z=0
.#.
..#
###

z=1
#..
..#
.#.";
        let space = Space::parse(input);
        assert_eq!(
            space,
            Space {
                active_blocks: vec![
                    (1, 0, 0),
                    (2, 1, 0),
                    (0, 2, 0),
                    (1, 2, 0),
                    (2, 2, 0),
                    (0, 0, 1),
                    (2, 1, 1),
                    (1, 2, 1)
                ]
                .into_iter()
                .collect()
            }
        );
    }
}
