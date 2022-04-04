use yareio::spirit::{position, energy};
use crate::geometry::models::SpiritSpecialty;
use crate::geometry::Vector;

struct Spirit {
    specialty: SpiritSpecialty,
    index: usized,
    alive: bool,
    friendly: bool,
    energy: energy,
    position: position,
}

trait Behave {
    fn get_all() -> Vec<Spirit>;
    fn goto(&self, vector: Vector, distance: f32);
    fn behave();
}

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
                    energy: spirit::energy(index),
                    position: spirit::position(index),
                    alive: spirit::hp(index) > 0,
                    friendly: spirit::id(index).number == me
                });
            }
            spirits
        }
    }

    fn goto(&self, vector: Vector, distance: f32) {
        let mut to_x: f32;
        let mut to_y: f32;

        unsafe {

        
            let pos = spirit::position(self.index);
            let vector = Point::unitirize_vector(vector);
            to_x = pos.x + vector;
            spirit::goto(self.index, to_x, to_y)
        }
    }

    fn behave() {
    }
}

