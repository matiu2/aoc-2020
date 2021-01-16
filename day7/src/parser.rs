use crate::{Containers, Contents};
use std::collections::HashMap;

/// Parse the contents part of the rule, a ',' delimited list in the format:
/// eg. 2 shiny gold bags, 9 faded blue bags.
pub fn parse_contents<'a>(line_num: usize, line: &'a str, contents: &'a str) -> Contents<'a> {
    let mut out = HashMap::new();
    let contents: Vec<&str> = contents.split(",").collect();
    contents.iter().for_each(|rule| {
        let parts: Vec<&str> = rule.trim().split(" ").collect();
        match parts.as_slice() {
            ["no", "other", bags] => {
                assert!(bags.starts_with("bags"))
            }
            [number, adjective, colour, bag] => {
                assert!(bag.starts_with("bag"), "found {}", bag);
                // Insert the rule into the sub-hashmap
                let key = (*adjective, *colour);
                let value: usize = number.parse().expect("Unable ot parse number");
                out.insert(key, value);
            }
            other => panic!(
                "Unable to parse rule at line {}: {} - parts: {:?}",
                line_num, line, other
            ),
        }
    });
    out
}

/// Parses the rules into our internal representation
pub fn rule_parser(input: &str) -> Containers {
    let mut out = HashMap::new();
    input
        .lines()
        .enumerate()
        // Remove empty lines
        .filter(|(_number, line)| !line.is_empty())
        // Split the rule into container and contents
        // eg. ["light red bags", <CONTAIN>, "1 bright white bag, 2 muted yellow bags."]
        .map(|(line_num, line)| {
            let parts: Vec<&str> = line.split("contain").collect();
            if let &[container, contents] = parts.as_slice() {
                (line_num, line, container, contents)
            } else {
                panic!("Unable to parse rule at line {}: {}", line_num, line);
            }
        })
        // Get the key to the container
        .map(|(line_num, line, container, contents)| {
            let container: Vec<&str> = container.trim().split(" ").collect();
            if let &[adjective, colour, "bags"] = container.as_slice() {
                let key = (adjective, colour);
                (line_num, line, key, contents)
            } else {
                panic!("Unable to parse rule at line {}: {}", line_num, line);
            }
        })
        // Parse the contents rules (which are ',' delimited)
        .for_each(|(line_num, line, key, contents)| {
            out.insert(key, parse_contents(line_num, line, contents));
        });
    out
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    fn input() -> &'static str {
        r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#
    }

    #[test]
    fn test_parser() {
        let got = super::rule_parser(input());
        let mut expected = HashMap::new();

        // Insert a new entry in 'expected'
        macro_rules! new_entry {
            // Insert all entries into the container rules
            ($target:expr, $(($colour:expr, $amount:literal)),+) => {
                let mut tmp = HashMap::new();
                $(
                    tmp.insert($colour, $amount);
                )+
                expected.insert($target, tmp);
            }
        }

        new_entry!(
            ("light", "red"),
            (("bright", "white"), 1),
            (("muted", "yellow"), 2)
        );
        new_entry!(
            ("dark", "orange"),
            (("bright", "white"), 3),
            (("muted", "yellow"), 4)
        );
        new_entry!(("bright", "white"), (("shiny", "gold"), 1));
        new_entry!(
            ("muted", "yellow"),
            (("shiny", "gold"), 2),
            (("faded", "blue"), 9)
        );
        new_entry!(
            ("shiny", "gold"),
            (("dark", "olive"), 1),
            (("vibrant", "plum"), 2)
        );
        new_entry!(
            ("dark", "olive"),
            (("faded", "blue"), 3),
            (("dotted", "black"), 4)
        );
        new_entry!(
            ("vibrant", "plum"),
            (("faded", "blue"), 5),
            (("dotted", "black"), 6)
        );
        expected.insert(("faded", "blue"), HashMap::new());
        expected.insert(("dotted", "black"), HashMap::new());

        assert_eq!(expected, got);
    }
}
