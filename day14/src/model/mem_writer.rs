mod parse;
/// Writes memory to a location
#[derive(PartialEq, Eq, Debug)]
pub struct MemWriter {
    location: usize,
    value: usize,
}
