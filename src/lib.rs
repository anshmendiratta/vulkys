use std::ops::AddAssign;

use renderer::vk_core::CustomVertex;
use serde::Serialize;

pub mod core;
pub mod gui;
pub mod physics;
pub mod renderer;

#[derive(Clone, Copy, Serialize, Debug, PartialEq)]
pub struct FVec2 {
    x: f32,
    y: f32,
}

impl FVec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
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
