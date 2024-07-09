use crate::physics::motion::physics;
use crate::physics::rigidbodies::*;
use crate::physics::world::*;

pub fn world_step<T: Updateable>(world: World, _dt: f64) {
    // Getter for time step.
    let dt: f64 = world.get_timestep();

    // Iterates through all objects and matches it to a rigid body for appropriate handling.
    for mut object in world.get_objects() {
        match *object {
            _ => {
                // Updating all relevant fields of the rigid body after the time dt has passed.
                physics::update_velocity::<T>(&mut object, &dt);
                physics::update_position::<T>(&mut object, &dt);
            }
        };
    }
}
