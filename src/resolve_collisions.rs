use crate::rigidbodies::*;
use crate::data_structures::linearqueue::*;
use crate::collision_definition::*;

fn resolve_collision<T: Updateable + HandleData<T>>(collisions: LinearQueue<Collision<T>>) {
    for collision in collisions {
        match collision.get_objects() {
            (body1, World) => resolve_boundary_collision(collision),
            (body1, body2) => resolve_object_collision(collision)
        }
    }
}
  
fn resolve_boundary_collision<T: Updateable + HandleData<T>>(collision: Collision<T>) {
    // (xord, yord) = collision.get;
}

fn resolve_object_collision<T: Updateable + HandleData<T>>(collision: Collision<T>) {

}

fn update_velocity<T: Updateable + HandleData<T>>(mut object1: T, mut object2: T) {
    // object1.velocity = (2 * object2.get_velocity().scale_vector(*object2.get_mass()) + *object1.get_velocity().scale(object1.get_mass() - object2.get_mass())).scale(object1.get_mass() + object2.get_mass());
    // object2.velocity = (2 * object1.get_velocity().scale_vector(*object1.get_mass()) + *(object2.get_velocity()).scale(object2.get_mass() - object1.get_mass())).scale(object1.get_mass() + object2.get_mass());
}

fn update_angular_velocity(mut object1: &RigidBody, mut object2: &RigidBody) {
    // object1.angular_velocity = 
}