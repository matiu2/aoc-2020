use anyhow::anyhow;
use std::fs::read_to_string;

use day16::{model::Problem, part1::get_invalid_values, part2::order_fields};

fn main() -> anyhow::Result<()> {
    let input = read_to_string("input.txt")?;
    {
        let (_rest, problem) = Problem::parse(input.as_str())
            .map_err(|e| anyhow!("Unable to parse problem: {:?}", e))?;
        let part1_answer: usize = problem
            .tickets
            .nearby_tickets
            .iter()
            .map(|ticket| -> usize { get_invalid_values(ticket, &problem.fields).iter().sum() })
            .sum();
        println!("Day 16 - Part 1 = {}", part1_answer);
        // Part 2
        let part2_field_map = order_fields(&problem);
        let part2_answer = part2_field_map
            .iter()
            .filter(|(field, _column_index)| field.name.starts_with("departure"))
            .map(|(_field, column_index)| problem.tickets.your_ticket.values[*column_index])
            .fold(1, |product, value| product * value);
        println!("Day 16 - Part 2 = {}", part2_answer);
    }
    Ok(())
}
