use yareio::spirit;
use yareio::player;
use yareio::position::Position;

use crate::geometry::models::{SpiritStatus, SpiritSpecialty, SpiritShape};
use crate::geometry::{Vector, Point, VectorialCalculus};

struct Spirit {
    specialty: SpiritSpecialty,
    shape: SpiritShape,
    index: usize,
    alive: bool,
    friendly: bool,
    energy: i32,
    position: Position,
    status: SpiritStatus
}

trait Behave {
    fn get_all() -> Vec<Spirit>;
    fn goto(&mut self, vector: Vector, distance: f32);
    fn goto_point(&mut self, to: Point);
    fn behave(spirits: Vec<Spirit>);
}


//TODO : Implement Triangle shape ?
impl Behave for Spirit {
    fn get_all() -> Vec<Spirit> {
        unsafe {
            let me = player::me();
            let count = spirit::count();
            let mut spirits = Vec::with_capacity(count);
            for index in 0..count {
                spirits.push(Spirit {
                    index,
                    specialty: SpiritSpecialty::Harvester,
                    shape: if spirit::shape(index) == 0 {
                            SpiritShape::Circle
                      } else if spirit::shape(index) == 1 {
                            SpiritShape::Square
                      } else if spirit::shape(index) == 2 {
                            SpiritShape::Triangle
                      } else {
                            SpiritShape::Unknown
                      },
                    energy: spirit::energy(index),
                    position: spirit::position(index),
                    alive: spirit::hp(index) > 0,
                    friendly: spirit::id(index).number == me,
                    status: SpiritStatus::Available
                });
            }
            spirits
        }
    }
    #[warn(unused_variables)]
    fn goto(&mut self, vector: Vector, distance: f32) {
        let to_x: f32;
        let to_y: f32;

        self.status = SpiritStatus::InTransit;

        unsafe {        
            let pos = spirit::position(self.index);
            let vector = Point::normalize(vector);
            to_x = pos.x + vector.dx;
            to_y = pos.y + vector.dy;
            spirit::goto(self.index, to_x, to_y)
        }

        self.status = SpiritStatus::Available;
    }

    fn goto_point(&mut self, to: Point) {

    }

    fn behave(spirits: Vec<Spirit>) {
        for spirit in spirits {
            match spirit.specialty {
                SpiritSpecialty::Harvester => (),
                SpiritSpecialty::Provisionner => (),
                SpiritSpecialty::Soldier => ()
            }
        }
    }
}

