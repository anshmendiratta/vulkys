use ecolor::Color32;
use glm::Vec3;

use super::rigidbody::GenericObject;

#[derive(Debug, Clone, PartialEq)]
pub struct RawBall {
    pub radius: f32,
    pub init_position: Vec3,
    pub init_velocity: Vec3,
    pub color: Color32,
}

impl GenericObject for RawBall {
    fn get_debug(&self) -> String {
        format!(
            "r = {},
                p = {},
                v = {}",
            self.radius, self.init_position, self.init_velocity
        )
    }
    fn get_radius(&self) -> f32 {
        self.radius
    }
    fn get_color(&self) -> Color32 {
        self.color
    }
    fn get_init_position(&self) -> Vec3 {
        self.init_position
    }
    fn get_init_velocity(&self) -> Vec3 {
        self.init_velocity
    }
}
