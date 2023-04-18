use crate::motion::{Physics, PhysicsMath};
use crate::world::*;
use crate::rigidbodies::*;

pub fn step(world: World, dt: f64) {
    for object in world.get_objects() {
        match object {
            RigidBody::Ball { mass, radius, position, velocity, angular_velocity } => {
                object.velocity = Physics::update_velocity(object.clone(), dt);
                object.position = Physics::update_position(object.clone(), dt);
                object.angular_velocity = Physics::update_angular_velocity(object.clone().get_angular_velocity())
            },
            _ => (),
        }
    }
}
