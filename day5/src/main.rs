struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    /// Create a new seat, reading in a seat specification line
    fn new(id: &str) -> Seat {
        let mut row = (0, 127);
        let mut col = (0, 7);
        id.chars().for_each(|c| {
            match c {
                'F' => row.1 = row.0 + (row.1 - row.0) / 2,
                'B' => row.0 += 1 + (row.1 - row.0) / 2,
                'L' => col.1 = col.0 + (col.1 - col.0) / 2,
                'R' => col.0 += 1 + (col.1 - col.0) / 2,
                other => panic!("Got unexpected char: {}", other),
            };
        });
        // By the time we've parsed the seat number, we should be down to a single number
        assert_eq!(row.0, row.1);
        assert_eq!(col.0, col.1);
        Seat {
            row: row.0,
            col: col.0,
        }
    }
    /// The ID of the seat
    fn id(&self) -> usize {
        self.row * 8 + self.col
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input.txt");
    let mut seat_ids: Vec<usize> = input.lines().map(Seat::new).map(|seat| seat.id()).collect();
    let max_seat_id = seat_ids.iter().max().expect("No data lines found");
    println!("Day 5 - Part 1: highest seat id: {}", max_seat_id);
    // Part 2 - find our seat
    // Find all the rows (excluding the first and last row)
    seat_ids.sort();
    // Our seat will be the one missing number
    let mut iter = seat_ids
        .iter()
        // We ignore the first and last rows
        // First row IDs are 0..7
        .skip_while(|&&id| id <= 7)
        // Ignore the last row IDs are (127 * 8)..(127 * 8 + 7) (1016..1024)
        .take_while(|&&id| id < 1016);
    // Predict the next seat id
    let mut last_id = *iter.next().expect("Not enough data");
    // Find the missing ID (our seat id)
    let mut my_seat_id = None;
    for &id in iter {
        dbg!(id, last_id + 2, last_id);
        // If this chair has the ID of the last chair, plus 2,
        // They skipped over my chair
        if id == last_id + 2 {
            my_seat_id = Some(last_id + 1);
            break;
        }
        last_id = id
    }
    if let Some(my_seat_id) = my_seat_id {
        println!("My seat ID: {}", my_seat_id);
    } else {
        println!("Couldn't find a missing ID");
    }
}

#[cfg(test)]
mod tests {
    use super::Seat;

    #[test]
    fn test1() {
        let input = "FBFBBFFRLR";
        let seat = Seat::new(input);
        assert_eq!(seat.row, 44);
        assert_eq!(seat.col, 5);
        assert_eq!(seat.id(), 357);
    }

    #[test]
    fn test2() {
        let input = "BFFFBBFRRR";
        let seat = Seat::new(input);
        assert_eq!(seat.row, 70);
        assert_eq!(seat.col, 7);
        assert_eq!(seat.id(), 567);
    }

    #[test]
    fn test3() {
        let input = "FFFBBBFRRR";
        let seat = Seat::new(input);
        assert_eq!(seat.row, 14);
        assert_eq!(seat.col, 7);
        assert_eq!(seat.id(), 119);
    }

    #[test]
    fn test4() {
        let input = "BBFFBBFRLL";
        let seat = Seat::new(input);
        assert_eq!(seat.row, 102);
        assert_eq!(seat.col, 4);
        assert_eq!(seat.id(), 820);
    }
}
