use crate::type_traits::*;
// use crate::vector_transform; 
use crate::rigidbodies::*;
use crate::data_structures::linearqueue::*;
use crate::collision_definition::*;

fn resolve_collision(collisions: LinearQueue<Collision>) {
    for collision in collisions {
        match collision.get_kind() {
            boundary => resolve_boundary_collision(collision.get_participants()),
            object => resolve_object_collision()
        }
    }
}

fn resolve_boundary_collision(collision: Collision) {
    // (xord, yord) = collision.get;
}

fn resolve_object_collision() {

}

fn update_velocity(mut object1: &RigidBody, mut object2: &RigidBody) {
    object1.velocity = (2 * object2.get_velocity().scale_vector(object2.get_mass()) + object1.get_velocity().scale(object1.get_mass() - object2.get_mass())).scale(object1.get_mass() + object2.get_mass());
    object2.velocity = (2 * object1.get_velocity().scale_vector(object1.get_mass()) + object2.get_velocity().scale(object2.get_mass() - object1.get_mass())).scale(object1.get_mass() + object2.get_mass());
}

fn update_angular_velocity(mut object1: &RigidBody, mut object2: &RigidBody) {
    // object1.angular_velocity = 
}