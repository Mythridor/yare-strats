use crate::geometry::Point;
use crate::geometry::models::StructureType;

pub trait Distance {

    /// Get the distance betwenn two points.
    ///
    /// Given that the destination point is a Point. 
    /// It.calculate distance between two points.
    fn calculate_distance(self, to: Point) -> u32;

    /// Get the closest Structure.
    ///
    /// Given that the destination point is a structure defined in the enum and that it returns the closest one. 
    /// A match will to behave accordingly to the nature of the structure.
    fn define_closest_structure() -> StructureType;


}