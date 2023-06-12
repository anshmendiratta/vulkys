use crate::motion::{Physics};
use crate::rigidbodies::*;
use crate::world::*;


pub fn world_step<T: Updateable>(world: World, _dt: f64) {
    // Getter for time step.
    let dt: f64 = world.get_timestep();

    // Iterates through all objects and matches it to a rigid body for appropriate handling.
    for mut object in world.get_objects() {
        match object {
            // ref prevents a move from occuring.
            ref _Ball => {
                // Updating all relevant fields of the rigid body after the time dt has passed.
                Physics::update_velocity::<T>(&mut object, &dt);
                Physics::update_position::<T>(object, &dt);
            }
            // "All other rigidbodies are not treated"
            _ => (),
        }
    }
}


