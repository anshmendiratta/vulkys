use ecolor::Color32;
use glm::Vec3;

use super::rigidbody::GenericObject;

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: f32,
    pub position: Vec3,
    pub velocity: Vec3,
    pub color: Color32,
}

impl GenericObject for Circle {
    fn get_debug(&self) -> String {
        format!(
            "r = {},
                p = {},
                v = {}",
            self.radius, self.position, self.velocity
        )
    }
    fn get_radius(&self) -> f32 {
        self.radius
    }
    fn get_position(&self) -> Vec3 {
        self.position
    }
    fn get_color(&self) -> Color32 {
        self.color
    }
}
