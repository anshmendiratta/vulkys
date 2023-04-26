use crate::collision_definition::*;
use crate::data_structures::linearqueue::*;
use crate::rigidbodies::*;
use crate::world::World;

/// A parent function that branches off into two functions depending on the collisio type.
fn resolve_collision<T>(collisions: LinearQueue<Collision<T>>, true_world: World)
where
    T: Updateable + AsRef<T> + Copy,
{
    // Iterating through all detected collisions, of either type.
    for collision in collisions {
        // Matching the collision type with the possible options.
        match collision.get_objects() {
            (body1, _world) => resolve_boundary_collision(*body1, &true_world),
            // (body1, body2) => resolve_object_collision::(body1, body2.unwrap()),
        }
    }
}

/// The reflection of the velocity depending on the boundary collided with.
fn resolve_boundary_collision<T>(mut object: T, world: &World)
where
    T: Updateable + AsRef<T>,
{
    let boundary = world.get_boundary();
    let position = object.get_position();
    let mut velocity = object.get_velocity();

    // Checking if the x position is outside the vertical boundaries on the left and the right.
    if position.0.abs() >= boundary.x_range.0.abs() {
        velocity.0 *= -1.0
    } 
    
    // Checking if the y position is outside the vertical boundaries on the top and the bottom..
    if position.0.abs() >= boundary.y_range.0.abs() {
        velocity.1 *= -1.0
    } 

    object.set_velocity(velocity)
}

/// Dealing with two objects colliding. 
fn resolve_object_collision<T>(_body1: T, _body2: T)
where
    T: Updateable + AsRef<T>,
{
}
