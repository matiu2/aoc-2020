use crate::{Space, Spaces};

impl std::str::FromStr for Spaces {
    type Err = parse_display::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out: Self = Default::default();
        for line in s.lines() {
            let row: Result<Vec<Space>, Self::Err> =
                (0..line.len()).map(|i| line[i..i + 1].parse()).collect();
            out.push_row(row?)
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use crate::Space;

    #[test]
    fn test_parser() {
        let input = r#"L.LL.LL.LL
LLL#LLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
        let got = input.parse::<super::Spaces>().expect("Parse failed");
        assert_eq!(got.width(), 10);
        assert_eq!(got.height(), 10);
        assert_eq!(got.get(0, 0), Some(&Space::EmptySeat));
        assert_eq!(got.get(1, 0), Some(&Space::Floor));
        assert_eq!(got.get(3, 1), Some(&Space::OccupiedSeat));
    }
}
