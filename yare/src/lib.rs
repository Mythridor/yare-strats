mod geometry;

#[cfg(test)]
mod tests {
    use crate::geometry::{self, Point, Vector, VectorialCalculus};

    #[test]
    fn vector_definition() {
        let a: Point = Point{ x:2, y:3};
        let b: Point = Point{ x:10, y:5};

        assert_eq!(Point::define_vector(a, b), Vector {dx: 8, dy:2});
    }
}
