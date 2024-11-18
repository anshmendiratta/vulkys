use serde::Serialize;

use crate::FVec2;

use super::{
    lib::DELTA_TIME,
    rigidbody::{GenericObject, Updateable},
};

#[derive(Debug, Clone, Serialize)]
pub struct Circle {
    pub radius: f32,
    pub position: FVec2,
    pub velocity: FVec2,
}

impl Updateable for Circle {
    fn update_position(&mut self, velocity: FVec2) {
        self.position += FVec2::new(velocity.x * DELTA_TIME, velocity.y * DELTA_TIME);
    }
    fn update_velocity(&mut self, acceleration: FVec2) {
        self.velocity += FVec2::new(acceleration.x * DELTA_TIME, acceleration.y * DELTA_TIME);
    }
}

impl GenericObject for Circle {
    fn get_debug(&self) -> &str {
        "r={self.radius},p={self.position},v={self.velocity}"
    }
}
