
use NEA::collision_definition::*;
use NEA::data_structures::LinearQueue;
use NEA::world::*;

use crate::world::World;

/// Comparing the x and y coordinates of each object in the World with the boundary to see if any object has "crossed" it.
fn detect_world_collision(world: &World) -> LinearQueue<Collision> {
    let collisions: LinearQueue<Collision> = LinearQueue::new();
    let objects = world.get_objects();
    let boundary: Vec<Coordinate> = world.get_boundary();

    /// Iterating through each object.
    for object in objects {
        let position: Coordinate = object.get_position();
        /// Can equally use boundary[1]
        if position[0].abs() >= boundary[0].abs() {
            collisions.enqueue(Collision::ObjWorld(object, world))
        } else {
            continue
        }
        if position[1].abs() >= boundary[1].abs() {
            collisions.enqueue(Collision::ObjWorld(object, world))
        } else {
            continue
        }
    }

    /// Returning all detected collisions with the world.
    collisions
}

/// The `Option` type is an enum consisting of data, or of nothing. 
fn detect_balls_collision(ball1: RigidBody::Ball, ball2: RigidBody::Ball) -> Option<Collision::ObjObj(ball1, ball2)> {
    let centers_of_masses: Vec<Vec<f64>> = vec![ball1.get_position(), ball2.get_position()];
    let position_differential: Vec<f64> = vec![centers_of_masses[0][0] - centers_of_masses[1][0], centers_of_masses[0][1] - centers_of_masses[1][1]];
    
    /// Checking if the distance between the circles is less than the minimum allowed distance.
    if position_differential.magnitude() < ball1.get_radius() + ball2.get_radius() {
        return Collision::ObjObj(ball, ball2)
    }

    /// Satisfying the `Option` return type
    Some(())
}

// SAT collision detection (a known algorithm).
// Meant for non-circular objects
fn sat_collision_detection() {
}

#[cfg(test)]
mod test {
    use crate::collision_detection::detect_world_collision;
    use crate::data_structures::linearqueue::LinearQueue;
    use crate::rigidbodies::{Ball, RigidBody};
    use crate::world::*;

    #[test]
    fn check_boundary_collision() {
        let w = World {
            gravity: (0, -9.81),
            objects: Vec::new(),
            restitution: 1.0,
            boundary: crate::world::Boundary { x_range: (-1.0, 1.0), y_range: (-1.0, 1.0) },
            time: 0,
            dt: 0.1,
        };
        let ball = Ball { mass: 1.0, radius: 0.5, position: (0.0, 0.0), velocity: (1.0, 1.0), acceleration: (0.0, 0.0), angular_velocity: 0.0, parent: RigidBody };
        w.add(ball);

        assert_eq!(detect_world_collision(&w), Some(()))
    }
}