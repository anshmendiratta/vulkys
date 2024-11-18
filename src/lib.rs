use std::ops::AddAssign;

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
}

impl AddAssign for FVec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}
