use std::collections::HashMap;

use crate::model::RuleLogic;

/// Handles a simple rule: does this character match with the next character
/// Returns the rest of the input if successful, or nothing if it fails
pub fn simple_rule(rule_char: char, input: &str) -> Option<&str> {
    input
        // For every unicode character (ie. decode utf-8)
        .chars()
        // Only take the first one
        .next()
        // Only take this first one if it matches the character in our rule
        .filter(|c2| *c2 == rule_char)
        // If it matches, return the rest of the input
        .map(|_| &input[1..])
}

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
/// If any fail, it returns an empty vec
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
            // For each previous output, reuse it as an input to process this link in the chain
            let new_solutions: Vec<&str> = solutions
                .iter()
                // Find all the possibilites that match using each of the
                // previous inputs, and the next rule index in the chain,
                // then flatten them into the possible output solutions (which
                // will be used for input to the next link in the chain, or for
                // the last link, returned)
                .flat_map(|input| process_rule(rules, *index, input, indent))
                .collect();
            if new_solutions.is_empty() {
                // If the next link, using the previous output as input, found no solutions
                // The chain is broken
                None
            } else {
                // Continue to the next link, providing all the solutions/remainders we've found so far
                Some(new_solutions)
            }
        })
        .unwrap_or_else(Vec::new)
}

/// Processes a single rule recursively following chains and alternate chains
/// Returns the rest of the string to be processed
fn process_rule<'a>(
    rules: &HashMap<usize, RuleLogic>,
    index: usize,
    input: &'a str,
    indent: usize,
) -> Vec<&'a str> {
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
    // If any solution exists that has consumed the whole input, this is a pass
    solutions.contains(&"")
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use pretty_assertions::assert_eq;

    use crate::model::RuleLogic;

    use super::check_input;

    /// The input for the advanced tests
    fn advanced_input() -> Vec<&'static str> {
        "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
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
            .collect()
    }

    /// The rules for the avanced tests
    fn advanced_rules() -> HashMap<usize, RuleLogic> {
        let rules = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"#;
        crate::nom_parse::rules(rules.lines()).unwrap()
    }

    /// Returns the rules for part2 of the puzzle
    fn part2_rules() -> HashMap<usize, RuleLogic> {
        let mut rules = advanced_rules();
        rules.insert(8, RuleLogic::Chain(vec![vec![42], vec![42, 8]]));
        rules.insert(11, RuleLogic::Chain(vec![vec![42, 31], vec![42, 11, 31]]));
        rules
    }

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
        let rules = crate::nom_parse::rules(rules.into_iter()).unwrap();
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
    fn test_process_rules_advanced() {
        // Part 1 test still
        pretty_env_logger::try_init().ok();
        let rules = advanced_rules();
        let lines = advanced_input();
        let passes: Vec<&str> = lines
            .into_iter()
            .filter(|line| super::check_input(&rules, line))
            .collect();
        assert_eq!(
            passes,
            vec!["bbabbbbaabaabba", "ababaaaaaabaaab", "ababaaaaabbbaba"]
        );
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
        let input = advanced_input();
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
