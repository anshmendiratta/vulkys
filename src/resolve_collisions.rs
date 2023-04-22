use crate::collision_definition::*;
use crate::data_structures::linearqueue::*;
use crate::rigidbodies::*;

fn resolve_collision<T>(collisions: LinearQueue<Collision<T>>)
where
    T: Updateable + AsRef<T>,
{
    for collision in collisions {
        match collision.get_objects() {
            (_body1, _World) => resolve_boundary_collision(collision),
            (_body1, _body2) => resolve_object_collision(collision),
        }
    }
}

fn resolve_boundary_collision<T>(_collision: Collision<T>)
where
    T: Updateable + AsRef<T>,
{
    // (xord, yord) = collision.get;
}

fn resolve_object_collision<T>(_collision: Collision<T>)
where
    T: Updateable + AsRef<T>,
{
}

fn update_velocity<T>(_object1: T, _object2: T)
where
    T: Updateable + AsRef<T>,
{
    // object1.velocity = (2 * object2.get_velocity().scale_vector(*object2.get_mass()) + *object1.get_velocity().scale(object1.get_mass() - object2.get_mass())).scale(object1.get_mass() + object2.get_mass());
    // object2.velocity = (2 * object1.get_velocity().scale_vector(*object1.get_mass()) + *(object2.get_velocity()).scale(object2.get_mass() - object1.get_mass())).scale(object1.get_mass() + object2.get_mass());
}

fn update_angular_velocity(_object1: &RigidBody, _object2: &RigidBody) {
    // object1.angular_velocity =
}
