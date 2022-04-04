mod distance;
mod models;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Vector {
    pub dx: i32,
    pub dy: i32
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}


pub trait VectorialCalculus {
    fn define_vector(a: Point, b: Point) -> Vector;
}

impl VectorialCalculus for Point {
    fn define_vector(a: Point, b: Point) -> Vector {
        Vector {dx: b.x - a.x, dy: b.y - a.y}
    }
}
