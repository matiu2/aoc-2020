mod parser;
use std::collections::HashMap;

pub use parser::rule_parser;

mod find_containers;
pub use find_containers::find_containers;

mod find_contents;
pub use find_contents::count_content_recursive;

// The adjective and colour of a bag (that we want more info about)
pub type BagID<'a> = (&'a str, &'a str);
// Tells us how many of a certain bag (are contained in another bag)
pub type Contents<'a> = HashMap<BagID<'a>, usize>;
// Tells us how many of any other kind of bag are contained in a top level bag
pub type Containers<'a> = HashMap<BagID<'a>, Contents<'a>>;

#[cfg(test)]
pub fn test_input() -> &'static str {
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
