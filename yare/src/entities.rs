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
    fn move_to(&self, vector: Vector);
}

