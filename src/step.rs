use crate::motion::{Physics, PhysicsMath};
use crate::world::*;
use crate::rigidbodies::*;

pub fn step(world: World, dt: f64) {
    for object in world.get_objects() {
        object.velocity = Physics::update_velocity(object.get_velocity, dt);
        object.position = Physics::update_position(object.get_position, dt);
        object.angular_velocity = Physics::update_angular_velocity(object.get_angular_velocity)
    }
}
