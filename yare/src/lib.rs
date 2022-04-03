mod geometry;

#[cfg(test)]
mod tests {
    use crate::geometry::{self, VectorialCalculus};

    #[test]
    fn vector_definition() {
        let a: geometry::Point = geometry::Point{ x:2, y:3};
        let b: geometry::Point = geometry::Point{ x:10, y:5};

        assert_eq!(geometry::Point::define_vector(a, b), geometry::Vector {dx: 8, dy:2});
    }
}
