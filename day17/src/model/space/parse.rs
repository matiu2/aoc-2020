use super::Space;
/// A block of space for parsing
#[derive(Debug, PartialEq, Eq)]
struct Block {
    z: i64,
    x: Vec<i64>,
    y: Vec<i64>,
}

impl Block {
    fn parse<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Option<Block> {
        // Read `z=0`
        let z_line = lines.next()?;
        let parts: Vec<&str> = z_line.split('=').collect();
        let z = match parts.as_slice() {
            ["z", z] => z.parse().ok(),
            _other => None,
        }?;
        // Read each line of the block
        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut current_y = 0;
        while let Some(row) = lines.next() {
            if row.is_empty() {
                break;
            }
            // Record the coords of the active cubes
            row.chars()
                .enumerate()
                .flat_map(|(x, c)| if c == '#' { Some(x) } else { None })
                .for_each(|current_x| {
                    x.push(current_x as i64);
                    y.push(current_y);
                });
            current_y += 1;
        }
        Some(Block { z, x, y })
    }
}

impl Space {
    pub fn parse(input: &str) -> Space {
        let mut lines = input.lines();
        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut z = Vec::new();
        while let Some(block) = Block::parse(&mut lines) {
            z.extend(vec![block.z; block.x.len()]);
            x.extend(block.x);
            y.extend(block.y);
        }
        Space { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::{Block, Space};

    #[test]
    fn test_parse_block() {
        let input = "z=0
.#.
..#
###";
        let block = Block::parse(&mut input.lines());
        assert_eq!(
            block,
            Some(Block {
                z: 0,
                x: vec![1, 2, 0, 1, 2],
                y: vec![0, 1, 2, 2, 2],
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
                x: vec![1, 2, 0, 1, 2, 0, 2, 1],
                y: vec![0, 1, 2, 2, 2, 0, 1, 2],
                z: vec![0, 0, 0, 0, 0, 1, 1, 1],
            }
        );
    }
}
