use crate::model::Rule;

/// Handles a simple rule: does this character match with the next character
/// Returns the rest of the input if successful, or nothing if it fails
fn simple_rule(rule_char: char, input: &str) -> Option<&str> {
    input
        .chars()
        .next()
        .filter(|c2| *c2 == rule_char)
        .map(|_| &input[1..])
}

// Takes a bunch of alternate rule index chains; if at least one matches, returns the rest of the input
fn chains<'a>(chains: &[Vec<usize>], rules: &[Rule], input: &'a str) -> Option<&'a str> {
    chains
        .iter()
        .flat_map(|this_chain| chain(this_chain, rules, input))
        .next()
}

/// Takes a chain of rule indexes, if they all match, it returns the rest of the string
/// If any fail, it returns None
fn chain<'a>(chain: &[usize], rules: &[Rule], input: &'a str) -> Option<&'a str> {
    chain
        .iter()
        .try_fold(input, |input, index| process_rule(rules, *index, input))
}

/// Processes a single rule recursively following chains and alternate chains
/// Returns true all characters match the rule
fn process_rule<'a>(rules: &[Rule], index: usize, input: &'a str) -> Option<&'a str> {
    let rule = &rules[index];
    log::debug!(
        "Checking Rule {}: {:?} against input: {}",
        index,
        rule,
        input,
    );
    let result = match rule {
        // This char matches
        Rule::Simple(c) => simple_rule(*c, input),
        Rule::Chain(indexes) => chains(indexes, rules, input),
    };
    log::debug!("Rule: {:?} Input: {} = {:?}", rule, input, result);
    result
}

/// Check an input line of text against the rule collection
pub fn check_input(rules: &[Rule], input: &str) -> bool {
    process_rule(rules, 0, input)
        .map(|left_over| left_over.is_empty())
        .unwrap_or(false)
}

#[cfg(test)]
mod test {

    use crate::{model::Rule, nom_parse::rule as parse};

    #[test]
    fn test_process_rule() {
        pretty_env_logger::try_init().ok();
        let rules = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#;
        let input = r#"ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        let rules: Vec<Rule> = rules.lines().map(|rule| parse(rule).unwrap().1).collect();
        let passes: Vec<bool> = input
            .lines()
            .map(|line| super::check_input(&rules, line))
            .collect();
        assert_eq!(passes, vec![true, false, true, false, false]);
    }
}
