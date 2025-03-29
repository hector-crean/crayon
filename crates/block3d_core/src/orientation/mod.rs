use serde::{Serialize, Deserialize};
use strum::EnumIter;



/// A partial orientation group with 4 discrete states for demonstration.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Default, Hash, EnumIter)]
pub enum Orientation {
    #[default]
    O0,
    O90,
    O180,
    O270,
}

impl Orientation {
    pub fn compose(self, other: Orientation) -> Orientation {
        // Just an example of group-like composition
        let sum = (self.as_degrees() + other.as_degrees()) % 360;
        Self::from_degrees(sum)
    }

    fn as_degrees(self) -> u16 {
        match self {
            Orientation::O0 => 0,
            Orientation::O90 => 90,
            Orientation::O180 => 180,
            Orientation::O270 => 270,
        }
    }

    fn from_degrees(d: u16) -> Orientation {
        match d {
            0   => Orientation::O0,
            90  => Orientation::O90,
            180 => Orientation::O180,
            270 => Orientation::O270,
            _   => panic!("Invalid discrete orientation"),
        }
    }

    

   
}




