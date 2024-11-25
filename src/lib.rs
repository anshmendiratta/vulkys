use std::ops::AddAssign;

use ecolor::Color32;
use renderer::vk_core::CustomVertex;
use serde::{Deserialize, Serialize};

pub mod core;
pub mod gui;
pub mod physics;
pub mod renderer;

const WINDOW_LENGTH: f32 = 1000.;

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq, bytemuck::AnyBitPattern)]
pub struct FVec2 {
    x: f32,
    y: f32,
}

impl FVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn to_custom_vertex(&self, color: Option<Color32>) -> CustomVertex {
        CustomVertex {
            position_in: *self,
            color: {
                if let Some(color) = color {
                    color.to_array()
                } else {
                    [0, 0, 0, 0]
                }
            },
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
