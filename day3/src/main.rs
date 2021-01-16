use std::fs::read_to_string;

/// Takes a string in the format ...# where `.` is a space (piece of snow)
/// and `#` is a tree.
/// Returns a the number of columns in the first line, and a vec of rows of indexes of trees.
fn convert_input(input: &str) -> (usize, Vec<Vec<usize>>) {
    let lines: Vec<&str> = input.lines().collect();
    let col_count = lines[0].chars().count();
    let output = lines
        .iter()
        .enumerate()
        .map(|(_row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(col, _)| col)
                .collect()
        })
        .collect();
    (col_count, output)
}

/// Counts the trees hit given an `input`
///  * col_count - how many columns exist in the input
///  * tree_rows - A vec of rows, each holding the index of trees on that row
///  * horizontal_steps - how many steps to move to the right each turn
///  * vertical_steps - how many rows we move down each tern
fn count_trees_hit(
    col_count: usize,
    tree_rows: &[Vec<usize>],
    horizontal_steps: usize,
    vertical_steps: usize,
) -> usize {
    // Each line is a row where trees are
    // containing a vector of the columns that contain trees
    // Navigate to the bottom, counting tree hits
    let mut current_col = 0;
    tree_rows
        .iter()
        // Skip the first step's rows, because we're navigating down `vertical_steps` at the start
        .skip(vertical_steps)
        // Step the rows
        .step_by(vertical_steps)
        .filter(|row_of_trees| {
            // Navigate
            current_col += horizontal_steps;
            // Wrap around horizontally
            current_col %= col_count;
            // See if our current col hits a tree
            row_of_trees.contains(&current_col)
        })
        .count()
}

fn main() -> anyhow::Result<()> {
    // Get the coordinates of the trees
    let input = read_to_string("input.txt")?;
    let (col_count, tree_rows) = convert_input(&input);
    let tree_count = count_trees_hit(col_count, &tree_rows, 3, 1);
    println!("Part 1: We hit {} trees", tree_count);

    // Each tuple holds, horiz_steps, vert_steps
    let part_2_inputs: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let answer = part_2_inputs
        .iter()
        .map(|(horiz_steps, vert_steps)| {
            let count = count_trees_hit(col_count, &tree_rows, *horiz_steps, *vert_steps);
            println!("Input ({}, {}) = {}", horiz_steps, vert_steps, count);
            count
        })
        .fold(1, |previous, count| previous * count);

    println!("Part 2: We hit {} trees", answer);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{convert_input, count_trees_hit};

    fn get_input() -> &'static str {
        r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#
    }

    #[test]
    fn test_part1() {
        let input = get_input();
        println!("{}", input);
        let (col_count, tree_rows) = convert_input(&input);
        let tree_count = count_trees_hit(col_count, tree_rows.as_slice(), 3, 1);
        assert_eq!(7, tree_count);
    }

    #[test]
    fn test_part2() {
        let input = get_input();
        println!("{}", input);
        let (col_count, tree_rows) = convert_input(&input);
        let part_2_inputs: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let answer = part_2_inputs
            .iter()
            .map(|(horiz_steps, vert_steps)| {
                let count = count_trees_hit(col_count, &tree_rows, *horiz_steps, *vert_steps);
                println!("Input ({}, {}) = {}", horiz_steps, vert_steps, count);
                count
            })
            .fold(1, |previous, count| previous * count);
        assert_eq!(336, answer);
    }
}
