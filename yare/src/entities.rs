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
    fn goto(&self, vector: Vector, distance: f32);
    fn goto_point(self, to: Point);
    fn behave();
}


//TODO : Implement shape when not using 'Circle only' rule anymore
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
                    shape: SpiritShape::Circle,
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

    fn goto(&self, vector: Vector, distance: f32) {
        let mut to_x: f32;
        let mut to_y: f32;

        self.status = SpiritStatus::InTransit;

        unsafe {        
            let pos = spirit::position(self.index);
            let vector = Point::unitirize_vector(vector);
            to_x = pos.x + vector;
            spirit::goto(self.index, to_x, to_y)
        }

        self.status = SpiritStatus::Available;
    }

    fn goto_point(&self, to: Point) {

    }

    fn behave() {
    }
}

