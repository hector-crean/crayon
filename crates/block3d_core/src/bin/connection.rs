use block3d_core::connection::{ConnectorInterface, OrientedInterface};
use block3d_core::Orientation;

fn main() {
    use ConnectorInterface::*;
    


    // Example: create two oriented interfaces
    let oriented_stud = OrientedInterface { interface: Stud, orientation: Orientation::O0 };
    let oriented_tube = OrientedInterface { interface: Tube, orientation: Orientation::O180 };

    if let Some(conn) = oriented_stud.connect(oriented_tube) {
        println!("Connection found: {:?}", conn);
    } else {
        println!("No valid connection!");
    }

    // Another example:
    let oriented_axle = OrientedInterface { interface: Axle, orientation: Orientation::O90 };
    let oriented_axlehole = OrientedInterface { interface: AxleHole, orientation: Orientation::O270 };
    
    if let Some(conn) = oriented_axle + oriented_axlehole {
        println!("Axle -> AxleHole works: {:?}", conn);
    }
}