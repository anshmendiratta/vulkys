#[derive(Debug)]
pub enum RigidBody {
    Ball {
        mass: f32,
        radius: f32,
        position: Vec<f32>,
        velocity: Vec<f32>,
    },
}

#[derive(Debug)]
pub struct Ball {
    pub mass: f32, 
    pub radius: f32,
    pub position: Vec<f32>,
    pub velocity: Vec<f32>,
}

trait GetData {
    fn get_radius(&self) -> &f32 { &0.0 }
    fn get_mass(&self) -> &f32 { &0.0 }
}

impl GetData for Ball {
    fn get_radius(&self) -> &f32 {
        &self.radius
    }

    fn get_mass(&self) -> &f32 {
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
        }
    }
}
