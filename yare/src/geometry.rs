#[derive(Debug)]
#[derive(PartialEq)]
pub struct Vector {
    pub dx: i32,
    pub dy: i32
}

pub struct Point {
    pub x: i32,
    pub y: i32
}


pub trait VectorialCalculus {
    fn define_vector(A: Point, B: Point) -> Vector;
}

impl VectorialCalculus for Point {
    fn define_vector(A: Point, B: Point) -> Vector {
        let dx = B.x - A.x;
        let dy = B.y - A.y;
        Vector {dx, dy}
    }
}
