use crate::motion::{Physics, PhysicsMath};
use crate::world::*;
use crate::rigidbodies::*;

pub fn step<T: HandleData>(world: World, dt: f64) {
    for object in *world.get_objects() {
        match object {
            RigidBody::None => (),
            _ => {
                object.velocity = Physics::update_velocity(object.clone(), dt);
                object.position = Physics::update_position(object.clone(), dt);
                object.angular_velocity = Physics::update_angular_velocity(object.clone().get_angular_velocity());
            }
        }
    }
}
