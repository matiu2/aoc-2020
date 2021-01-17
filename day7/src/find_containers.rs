use crate::{BagID, Containers};
use std::collections::HashSet;

/// Finds all the direct containers of a certain type of magic bag (`query`)
fn direct_containers<'a>(query: BagID, map: &Containers<'a>) -> Vec<BagID<'a>> {
    map.iter()
        .filter(|(_key, value)| value.contains_key(&query))
        .map(|(key, _value)| key)
        .cloned()
        .collect()
}

/// Recursively finds all magic bags that can eventually contain a certain magic bag (`query`)
pub fn find_containers<'a>(query: BagID, map: &Containers<'a>) -> HashSet<BagID<'a>> {
    // First find all the top level bags that contain our query (eg. shiny gold)
    // Because this is a recursive search, this acts as our stack
    let mut containers_to_search: Vec<BagID> = direct_containers(query, map);
    // Now recurse through all the direct containers that contain our query
    let mut searched_queries = HashSet::new();
    // Get the next query from the stack
    while let Some(query) = containers_to_search.pop() {
        // Ignore this query if we've alread searched it
        if searched_queries.contains(&query) {
            continue;
        }
        // Add to the list of containers to search
        let new_containers_to_search = direct_containers(query, map);
        // Push these new containers onto the stack
        containers_to_search.extend(new_containers_to_search.into_iter());
        // Remember this container that we just searched
        searched_queries.insert(query);
    }
    searched_queries
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_count() {
        // We need to count how many bags could eventually contain a shiny gold bag
        let map = crate::rule_parser(crate::test_input());
        let got = super::find_containers(("shiny", "gold"), &map).len();
        let expected = 4;
        assert_eq!(expected, got);
    }
}
