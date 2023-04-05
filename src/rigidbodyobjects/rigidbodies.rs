use std::os::unix::ucred::impl_mac;

#[derive(Debug)]
pub enum RigidBody {
    Ball {
        radius: f32,
        position: Vec<f32>,
        velocity: Vec<f32>,
    },
}

#[derive(Debug)]
pub struct Ball {
    pub radius: f32,
    pub position: Vec<f32>,
    pub velocity: Vec<f32>,
}

impl Ball {
    pub fn get_position(&self) -> &Vec<f32> {
        &self.position
    }

    pub fn get_velocity(&self) -> &Vec<f32> {
        &self.velocity
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            radius: 1.0,
            position: vec![0.0, 0.0],
            velocity: vec![0.0, 0.0],
        }
    }
}
