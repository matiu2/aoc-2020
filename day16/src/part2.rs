//! Figure out which field matches each column of values

use std::collections::{HashMap, HashSet};

use crate::model::{Field, Problem, Ticket};

/// Takes in a problem and returns a map of which field validates which column of ticket data
pub fn order_fields(problem: &Problem) -> HashMap<&Field, usize> {
    // First remove tickets with any invalid fields
    let valid_tickets: Vec<&Ticket> = problem
        .tickets
        .nearby_tickets
        .iter()
        // Include our ticket
        .chain(vec![&problem.tickets.your_ticket])
        // Only keep tickets where at least one column validates against at least one field
        .filter(|ticket| {
            // If any value
            !ticket.values.iter().any(|value| {
                problem
                    .fields
                    .iter()
                    // fails to match all fields, the ticket is invalid
                    .all(|field| !field.range_1.contains(value) && !field.range_2.contains(value))
            })
        })
        .collect();
    // Store each field and the columns it could possibly be part of
    let field_count = problem.fields.len();
    let mut possible_columns: HashMap<&Field, HashSet<usize>> = problem
        .fields
        .iter()
        .map(|field| (field, (0..field_count).collect()))
        .collect();
    for field in &problem.fields {
        let possibilities = possible_columns.get(field).unwrap();
        // Remove any columns that don't validate
        let to_remove: HashSet<usize> = valid_tickets
            .iter()
            // For every ticket, get every column value, that could possibly match this field
            .flat_map(|ticket| {
                ticket
                    .values
                    .iter()
                    .enumerate()
                    // We only care about columns that could possibly match this field
                    .filter(|(index, _value)| possibilities.contains(index))
                    .map(move |(index, value)| (index, value))
            })
            // Now we have, (column index, value) - we want to remove values that are invalid
            .flat_map(|(index, value)| {
                if field.validate(value) {
                    None
                } else {
                    // We need to remove this from the possible columns for this field
                    Some(index)
                }
            })
            .collect();
        let new_possibilities: HashSet<usize> =
            possibilities.difference(&to_remove).cloned().collect();
        possible_columns.insert(field, new_possibilities);
    }
    // A map of each field to its column
    let mut out = HashMap::new();
    // Now go throug the list of possible columns, matching any field that matches exactly one column, until all fields have been assigned a column
    while let Some((&field, &column)) = possible_columns
        .iter()
        .find(|(_field, possibilities)| possibilities.len() == 1)
        .and_then(|(field, possibilities)| {
            possibilities.iter().next().map(|column| (field, column))
        })
    {
        // Remember this combination
        out.insert(field, column);
        // Remove the field from possible_columns
        possible_columns.remove(field);
        // Remove the column from all other field's possibilites
        for (_field, possibilites) in &mut possible_columns {
            possibilites.remove(&column);
        }
    }
    // Return the map of fields to columns
    out
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::model::Problem;
    use anyhow::anyhow;

    #[test]
    fn test_part2() -> anyhow::Result<()> {
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
        let map = super::order_fields(&problem);
        let expected = vec![("class", 1), ("row", 0), ("seat", 2)];
        let mut got: Vec<(&str, usize)> = map
            .iter()
            .map(|(field, &column_index)| (field.name.as_str(), column_index))
            .collect();
        got.sort();
        assert_eq!(expected, got);
        Ok(())
    }

    #[test]
    fn test_part2_big() -> anyhow::Result<()> {
        let input = read_to_string("input.txt")?;
        let (_input, problem) = Problem::parse(input.as_str()).map_err(|e| anyhow!("{:?}", e))?;
        let map = super::order_fields(&problem);
        let expected = vec![
            ("arrival location", 8),
            ("arrival platform", 0),
            ("arrival station", 16),
            ("arrival track", 19),
            ("class", 13),
            ("departure date", 17),
            ("departure location", 10),
            ("departure platform", 4),
            ("departure station", 18),
            ("departure time", 15),
            ("departure track", 14),
            ("duration", 12),
            ("price", 2),
            ("route", 11),
            ("row", 9),
            ("seat", 5),
            ("train", 6),
            ("type", 1),
            ("wagon", 3),
            ("zone", 7),
        ];
        let mut got: Vec<(&str, usize)> = map
            .iter()
            .map(|(field, &column_index)| (field.name.as_str(), column_index))
            .collect();
        got.sort();
        assert_eq!(expected, got);
        Ok(())
    }
}
