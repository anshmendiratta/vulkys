use crate::motion::{Physics};
use crate::rigidbodies::*;
use crate::world::*;
use crate::type_traits;

pub fn world_step<T: Updateable>(world: World, dt: f64) {
    let dt: f64 = world.get_timestep();

    for mut object in world.get_objects() {
        match object {
            ref Ball => {
                // Physics::update_angular_velocity(&object);
                Physics::update_velocity::<T>(&mut object, &dt);
                // Physics::update_position(Box::leak(object), &dt);
            }
            _ => (),
        }
    }
}
