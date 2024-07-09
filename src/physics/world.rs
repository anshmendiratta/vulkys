use crate::physics::rigidbodies::Updateable;
use crate::physics::step::world_step;

/// Using Rust's derive macro to add a default state to world if it was just initialized without the relevant fields filled in.
pub struct World {
    pub gravity: (f64, f64),
    pub objects: Vec<Box<dyn Updateable>>,
    pub restitution: f64,
    pub boundary: Boundary,
    pub time: f64,
    pub dt: f64,
}

/// Defining methods associated with World.
impl World {
    pub fn new() -> Self {
        Self {
            gravity: (0.0, -9.81),
            objects: Vec::new(),
            restitution: 1.0,
            boundary: Boundary {
                x_range: (-1.0, 1.0),
                y_range: (-1.0, 1.0),
            },
            time: 0.0,
            dt: 0.1,
        }
    }

    pub fn get_boundary(&self) -> Boundary {
        self.boundary
    }

    /// This adds a new rigid body to world.
    pub fn add<T>(&mut self, object: T)
    where
        T: Updateable + AsRef<T> + 'static,
    {
        self.objects.push(Box::new(object))
    }

    /// The following functions are getters.
    pub fn get_gravity(self) -> (f64, f64) {
        self.gravity
    }

    pub fn get_objects(self) -> Vec<Box<dyn Updateable>> {
        self.objects
    }

    /// A constant concerning the velocities of objects when they rebound from a collision (coefficient of restitution).
    pub fn get_restitution(&self) -> f64 {
        self.restitution
    }

    pub fn get_timestep(&self) -> f64 {
        self.dt
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    /// The primary function that advances the world by time step dt.
    pub fn world_step<T>(self, dt: f64)
    where
        T: Updateable + AsRef<T> + Clone,
    {
        world_step::<T>(self, dt)
    }
}

/// Defining the world's edges.
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Boundary {
    pub x_range: (f64, f64),
    pub y_range: (f64, f64),
}

/// Defining useful methods and getters.
impl Boundary {
    pub fn new() -> Self {
        Self {
            x_range: (-1.0, 1.0),
            y_range: (-1.0, 1.0),
        }
    }

    pub fn get_x_range(&self) -> (f64, f64) {
        self.x_range
    }

    pub fn get_y_range(&self) -> (f64, f64) {
        self.y_range
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::rigidbodies::{Ball, RigidBody};
//     use super::*;

//     fn check_world_add() {
//         let mut w = World::new();
//         let object = Ball {
//             mass: 1.0,
//             radius: 1.0,
//             position: (0.0, 0.0),
//             velocity: (0.0, 0.0),
//             acceleration: (0.0, 0.0),
//             angular_velocity: 0.0,
//             parent: RigidBody {
//                 position: (0.0, 0.0),
//                 velocity: (0.0, 0.0),
//                 mass: 1.0,
//             },
//         };

//         w.add(object);

//         assert_eq!(w, World { gravity: (0.0, -9.81), objects: vec!(Box::new(object)), restitution: 1.0, boundary: Boundary::new(), time: 0.0, dt: 0.1 })
//     }
// }
