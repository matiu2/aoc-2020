mod parser;
use std::collections::HashMap;

pub use parser::rule_parser;

mod find_containers;
pub use find_containers::find_containers;

// The adjective and colour of a bag (that we want more info about)
pub type BagID<'a> = (&'a str, &'a str);
// Tells us how many of a certain bag (are contained in another bag)
pub type Contents<'a> = HashMap<BagID<'a>, usize>;
// Tells us how many of any other kind of bag are contained in a top level bag
pub type Containers<'a> = HashMap<BagID<'a>, Contents<'a>>;
