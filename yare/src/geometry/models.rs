pub enum StructureType {
    Star,
    Outpost,
    Base,
}


//TODO: Implement function injection as behaviour selector during match
pub enum SpiritSpecialty {
    Soldier,      //Soldiers defends and attacks at will
    Harvester,    //Harvesters have basic implementation of energy harvesting
    Provisionner, //Provisionners convoys energy from one source to a destination
}

pub enum SpiritStatus {
    InTransit,
    Available
}

pub enum SpiritShape {
    Circle,
    Triangle,
    Square,
    Unknown
}