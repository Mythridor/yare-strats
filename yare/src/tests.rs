#[cfg(test)]
mod geo_tests {
    use crate::geometry::{Point, Vector, VectorialCalculus};

    #[test]
    fn vector_definition() {
        let a: Point = Point{ x:2.0, y:3.0};
        let b: Point = Point{ x:10.0, y:5.0};

        assert_eq!(Point::define_vector(a, b), Vector {dx: 8.0, dy:2.0});
    }
}
