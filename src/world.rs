use std::default;

use crate::rigidbodies::*;
// use crate::resolve;
use crate::step;

#[derive(Debug, Default)]
pub struct World {
    gravity: Vec<f64>, 
    objects: Vec<RigidBody>,
    restitution: f64,
    boundary: Vec<Vec<f64>>,
    time: f64,
}

pub struct Plane {
    y: f64,
    angle: f64,
}

impl World {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn add(&self, object: RigidBody) {
        self.objects.push(object)
    }

    pub fn get_gravity(self) -> &'static Vec<f64> {
        &self.gravity
    }

    pub fn get_objects(&self) -> &Vec<RigidBody> {
        &self.objects
    }

    pub fn get_restitution(&self) -> &f64 {
        &self.restitution
    }

    pub fn world_step(self, dt: f64) {
        step::step(self, dt)
    }
}
