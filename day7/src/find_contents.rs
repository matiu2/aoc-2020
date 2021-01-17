//! Tools to find which bags a single bag will eventuall contain

use crate::{BagID, Containers};

pub fn count_content_recursive<'a>(query: BagID, multiplier: usize, map: &Containers<'a>) -> usize {
    if let Some(contents) = map.get(&query) {
        let mut sum = 0;
        for (bag_id, count) in contents {
            sum += *count;
            sum += count_content_recursive(*bag_id, *count, map);
        }
        sum * multiplier
    } else {
        unreachable!("Query for a bag that's not in the list: {:?}", query)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_count() {
        // We need to count how many bags could eventually contain a shiny gold bag
        let input = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
        let map = crate::rule_parser(input);
        let got = super::count_content_recursive(("shiny", "gold"), 1, &map);
        let expected = 126;
        assert_eq!(expected, got);
    }
}
