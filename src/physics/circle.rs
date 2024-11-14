use egui::Vec2;

use super::{
    lib::DELTA_TIME,
    rigidbody::{GenericObject, Updateable},
};

#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f32,
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Updateable for Circle {
    fn update_position(&mut self, velocity: Vec2) {
        self.position += Vec2::new(velocity.x * DELTA_TIME, velocity.y * DELTA_TIME);
    }

    fn update_velocity(&mut self, acceleration: Vec2) {
        self.velocity += Vec2::new(acceleration.x * DELTA_TIME, acceleration.y * DELTA_TIME);
    }
}

impl GenericObject for Circle {
    fn get_debug(&self) -> &str {
        "r={self.radius},p={self.position},v={self.velocity}"
    }
}
