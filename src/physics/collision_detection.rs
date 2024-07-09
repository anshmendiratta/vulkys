// use crate::rigidbodies::{Updateable, Ball};
// use crate::collision_definition::{Collision, CollisionType};
// use crate::data_structures::linearqueue::LinearQueue
// use crate::world::World;
// use crate::type_traits::VectorMethods;
// use crate::merge_sort::merge_sort;

// type Coordinate = (f64, f64);

// /// Comparing the x and y coordinates of each object in the World with the boundary to see if any object has "crossed" it.
// fn detect_world_collision(world: &World) -> LinearQueue<Collision<Ball>> {
//     let mut collisions: LinearQueue<Collision<Ball>> = LinearQueue::new();
//     let objects = world.get_objects();
//     let boundary_x: Coordinate = world.get_boundary().get_x_range();
//     let boundary_y: Coordinate = world.get_boundary().get_y_range();
//     let time: f64 = world.get_time();

//     // Iterating through each object.
//     for object in objects {
//         let position: Coordinate = object.get_position();
//         // Can equally use boundary[1]
//         if position.0.abs() >= boundary_x.0.abs() {
//             collisions.enqueue(Collision { objects: CollisionType::ObjWorld(object, world), time: time })
//         } else if position.1.abs() >= boundary_y.1.abs() {
//             collisions.enqueue(Collision { objects: CollisionType::ObjWorld(object, world), time: time })
//         } else {
//             continue
//         }
//     }

//     // Returning all detected collisions with the world.
//     collisions
// }

// /// The `Option` type is an enum consisting of data, or of nothing.
// fn detect_balls_collision<T>(ball1: T, ball2: T) -> Option<Collision<T>>
//     where T: Updateable
//     {
//     let centers_of_masses: Vec<Coordinate> = vec![ball1.get_position(), ball2.get_position()];
//     let position_differential: Vec<f64> = vec![centers_of_masses[0].0 - centers_of_masses[1].0, centers_of_masses[0].1 - centers_of_masses[1].1];

//     // Checking if the distance between the circles is less than the minimum allowed distance.
//     if position_differential.magnitude() < ball1.get_radius() + ball2.get_radius() {
//         return Collision::ObjObj(ball1, ball2)
//     } else { return Option::None }

//     // Satisfying the `Option` return type
//     Option::None
// }

// // SAT collision detection (a known algorithm).
// // Meant for non-circular objects
// fn sat_collision_detection() {
// }

// fn sort_collisions<T>(collisions: LinearQueue<Collision<T>>) where T: Updateable {
//     let mut collisions_list = [0; collisions.len().clone()];
//     for collision in collisions {
//         collisions_list.append(collision)
//     }
//     merge_sort(collisions_list);
//     collisions_list

// }

// #[cfg(test)]
// mod tests {
//     use crate::rigidbodies::{Ball, RigidBody};
//     use crate::world::*;

//     #[test]
//     fn check_boundary_collision() {
//         let mut w = World {
//             gravity: (0.0, -9.81),
//             objects: Vec::new(),
//             restitution: 1.0,
//             boundary: crate::world::Boundary { x_range: (-1.0, 1.0), y_range: (-1.0, 1.0) },
//             time: 0.0,
//             dt: 0.1,
//         };
//         let ball = Ball { mass: 1.0, radius: 0.5, position: (0.0, 0.0), velocity: (1.0, 1.0), acceleration: (0.0, 0.0), angular_velocity: 0.0, parent: RigidBody {
//             mass: 1.0,
//             position: (0.0, 0.0),
//             velocity: (0.0, 0.0)
//         } };
//         w.add(ball);

//         assert_eq!(detect_world_collision(&w), None)
//     }
// }
