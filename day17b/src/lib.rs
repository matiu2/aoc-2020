/// A point in space
pub struct Point {
    /// In order, x,y,z,w, etc..
    coordinates: Vec<i64>,
}

impl Point {
    /// Returns true if we're next to this other point
    pub fn is_neighbour(&self, other: &Point) -> bool {
        // If the euclidean distance is 1, this is a neighbour
        // Euclidean distance is the square root of the sum of the squares of
        // the distances on each dimension
        assert_eq!(self.coordinates.len(), other.coordinates.len());
        let distance = self
            .coordinates
            .iter()
            .zip(other.coordinates.iter())
            // Get the distance squared for each coordinate (x,y,z, etc.)
            .map(|(a, b)| (*a - b).pow(2) as f32)
            // Sum them
            .sum::<f32>()
            // Take the square root of that
            .sqrt();
        distance == 1.0
    }
}

#[cfg(test)]
mod test {
    use super::Point;

    macro_rules! point {
        ($($x:expr),+) => {
            Point{coordinates: vec! {$($x),+}}
        };
    }

    #[test]
    fn test_is_neighbour_1_dim() {
        // 1 dimensional
        let a = point!(4);
        let b = point!(5);
        assert!(a.is_neighbour(&b));
        assert!(b.is_neighbour(&a));
        // 1 dim same point = not neighbour
        let b = point!(4);
        assert!(!a.is_neighbour(&b));
        assert!(!b.is_neighbour(&a));
    }
}
