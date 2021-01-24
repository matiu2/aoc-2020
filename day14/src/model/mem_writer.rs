mod parse;
/// Writes memory to a location
#[derive(PartialEq, Eq, Debug)]
pub struct MemWriter {
    pub location: usize,
    pub value: usize,
}
