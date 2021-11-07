use std::collections::HashMap;

use crate::logic_shared::simple_rule;
use crate::model::RuleLogic;

// Takes a bunch of alternate rule index chains, return the first one that matches
fn chains<'a>(
    chains: &[Vec<usize>],
    rules: &HashMap<usize, RuleLogic>,
    input: &'a str,
    indent: usize,
) -> Option<&'a str> {
    // Collect all the possibilites, and use the one that consumes the most
    chains
        // Look at every posssible chain
        .iter()
        // Only take chains that pass
        .flat_map(|this_chain| chain(this_chain, rules, input, indent + 1))
        // Take the first one that matches
        .next()
}

/// Takes a chain of rule indexes, if they all match, it returns the rest of the string
/// If any fail, it returns None
fn chain<'a>(
    chain: &[usize],
    rules: &HashMap<usize, RuleLogic>,
    input: &'a str,
    indent: usize,
) -> Option<&'a str> {
    chain
        .iter()
        // Try to go through all the links in the chain
        // Fail if any links fail
        .try_fold(input, |input, index| {
            // Pass the output from the last link as the output to the next
            process_rule(rules, *index, input, indent)
        })
}

/// Processes a single rule recursively following chains and alternate chains
/// Returns the rest of the string to be processed
fn process_rule<'a>(
    rules: &HashMap<usize, RuleLogic>,
    index: usize,
    input: &'a str,
    indent: usize,
) -> Option<&'a str> {
    let rule = &rules[&index];
    log::debug!(
        "{:-indent$}Checking: {:2}: {:?} Input: {}",
        "",
        index,
        rule,
        input,
        indent = indent,
    );
    let result: Option<&str> = match rule {
        // Match a single char
        RuleLogic::Simple(c) => simple_rule(*c, input),
        // Or match a chain
        RuleLogic::Chain(indexes) => chains(indexes, rules, input, indent),
    };
    log::debug!(
        "{:-indent$}{} {:?}",
        "",
        result.is_some(),
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
    // input matches if there is a solution and it has zero remainders
    solutions
        .map(|remainder| remainder.is_empty())
        .unwrap_or(false)
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    #[test]
    fn test_process_rules() {
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
            .filter(|line| super::check_input(&rules, line))
            .collect();
        assert_eq!(passes, vec!["ababbb", "abbbab"]);
    }

    #[test]
    fn test_process_rules_advanced() {
        pretty_env_logger::try_init().ok();
        let rules: Vec<&str> = r#"42: 9 14 | 10 1
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
24: 14 1"#
            .lines()
            .collect();
        let rules = crate::nom_parse::rules(&rules).unwrap();
        let lines: Vec<&str> = "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
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
        let passes: Vec<&str> = lines
            .into_iter()
            .filter(|line| super::check_input(&rules, line))
            .collect();
        assert_eq!(
            passes,
            vec!["bbabbbbaabaabba", "ababaaaaaabaaab", "ababaaaaabbbaba"]
        );
    }
}
