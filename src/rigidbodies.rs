// use std::{str::FromStr, fmt::Display};
// use strum_macros::{Display, EnumCount, EnumString};
// use crate::datastructures::linearqueue;
use crate::type_traits::*;

pub trait Updateable {
    fn get_rigidbody(&self) -> RigidBody;
}

pub enum RigidBodyCollection {
    RigidBody,
    Ball,
}
#[derive(Debug, PartialEq, Clone, Default)]
pub struct RigidBody {
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub mass: f64,
}

// impl<T: Updateable> HandleData<T> for RigidBody {}
impl MetaMethods for RigidBody {}

// #[derive(Debug, EnumCount, EnumString, PartialEq, Clone, Display)]
// pub enum RigidBodyMatch {
//     None,
//     Ball
// }

#[derive(Debug)]
pub struct Ball {
    pub mass: f64,
    pub radius: f64,
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub acceleration: (f64, f64),
    pub angular_velocity: f64,
    pub parent: RigidBody,
}

impl MetaMethods for Ball {}

impl Updateable for Ball {
    fn get_rigidbody(&self) -> RigidBody {
        self.parent
    }
}

// struct RigidBodyHistory<RigidBody> {
// velocity: LinearQueue<Vec<f64>>,
// }

pub trait HandleData<T: Updateable> {
    fn get_mass(&self) -> f64;

    fn get_position(&self) -> (f64, f64);
    fn set_position(&mut self, new_position: (f64, f64));

    fn get_velocity(&self) -> (f64, f64);
    fn set_velocity(&mut self, new_velocity: (f64, f64));

    fn get_angular_velocity(&self) -> f64;
    fn set_angular_velocity(&mut self, new_angular_velocity: f64);

    fn get_acceleration(&self) -> (f64, f64);
    fn set_acceleration(&mut self, acceleration: (f64, f64));
}

impl<T: Updateable> HandleData<T> for Ball {
    fn get_mass(&self) -> f64 {
        self.mass
    }

    fn get_position(&self) -> (f64, f64) {
        self.position
    }
    fn set_position(&mut self, new_position: (f64, f64)) {
        self.position = new_position
    }

    fn get_velocity(&self) -> (f64, f64) {
        self.velocity
    }
    fn set_velocity(&mut self, new_velocity: (f64, f64)) {
        self.velocity = new_velocity
    }

    fn get_angular_velocity(&self) -> f64 {
        self.angular_velocity
    }
    fn set_angular_velocity(&mut self, new_angular_velocity: f64) {
        self.angular_velocity = new_angular_velocity
    }

    fn get_acceleration(&self) -> (f64, f64) {
        self.acceleration
    }
    fn set_acceleration(&mut self, new_acceleration: (f64, f64)) {
        self.acceleration = new_acceleration
    }
}

impl Ball {
    pub fn make_from_function(
        &self,
        mass: f64,
        radius: f64,
        position: (f64, f64),
        velocity: (f64, f64),
    ) -> Ball {
        Ball {
            mass: mass,
            radius: radius,
            position: position,
            velocity: velocity,
            acceleration: (0.0, 0.0),
            angular_velocity: 0.0,
            parent: RigidBody {
                position: (0.0, 0.0),
                velocity: (0.0, 0.0),
                mass: 0.0,
            },
        }
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}
