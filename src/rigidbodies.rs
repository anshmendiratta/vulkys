// use std::{str::FromStr, fmt::Display};
use strum_macros::{Display, EnumCount, EnumString};
// use crate::datastructures::linearqueue;
use crate::type_traits::*;

pub trait Updateable {
    fn get_rigidbody(&self) -> RigidBody;
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct RigidBody {
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub mass: f64,
}

impl<T> HandleData<T> for RigidBody {}
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
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub acceleration: Vec<f64>,
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
    fn get_radius(&self) -> &f64 {
        &self.radius
    }

    fn get_mass(&self) -> &f64 {
        &self.mass
    }

    fn get_position(&self) -> &Vec<f64> {
        &self.position
    }

    fn get_velocity(&self) -> &Vec<f64> {
        &self.velocity
    }

    fn get_angular_velocity(&self) -> &f64 {
        &self.angular_velocity
    }

    fn get_acceleration(&self) -> &Vec<f64> {
        &self.acceleration
    }
}

impl<T> HandleData<T> for Ball {}

impl Ball {
    pub fn make_from_function(
        &self,
        mass: f64,
        radius: f64,
        position: Vec<f64>,
        velocity: Vec<f64>,
    ) -> Ball {
        Ball {
            mass: mass,
            radius: radius,
            position: position,
            velocity: velocity,
            acceleration: (),
            angular_velocity: (),
            parent: RigidBody {
                position: (),
                velocity: (),
                mass: (),
            },
        }
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}
