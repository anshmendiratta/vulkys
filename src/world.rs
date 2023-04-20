// use crate::rigidbodies::{HandleData, Updateable};

// use NEA::rigidbodies::Updateable;

use crate::rigidbodies::{HandleData, Updateable};
use crate::step::world_step;

// trait RigidBodyTraits: Updateable + HandleData<RigidBodyTraits> {};

#[derive(Default)]
pub struct World {
    gravity: (f64, f64),
    objects: Vec<Box<dyn Updateable>>,
    restitution: f64,
    boundary: ((f64, f64), (f64, f64)),
    time: f64,
    dt: f64,
}
 
pub struct Plane {
    y: f64,
    angle: f64,
}

impl World {
    pub fn new() -> Self {
        Self {
            gravity: (0.0, -9.81),
            objects: Vec::new(),
            restitution: 1.0,
            boundary: ((-1.0, 1.0), (-1.0, 1.0)),
            time: 0.0,
            dt: 0.1,
        }
    }

    pub fn add<T>(&mut self, object: T) where T: Updateable + HandleData<T> + AsRef<T> {
        self.objects.push(Box::new(object))
    }

    pub fn get_gravity(self) -> (f64, f64) {
        self.gravity
    }

    pub fn get_objects(self) -> Vec<Box<dyn Updateable>> {
        self.objects
    }

    pub fn get_restitution(&self) -> f64 {
        self.restitution
    }

    pub fn get_timestep(&self) -> f64 {
        self.dt
    }

    pub fn world_step<T>(self, dt: f64) where T: Updateable + HandleData<T> + AsRef<T> {
        world_step::<T>(self, dt)
    }
}
