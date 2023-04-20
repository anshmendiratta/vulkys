use crate::motion::{Physics};
use crate::rigidbodies::*;
use crate::world::*;

pub fn world_step<T>(world: World, _dt: f64)
where
    T: Updateable + HandleData<T>,
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
