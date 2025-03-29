use std::ops::Add;
use serde::{Deserialize, Serialize};

use crate::Orientation;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Connector {
    Axle,
    Pin,
    Beam,
    Gear,
    Plate,
    Brick,
    TechnicBrick,
    TechnicLiftarm,
    TechnicPanel,
    TechnicConnector,
}


/// A LEGO interface (stud, tube, axle hole, etc.)
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ConnectorInterface {
    Stud,
    Tube,
    Axle,
    AxleHole,
    Pin,
    PinHole,
    BallJoint,
    BallSocket,
    Bar,
    Clip,
    StudReceptor,
    AntiStud,
    Hook,
    Hinge,
    FlexibleTube,
}

impl ConnectorInterface {
    /// If this interface has a "natural inverse," return it. Otherwise, None.
    pub fn inverse(&self) -> Option<Self> {
        use ConnectorInterface::*;
        match self {
            Stud => Some(Tube),
            Tube => Some(Stud),
            Axle => Some(AxleHole),
            AxleHole => Some(Axle),
            Pin => Some(PinHole),
            PinHole => Some(Pin),
            _ => None,
        }
    }
}

/// An oriented interface: The interface + how it’s rotated/flipped.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct OrientedInterface {
    pub interface: ConnectorInterface,
    pub orientation: Orientation,
}

/// A connection is (Interface1, Option<Connector>, Interface2) 
/// with orientation info included for extra realism.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct OrientedConnection {
    pub left: OrientedInterface,
    pub connector: Option<Connector>,
    pub right: OrientedInterface,
}

impl Add for OrientedInterface {
    type Output = Option<OrientedConnection>;

    fn add(self, rhs: Self) -> Self::Output {
        
        use Connector::*;
        
        // For brevity, ignore orientation constraints in this example:
        // but you *could* check self.orientation.compose(rhs.orientation) 
        // to see if alignment is valid.

        // Helper macro to build an OrientedConnection
        macro_rules! connection {
            ($c:expr) => {
                Some(OrientedConnection {
                    left: self,
                    connector: Some($c),
                    right: rhs,
                })
            };
        }

        match (self.interface, rhs.interface) {
            // Stud + Tube (or Tube + Stud) => Brick
            (ConnectorInterface::Stud, ConnectorInterface::Tube) | 
            (ConnectorInterface::Tube, ConnectorInterface::Stud) => connection!(Connector::Brick),
            
            // Axle + AxleHole => Axle
            (ConnectorInterface::Axle, ConnectorInterface::AxleHole) | 
            (ConnectorInterface::AxleHole, ConnectorInterface::Axle) => connection!(Connector::Axle),
            
            // Pin + PinHole => Pin
            (ConnectorInterface::Pin, ConnectorInterface::PinHole) | 
            (ConnectorInterface::PinHole, ConnectorInterface::Pin) => connection!(Connector::Pin),

            // BallJoint + BallSocket => TechnicConnector
            (ConnectorInterface::BallJoint, ConnectorInterface::BallSocket) | 
            (ConnectorInterface::BallSocket, ConnectorInterface::BallJoint) => connection!(Connector::TechnicConnector),
            
            // Bar + Clip => TechnicConnector
            (ConnectorInterface::Bar, ConnectorInterface::Clip) | 
            (ConnectorInterface::Clip, ConnectorInterface::Bar) => connection!(TechnicConnector),
            
            // etc...
            _ => None,
        }
    }
}

// Convenience method
impl OrientedInterface {
    pub fn connect(self, other: OrientedInterface) -> Option<OrientedConnection> {
        self + other
    }
}



