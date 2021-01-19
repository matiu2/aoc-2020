use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
pub enum Space {
    #[display("#")]
    OccupiedSeat,
    #[display("L")]
    EmptySeat,
    #[display(".")]
    Floor,
}

#[derive(Default, Debug)]
pub struct Spaces {
    // Data is stored [row,row,row]
    // So to access coordinate (x,y), index is y * width + x
    // So to access coordinate (col, row), index is row * width + col
    // eg. col 3 row 0 = index 3
    // eg. col 3 row 2 = index 2 * width + 3
    data: Vec<Space>,
    width: usize,
    height: usize,
}

impl Spaces {
    pub fn push_row(&mut self, row: Vec<Space>) {
        if self.width == 0 && self.data.is_empty() {
            // If this is the first push, get our row size
            self.width = row.len();
        }
        self.height += 1;
        self.data.extend(row);
    }

    /// Retrieves the space at column x and row y, if any
    pub fn get(&self, x: usize, y: usize) -> Option<&Space> {
        self.data.get(y * self.width + x)
    }

    /// Read-only width - set it using FromStr when you build this struct
    pub fn width(&self) -> usize {
        self.width
    }

    /// Read-only width - set it using FromStr when you build this struct
    pub fn height(&self) -> usize {
        self.height
    }
}
