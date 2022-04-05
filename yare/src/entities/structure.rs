use yareio::position::Position;
use std::collections::HashMap;

pub struct Structure {
    pub index: usize,
    pub position: Position,
    pub energy: i32,
    structure_type: StructureType,
    pub energy_capacity: Option<i32>,
    pub current_spirit_cost: Option<i32>,
    pub alive: Option<bool>,
    pub player_id: Option<usize>,
    pub range: Option<u32>,
}

impl Structure {
    pub fn get_all() -> HashMap<StructureType, Vec<Structure>> {
        let mut structures: HashMap<StructureType, Vec<Structure>>= HashMap::new();
        let mut bases: Vec<Structure> = Vec::new();
        let mut stars: Vec<Structure> = Vec::new();
        let mut Outpost: Vec<Structure> = Vec::new();

        return structures
    }
}
#[derive(PartialEq)]
pub enum StructureType {
    Star,
    Outpost,
    Base,
}
