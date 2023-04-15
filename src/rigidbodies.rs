use std::str::FromStr;
use strum_macros::{EnumCount, EnumString};
// use crate::datastructures::LinearQueue;

#[derive(Debug, EnumCount, EnumString, PartialEq, Clone, Copy)]
pub enum RigidBody {
    Ball,
}

// impl RigidBody {
//     fn get_variants(&self) -> usize {
//         let counter = 0;

//     }
// }

// struct RigidBodyHistory<RigidBody> {
// velocity: LinearQueue<Vec<f64>>,
// }

#[derive(Debug)]
pub struct Ball {
    pub mass: f64,
    pub radius: f64,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub angular_velocity: f64,
}

trait GetData {
    fn get_radius(&self) -> &f64 {
        &0.0
    }
    fn get_mass(&self) -> &f64 {
        &0.0
    }
}

impl GetData for Ball {
    fn get_radius(&self) -> &f64 {
        &self.radius
    }

    fn get_mass(&self) -> &f64 {
        &self.mass
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            mass: 1.0,
            radius: 1.0,
            position: vec![0.0, 0.0],
            velocity: vec![0.0, 0.0],
            angular_velocity: 0.0,
        }
    }
}
