use crate::motion::{Physics, PhysicsMath};
use crate::world::*;
use crate::rigidbodies::*;

pub fn step<T: HandleData>(world: World, dt: f64) {
    for object in *world.get_objects() {
        match object {
            RigidBody::None => (),
            RigidBody::Ball { .. } => {
                Physics::update_position(object, dt);
                Physics::update_velocity(object, dt);
                Physics::update_angular_velocity(object.get_angular_velocity());
            }
        }
    }
}
