use std::fs::read_to_string;

/// Parses the puzzle input and returns (earliest_you_can_leave, bus_ids)
fn parse(input: &str) -> (usize, Vec<usize>) {
    let lines: Vec<&str> = input.lines().collect();
    assert_eq!(lines.len(), 2);
    // The earliest I can leave
    let earliest: usize = lines[0].parse().unwrap();
    // The IDs of the busses
    let bus_ids: Vec<usize> = lines[1].split(',').flat_map(|id| id.parse().ok()).collect();
    (earliest, bus_ids)
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (earliest, bus_ids) = parse(&input);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse() {
        let input = "939
7,13,x,x,59,x,31,19";
        let (earliest, bus_ids) = super::parse(input);
        assert_eq!(earliest, 939);
        assert_eq!(bus_ids, vec![7, 13, 59, 31, 19]);
    }
}
