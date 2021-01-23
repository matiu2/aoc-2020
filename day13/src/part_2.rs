//! Because Part 2 is so different to part 1 - it has it's own module

use anyhow::Result;

/// Parses the single line of puzzle input containing the bus ids and into Option<usize> where None means `x` in the input
pub fn parse(input: &str) -> Result<Vec<Option<usize>>> {
    // The IDs of the busses
    let bus_ids: Vec<Option<usize>> = input.split(',').map(|id| id.parse().ok()).collect();
    Ok(bus_ids)
}

/// Tests if a number matches our requirements (all bus_ids leaving after that number leave 'n' minutes before)
/// Returns Some(true) if this start number gives a sequential leaving_times
/// Returns None if there are no bus_ids, or if the first or last bus_id is a `None` (we just don't expect that to ever happen with our input)
fn test_number(start: usize, bus_ids: &[Option<usize>]) -> Option<bool> {
    // Validate the sequence
    let found = bus_ids
        .iter()
        // Make a modulo starting at start
        .zip(start..usize::MAX)
        // If there's a bus leaving, modifier % bus_id should always be 0
        .map(|(bus_id, modifier)| bus_id.map(|bus_id| modifier % bus_id))
        // All bus ids should be sequential
        .all(|leaving_time| {
            match leaving_time {
                // We can ignore this leaving time, the modifier will still increment though
                None => true,
                // The secuence continues
                Some(0) => true,
                // Out of sequence
                _ => false,
            }
        });
    Some(found)
}

/// Given the list of bus_ids, returns the earliest point where each bus_id leaves one_minute after the other, in the order given, (if any)
pub fn old_calculate(bus_ids: &[Option<usize>]) -> Option<usize> {
    let first = bus_ids.first().cloned().flatten().unwrap();
    (0..usize::MAX)
        .step_by(first)
        .find(|earliest| test_number(*earliest, bus_ids).unwrap_or(false))
}

pub fn check_number(base: usize, number_to_check: usize, index: usize) -> bool {
    (base + index) % number_to_check == 0
}

pub fn calculate(bus_ids: &[Option<usize>]) -> Option<usize> {
    // Sort the numbers, biggest first
    let mut sorted: Vec<(usize, usize)> = bus_ids
        .iter()
        .enumerate()
        .flat_map(|(i, id)| id.map(|id| (i, id)))
        .collect();
    sorted.sort_by_key(|(_i, id)| *id);

    // Take the biggest number and its offset for later
    let (offset, step_size) = sorted.pop()?;

    // Walk over the biggest numbers, trying to match all other numbers
    ((step_size - offset)..usize::MAX)
        .step_by(step_size)
        .find(|base| sorted.iter().all(|(i, id)| check_number(*base, *id, *i)))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn test_parse() -> Result<()> {
        let input = "7,13,x,x,59,x,31,19";
        let bus_ids = super::parse(input)?;
        assert_eq!(
            bus_ids,
            vec![
                Some(7),
                Some(13),
                None,
                None,
                Some(59),
                None,
                Some(31),
                Some(19)
            ]
        );
        Ok(())
    }

    #[test]
    fn test_test_number() -> Result<()> {
        let input = "7,13,x,x,59,x,31,19";
        let bus_ids = super::parse(input)?;
        let got = super::test_number(1068781, &bus_ids);
        assert_eq!(got, Some(true));
        Ok(())
    }

    fn do_calculate(input: &str) -> Option<usize> {
        let bus_ids = super::parse(input).unwrap();
        super::calculate(&bus_ids)
    }

    #[test]
    fn test_calculate() {
        assert_eq!(do_calculate("7,13,x,x,59,x,31,19"), Some(1068781));
        assert_eq!(do_calculate("17,x,13,19"), Some(3417));
        assert_eq!(do_calculate("67,7,59,61"), Some(754018));
        assert_eq!(do_calculate("67,x,7,59,61"), Some(779210));
        assert_eq!(do_calculate("67,7,x,59,61"), Some(1261476));
        assert_eq!(do_calculate("1789,37,47,1889"), Some(1202161486));
    }
}
