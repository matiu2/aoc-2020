//! Parses the input

/// Parses a string of line separated numbers into a Vec of usize's
/// Panics if anything goes wrong
pub fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .enumerate()
        .filter(|(_index, line)| !line.is_empty())
        .map(|(index, line)| {
            line.parse()
                .expect(&format!("Unable to parse line {}: {}", index + 1, line))
        })
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parsing() {
        let input = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"#;
        let got = super::parse(input);
        let expected = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(got, expected);
    }
}
