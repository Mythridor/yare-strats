pub enum Structure {
    Star,
    Outpost,
    Base,
}


//TODO: Implement function injection as behaviour selector during match
enum SpiritSpecialty {
    Soldier,      //Soldiers defends and attacks at will
    Harvester,    //Harvesters have basic implementation of energy harvesting
    Provisionner, //Provisionners convoys energy from one source to a destination
}