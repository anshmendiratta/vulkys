use std::ops::AddAssign;

use renderer::vk_core::CustomVertex;
use serde::Serialize;

pub mod core;
pub mod gui;
pub mod physics;
pub mod renderer;

const WINDOW_LENGTH: f32 = 1000.;

#[derive(Clone, Copy, Serialize, Debug, PartialEq)]
pub struct FVec2 {
    x: f32,
    y: f32,
}

impl FVec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn to_custom_vertex(&self) -> CustomVertex {
        CustomVertex {
            position_in: [self.x, self.y],
        }
    }
}

impl AddAssign for FVec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl std::fmt::Display for FVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_string = &format!("({}, {})", self.x, self.y)[..];
        f.write_str(display_string)
    }
}
