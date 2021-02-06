mod parse;
#[derive(Debug, PartialEq, Eq)]
pub struct Space {
    /// All the x values of active blocks
    x: Vec<i64>,
    /// All the y values of active blocks
    y: Vec<i64>,
    /// All the z values of active blocks
    z: Vec<i64>,
}
