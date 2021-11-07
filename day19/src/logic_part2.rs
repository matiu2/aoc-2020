use std::collections::HashMap;

use crate::logic_shared::simple_rule;
use crate::model::RuleLogic;

// Takes a bunch of alternate rule index chains, We must try each possibility
fn chains<'a>(
    chains: &[Vec<usize>],
    rules: &HashMap<usize, RuleLogic>,
    input: &'a str,
    indent: usize,
) -> Vec<&'a str> {
    // Collect all the possibilites, and use the one that consumes the most
    chains
        // Look at every posssible chain
        .iter()
        // Only take chains that pass
        .flat_map(|this_chain| chain(this_chain, rules, input, indent + 1))
        .collect()
}

/// Takes a chain of rule indexes, if they all match, it returns the rest of the string
/// If any fail, it returns None
fn chain<'a>(
    chain: &[usize],
    rules: &HashMap<usize, RuleLogic>,
    input: &'a str,
    indent: usize,
) -> Vec<&'a str> {
    chain
        .iter()
        // Try to go through all the links in the chain
        .try_fold(vec![input], |solutions, index| {
            // For each previous output
            let new_solutions: Vec<&str> = solutions
                .iter()
                // Use it as the input for the next link
                // Flatten everything we find
                .flat_map(|input| process_rule(rules, *index, input, indent))
                .collect();
            if new_solutions.is_empty() {
                // If the next link, using the previous output as input, found no solutions
                // The chain is broken
                None
            } else {
                // Now return all the solutions we found for this chain
                Some(new_solutions)
            }
        })
        .unwrap_or_else(|| vec![])
}

/// Processes a single rule recursively following chains and alternate chains
/// Returns the rest of the string to be processed
fn process_rule<'a>(
    rules: &HashMap<usize, RuleLogic>,
    index: usize,
    input: &'a str,
    indent: usize,
) -> Vec<&'a str> {
    if input.is_empty() {
        return vec![];
    }
    let rule = &rules[&index];
    log::debug!(
        "{:-indent$}Checking: {:2}: {:?} Input: {}",
        "",
        index,
        rule,
        input,
        indent = indent,
    );
    let result: Vec<&str> = match rule {
        // Match a single char
        RuleLogic::Simple(c) => simple_rule(*c, input).into_iter().collect(),
        // Or match a chain
        RuleLogic::Chain(indexes) => chains(indexes, rules, input, indent),
    };
    log::debug!(
        "{:-indent$}{} {:?}",
        "",
        !result.is_empty(),
        result,
        indent = indent,
    );
    // Return whatever's left over from the input
    result
}

/// Check an input line of text against the rule collection
pub fn check_input(rules: &HashMap<usize, RuleLogic>, input: &str) -> bool {
    let solutions = process_rule(rules, 0, input, 0);
    log::info!("Solutions: {:?}", solutions);
    solutions.contains(&"")
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use crate::{model::RuleLogic, test_utils::part2_rules};

    use super::check_input;

    #[test]
    fn test_process_rule() {
        pretty_env_logger::try_init().ok();
        let rules = [
            "0: 4 1 5",     // A -> AAAB or AABA or BBAB or BBBA -> B
            "1: 2 3 | 3 2", // AAAB or AABA or BBAB or BBBA
            "2: 4 4 | 5 5", // AA or BB
            "3: 4 5 | 5 4", // AB or BA
            r#"4: "a""#,
            r#"5: "b""#,
        ];
        let rules = crate::nom_parse::rules(&rules).unwrap();
        let lines = [
            "ababbb",  // Matches
            "bababa",  // No match
            "abbbab",  // Match
            "aaabbb",  // No match
            "aaaabbb", // Too long
        ];
        let passes: Vec<&str> = lines
            .into_iter()
            .filter(|line| check_input(&rules, line))
            .collect();
        assert_eq!(passes, vec!["ababbb", "abbbab"]);
    }

    #[test]
    fn part2_simple() {
        pretty_env_logger::try_init().ok();
        let rules = part2_rules();
        let input = "aaaaabbaabaaaaababaa";
        assert!(super::check_input(&rules, input));
    }

    #[test]
    fn test_part2() {
        pretty_env_logger::try_init().ok();
        let input: Vec<&str> = "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            .lines()
            .collect();
        let expected: Vec<&str> = "bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            .lines()
            .collect();
        let rules = part2_rules();
        let output: Vec<&str> = input
            .iter()
            .filter(|line| check_input(&rules, line))
            .cloned()
            .collect();
        assert_eq!(expected, output);
    }

    #[test]
    fn test_rule8() {
        pretty_env_logger::try_init().ok();
        // Rule 8 is 1 or more rule 42s, so all of these should match
        // (see [rules.md](rules.md) for an expansion of rule 42 possibilites)
        let mut rules = part2_rules();
        rules.insert(0, RuleLogic::Chain(vec![vec![8]]));
        let input =
            "babbbbaabbbbaabbbabbbbbabbbbbbabbbbaabbbaaaabaaabbaaabaabababbbbaaaaaabaaaabbaaa";
        assert!(check_input(&rules, input));
    }

    #[test]
    fn test_rule11() {
        // Rule 11 is 42, follwed by 0 or more 42,31, followed by a trailing 31
        let mut rules = part2_rules();
        rules.insert(0, RuleLogic::Chain(vec![vec![11]]));
        let rule42 = "babbb";
        let rule31 = "bbaba";
        let rule4231 = format!("{}{}", rule42, rule31);
        let input = format!("{}{}{}", rule42, rule4231, rule31);
        assert!(check_input(&rules, &input));
    }
}
