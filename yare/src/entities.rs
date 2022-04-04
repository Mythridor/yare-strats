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
}

