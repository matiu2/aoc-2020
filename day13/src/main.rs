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

//// Returns (bus_id, minutes you have to wait (after `earliest`))
fn earliest_bus_id(earliest: usize, bus_ids: &[usize]) -> Option<(usize, usize)> {
    bus_ids
        .iter()
        .map(|bus_id| (bus_id, bus_id - (earliest % bus_id)))
        .min_by_key(|(_bus_id, to_wait)| *to_wait)
        .map(|(&bus_id, to_wait)| (bus_id, to_wait))
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (earliest, bus_ids) = parse(&input);
    let (bus_id, to_wait) = earliest_bus_id(earliest, &bus_ids).unwrap();
    let answer = to_wait * bus_id;
    println!(
        "Day13 - Part 1 - Bus ID: {} minutes to wait: {} answer: {}",
        bus_id, to_wait, answer
    );
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

    #[test]
    fn test_earliest_bus_id() {
        let earliest = 939;
        let bus_ids = vec![7, 13, 59, 31, 19];
        let (bus_id, to_wait) = super::earliest_bus_id(earliest, &bus_ids).unwrap();
        assert_eq!(bus_id, 59);
        assert_eq!(to_wait, 5);
    }
}
