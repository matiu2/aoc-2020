use parse_display::{Display, FromStr};
use std::fs::read_to_string;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style = "lowercase")]
enum Height {
    #[display("{0}cm")]
    Cm(u8),
    #[display("{0}in")]
    In(u8),
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style = "lowercase")]
enum EyeColour {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style = "lowercase")]
enum Field {
    // (Birth Year) - four digits; at least 1920 and at most 2002.
    #[display("byr:{0}")]
    Byr(u16),
    // (Issue Year) - four digits; at least 2010 and at most 2020.
    #[display("iyr:{0}")]
    Iyr(u16),
    // (Expiration Year) - four digits; at least 2020 and at most 2030.
    #[display("eyr:{0}")]
    Eyr(u16),
    // (Height) - a number followed by either cm or in:
    //    If cm, the number must be at least 150 and at most 193.
    //    If in, the number must be at least 59 and at most 76.
    #[display("hgt:{0}")]
    Hgt(Height),
    // (Hair Colour) - a # followed by exactly six characters 0-9 or a-f.
    #[display("hcl:{0}")]
    Hcl(String),
    // (Eye Colour) - exactly one of: amb blu brn gry grn hzl oth.
    #[display("ecl:{0}")]
    Ecl(EyeColour),
    // (Passport ID) - a nine-digit number, including leading zeroes.
    #[display("pid:{0}")]
    Pid(String),
    // (Country ID) - ignored, missing or not.
    #[display("cid:{0}")]
    Cid(String),
}

impl Field {
    fn is_valid(&self) -> bool {
        match self {
            Field::Byr(year) => (1920..=2002).contains(year),
            Field::Iyr(year) => (2010..=2020).contains(year),
            Field::Eyr(year) => (2020..=2030).contains(year),
            Field::Hgt(Height::Cm(cm)) => (150..=193).contains(cm),
            Field::Hgt(Height::In(inches)) => (59..=76).contains(inches),
            Field::Hcl(colour) => {
                colour.len() == 7
                    && colour.chars().next() == Some('#')
                    && colour
                        .chars()
                        .skip(1)
                        .all(|c| ('a'..='f').contains(&c) || ('0'..='9').contains(&c))
            }
            Field::Ecl(_) => true,
            Field::Pid(pid) => pid.len() == 9 && pid.chars().all(char::is_numeric),
            Field::Cid(_) => true,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = read_to_string("input.txt")?;
    let mut part_1_count = 0;
    let valid_record_count = input
        // Split the input into passport records
        .split("\n\n")
        // Split each record into fields
        .map(|record| record.split_ascii_whitespace().collect::<Vec<&str>>())
        // Run the part-1 validation check
        .filter(|fields| match fields.len() {
            // 8 fields is a valid record
            8 => true,
            // 7 fields is only valid as long as the missing one is `cid`, ie. All the present fields are not `Cid` fields
            7 => !fields.iter().any(|field| field.starts_with("cid:")),
            _ => false,
        })
        // Count the part 1 fields that are valid
        .inspect(|_| part_1_count += 1)
        // Run the part 2 validation check
        // Turn the text into Field enums, removing invalid fields
        .map(|fields| -> Vec<Field> {
            fields
                .into_iter()
                .flat_map(|field| field.parse().ok())
                .collect()
        })
        // Check the new length of the fields, if it has reduced, the whole record is invalid
        .filter(|fields| match fields.len() {
            // 8 fields is a valid record
            8 => true,
            // 7 fields is only valid as long as the missing one is `cid`, ie. All the present fields are not `Cid` fields
            7 => !fields.iter().any(|field| {
                if let Field::Cid(_) = field {
                    true
                } else {
                    false
                }
            }),
            _ => false,
        })
        // Do the second level validation check on each field
        .filter(|fields| fields.iter().all(Field::is_valid))
        // Return the new count
        .count();
    println!("Day 4 - part 1 - valid_records = {}", part_1_count);
    println!("Day 4 - part 2 - valid_records = {}", valid_record_count);
    Ok(())
}
