use libm::{cos, sin};
use std::f32::consts::PI;
use winit::dpi::PhysicalSize;

use super::vk_core::CustomVertex;

pub fn generate_hexagon_vertices<const N: usize>(
    window_dimensions: PhysicalSize<f32>,
) -> Vec<CustomVertex> {
    let window_height = window_dimensions.height;
    let radius: f32 = 0.5;
    let angles: Vec<f32> = [0.0; N]
        .into_iter()
        .enumerate()
        .map(|(idx, _)| 2.0 * PI / (N as f32) * idx as f32)
        .collect();

    let coordinates = angles
        .iter()
        .map(|angle| CustomVertex {
            position_in: [
                window_height * radius * (cos(angle.clone() as f64) as f32),
                window_height * radius * (sin(angle.clone() as f64) as f32),
            ],
        })
        .collect();

    coordinates
}
