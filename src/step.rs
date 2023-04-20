use crate::motion::{Physics, PhysicsMath};
use crate::rigidbodies::*;
use crate::world::*;

pub fn step<T>(world: World<T>, dt: f64)
where
    T: Updateable + HandleData<T> + AsRef<T>,
{
    let dt: f64 = world.get_timestep();

    for mut object in world.get_objects() {
        match &object {
            Ball => {
                // Physics::update_angular_velocity(&object);
                Physics::update_velocity(&mut object, &dt);
                Physics::update_position(&mut object, &dt);
            }
            _ => (),
        }
    }
}
