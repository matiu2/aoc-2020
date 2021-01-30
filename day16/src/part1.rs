use crate::model::{Field, Ticket};

/// Part 1 of the puzzle, find tickets where every value is outside of the field ranges
/// This function returns all the values in the ticket, that don't validate against any fields
pub fn get_invalid_values(ticket: &Ticket, fields: &[Field]) -> Vec<usize> {
    ticket
        .values
        .iter()
        // If any value ...
        .filter(|value| {
            // ... fails to validate all fields, the ticket is invalid
            fields
                .iter()
                .all(|field| !field.range_1.contains(value) && !field.range_2.contains(value))
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::model::Problem;

    use super::get_invalid_values;

    #[test]
    fn test_completely_invalid() -> anyhow::Result<()> {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let (_input, problem) = Problem::parse(input)?;
        let invalid_values: Vec<usize> = problem
            .tickets
            .nearby_tickets
            .iter()
            .flat_map(|ticket| get_invalid_values(ticket, &problem.fields))
            .collect();
        dbg!(&invalid_values);
        let scanning_error_rate: usize = invalid_values.iter().sum();
        assert_eq!(scanning_error_rate, 71);
        Ok(())
    }
}
