mod distance;
pub mod models;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Vector {
    pub dx: f32,
    pub dy: f32
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32
}


pub trait VectorialCalculus {
    fn define_vector(a: Point, b: Point) -> Vector;
    fn normalize(vector: Vector) -> Vector;
}

impl VectorialCalculus for Point {
    fn define_vector(a: Point, b: Point) -> Vector {
        Vector {dx: b.x - a.x, dy: b.y - a.y}
    }

    fn normalize(vector: Vector) -> Vector {
        vector
    }
}
