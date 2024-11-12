use libm::{cos, sin};
use std::f32::consts::PI;

use super::vk_core::MyVertex;

pub fn generate_hexagon_vertices<const N: usize>() -> Vec<MyVertex> {
    let radius: f32 = 0.5;
    let angles: Vec<f32> = [0.0; N]
        .into_iter()
        .enumerate()
        .map(|(idx, _)| 2.0 * PI / (N as f32) * idx as f32)
        .collect();

    let coordinates = angles
        .iter()
        .map(|angle| MyVertex {
            position: [
                radius * (cos(angle.clone() as f64) as f32),
                radius * (sin(angle.clone() as f64) as f32),
            ],
        })
        .collect();

    coordinates
}
