use serde::Serialize;

use crate::FVec2;

use super::rigidbody::GenericObject;

#[derive(Debug, Clone, Serialize)]
pub struct Circle {
    pub radius: f32,
    pub position: FVec2,
    pub velocity: FVec2,
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
}
