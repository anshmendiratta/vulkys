#![allow(unused_variables)]
#![allow(dead_code)]

use std::ops::{AddAssign, Sub};

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
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).powf(0.5)
    }
    pub fn to_custom_vertex(&self, color: Option<Color32>) -> CustomVertex {
        CustomVertex {
            position_in: *self,
            color: if let Some(color) = color {
                color.to_array()
            } else {
                [0, 0, 0, 0]
            },
        }
    }
    pub fn get_unit(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }
    pub fn get_orthogonal_unit(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.y / magnitude,
            y: -self.x / magnitude,
        }
    }
    pub fn dot(&self, other: FVec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    pub fn project_onto(&self, other: FVec2) -> Self {
        let other_as_unit = other.get_unit();
        let scalar_projection = self.dot(other_as_unit);
        Self {
            x: scalar_projection * other_as_unit.x,
            y: scalar_projection * other_as_unit.y,
        }
    }
    pub fn mirror_along(&self, other: FVec2) -> Self {
        let projection_onto_other = self.project_onto(other);
        let rejection_from_other = *self - projection_onto_other;
        Self {
            x: self.x - 2. * rejection_from_other.x,
            y: self.y - 2. * rejection_from_other.y,
        }
    }
}

impl Sub for FVec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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

#[cfg(test)]
mod tests {
    use crate::FVec2;

    #[test]
    fn check_projection() {
        let to_project = FVec2::new(1. / 2_f32.powf(0.5), 1. / 2_f32.powf(0.5));
        let to_project_onto = FVec2::new(1., 0.);
        let desired_output = FVec2::new(1. / 2_f32.powf(0.5), 0.);

        assert_eq!(desired_output, to_project.project_onto(to_project_onto));
    }
    #[test]
    fn check_mirroring() {
        let to_mirror = FVec2::new(1. / 2_f32.powf(0.5), 1. / 2_f32.powf(0.5));
        let to_mirror_along = FVec2::new(1., 0.);
        let desired_output = FVec2::new(1. / 2_f32.powf(0.5), -1. / 2_f32.powf(0.5));

        assert_eq!(desired_output, to_mirror.mirror_along(to_mirror_along));
    }
}
