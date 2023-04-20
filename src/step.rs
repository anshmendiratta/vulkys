use crate::motion::{Physics, PhysicsMath};
use crate::world::*;
use crate::rigidbodies::*;

pub fn step<T: Updateable + HandleData<T>>(world: World<T>, dt: f64) {
    let dt: f64 = world.get_timestep();

    for object in world.get_objects() {
        match object {
            RigidBody => (),
            Ball => {
                Physics::update_angular_velocity(object);
                Physics::update_velocity(object, dt);
                Physics::update_position(object, dt);
            }
        }
    }
}
