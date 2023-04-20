use std::default;
use crate::rigidbodies::*;
use crate::rigidbodies::{Updateable, HandleData};
use crate::step::{self, step};

#[derive(Debug, Default)]
pub struct World<T: Updateable + HandleData<T>> {
    gravity: (f64, f64),
    objects: Vec<T>,
    restitution: f64,
    boundary: ((f64, f64), (f64, f64)),
    time: f64,
    dt: f64,
}

pub struct Plane {
    y: f64,
    angle: f64,
}

impl<T: Updateable + HandleData<T>> World<T> {
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

    pub fn add(&self, object: T) {
        self.objects.push(object)
    }

    pub fn get_gravity(self) -> (f64, f64) {
        self.gravity
    }

    pub fn get_objects(&self) -> Vec<T> {
        self.objects
    }

    pub fn get_restitution(&self) -> f64 {
        self.restitution
    }
    
    pub fn get_timestep(&self) -> f64 {
        self.dt
    }

    pub fn world_step(&self, dt: f64) {
        step::step::<T>(*self, dt)
    }

}
