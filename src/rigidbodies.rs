// use std::{str::FromStr, fmt::Display};
use strum_macros::{Display, EnumCount, EnumString};
// use crate::datastructures::linearqueue;

#[derive(Debug, EnumCount, EnumString, PartialEq, Clone, Display)]
pub enum RigidBody {
    None,
    Ball {
        mass: f64,
        radius: f64,
        position: Vec<f64>,
        velocity: Vec<f64>,
        angular_velocity: f64,
    },
}

#[derive(Debug, EnumCount, EnumString, PartialEq, Clone, Display)]
pub enum RigidBodyMatch {
    None,
    Ball
}

impl RigidBody {
    fn get_velocity(&self) -> &Vec<f64> {
        &vec![0.0, 0.0]
    }

    fn get_position(&self) -> &Vec<f64> {
        &vec![0.0, 0.0]
    }

    fn get_angular_velocity(&self) -> &f64 {
        &0.0
    }
}

impl GetData for RigidBody {}

// struct RigidBodyHistory<RigidBody> {
// velocity: LinearQueue<Vec<f64>>,
// }

#[derive(Debug)]
pub struct Ball {
    pub mass: f64,
    pub radius: f64,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub acceleration: Vec<f64>,
    pub angular_velocity: f64,
}

// impl std::fmt::Display for Ball {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Ball: ", )
//     }
// }

pub trait GetData {
    fn get_radius(&self) -> &f64 {
        &0.0
    }

    fn get_position(&self) -> &Vec<f64> {
        &vec![0.0, 0.0]
    }
    
    fn get_velocity(&self) -> &Vec<f64> {
        &vec![0.0, 0.0]
    }

    fn get_mass(&self) -> &f64 {
        &0.0
    }

    fn get_angular_velocity(&self) -> &f64 {
        &0.0
    }

    fn get_acceleration(&self) -> Vec<f64> {
        &vec![0.0, 0.0]
    }
}

impl GetData for Ball {
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

    fn get_acceleration(&self) -> Vec<f64> {
        &self.acceleration
    }
}

impl Ball {
    pub fn make_from_function(
        &self,
        mass: f64,
        radius: f64,
        position: Vec<f64>,
        velocity: Vec<f64>,
        acceleration: Vec<f64>,
        angular_velocity: f64,
    ) -> Ball {
        Ball {
            mass: mass,
            radius: radius,
            position: position,
            velocity: velocity,
            acceleration: self.acceleration,
            angular_velocity: angular_velocity,
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
