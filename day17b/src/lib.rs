/// A point in space
#[derive(Debug)]
pub struct Point<const N: usize> {
    /// In order, x,y,z,w, etc..
    coordinates: [i64; N],
}

impl<const N: usize> Point<N> {
    pub fn new(coordinates: [i64; N]) -> Point<N> {
        Point { coordinates }
    }
    /// Returns true if we're next to this other point
    pub fn is_neighbour(&self, other: &Point<N>) -> bool {
        assert_eq!(self.coordinates.len(), other.coordinates.len());
        // Get the distance to travel on each dimesnion
        let mut one_count = 0;
        for distance in self
            .coordinates
            .iter()
            .zip(other.coordinates.iter())
            // Count
            // Get the distance squared for each coordinate (x,y,z, etc.)
            .map(|(a, b)| (a - b).abs())
        {
            if distance == 1 {
                // We need at least one coordinate to be
                one_count += 1;
            } else if distance > 1 {
                // If any dimensions are > 1 away, not neighbours
                return false;
            };
        }
        // We need at least one dimension to travel along, otherwise it's duplicate point
        one_count > 0
    }
}

#[cfg(test)]
mod test {
    use super::Point;

    macro_rules! point {
        ($($x:expr),+) => {
            Point::new([$($x),+])
        };
    }

    #[test]
    fn test_is_neighbour_1_dim() {
        // 1 dimensional
        let a = point!(4);
        let b = point!(5);
        assert!(a.is_neighbour(&b));
        assert!(b.is_neighbour(&a));
        // Same point = not neighbour
        let b = point!(4);
        assert!(!a.is_neighbour(&b));
        assert!(!b.is_neighbour(&a));
    }

    #[test]
    fn test_is_neighbour_2_dim() {
        // 2 dimensional
        let a = point!(4, 4);
        let b = point!(4, 5);
        assert!(a.is_neighbour(&b));
        assert!(b.is_neighbour(&a));
        assert!(a.is_neighbour(&point!(3, 3)));
        assert!(a.is_neighbour(&point!(3, 4)));
        assert!(a.is_neighbour(&point!(4, 3)));
        assert!(a.is_neighbour(&point!(5, 5)));
        assert!(a.is_neighbour(&point!(5, 3)));
        assert!(a.is_neighbour(&point!(4, 5)));
        // Same point = not neighbour
        assert!(!a.is_neighbour(&point!(4, 4)));
        // negatives
        assert!(!a.is_neighbour(&point!(-3, 4)));
        assert!(!a.is_neighbour(&point!(4, -3)));
        // A bit too far
        assert!(!a.is_neighbour(&point!(6, 5)));
        assert!(!a.is_neighbour(&point!(5, 2)));
        assert!(!a.is_neighbour(&point!(4, 6)));
    }

    #[test]
    fn test_is_neighbour_3_dim() {
        // 2 dimensional
        let a = point!(4, 4, 4);
        let b = point!(4, 5, 3);
        assert!(a.is_neighbour(&b));
        assert!(b.is_neighbour(&a));
        assert!(a.is_neighbour(&point!(3, 3, 3)));
        assert!(a.is_neighbour(&point!(3, 4, 3)));
        assert!(a.is_neighbour(&point!(4, 3, 4)));
        assert!(a.is_neighbour(&point!(5, 5, 3)));
        assert!(a.is_neighbour(&point!(5, 3, 4)));
        assert!(a.is_neighbour(&point!(4, 5, 4)));
        // Same point = not neighbour
        assert!(!a.is_neighbour(&point!(4, 4, 4)));
        // negatives
        assert!(!a.is_neighbour(&point!(-3, 4, 4)));
        assert!(!a.is_neighbour(&point!(4, -3, 4)));
        // A bit too far
        assert!(!a.is_neighbour(&point!(6, 5, 4)));
        assert!(!a.is_neighbour(&point!(5, 2, 4)));
        assert!(!a.is_neighbour(&point!(4, 4, 2)));
        assert!(!a.is_neighbour(&point!(4, 4, 6)));
    }
}
