use super::vk_core::CustomVertex;

use libm::{cos, sin};
use std::f32::consts::PI;

pub type Triangle = [CustomVertex; 3];
pub type Polygon = Vec<Triangle>;
pub fn generate_polygon_triangles(n: u8, with_center: CustomVertex) -> Polygon {
    let radius: f32 = 0.5;
    let angles: Vec<f32> = vec![0.; n as usize]
        .into_iter()
        .enumerate()
        .map(|(idx, _)| 2.0 * PI / (n as f32) * idx as f32)
        .collect();

    let mut outer_coordinates: Vec<CustomVertex> = angles
        .iter()
        .map(|angle| CustomVertex {
            position_in: [
                radius * (cos(angle.clone() as f64) as f32) + with_center.position_in[0],
                radius * (sin(angle.clone() as f64) as f32) + with_center.position_in[1],
            ],
        })
        .collect();
    outer_coordinates.push(outer_coordinates[0].clone());

    let mut triangles: Vec<Triangle> = Vec::with_capacity(n as usize);
    outer_coordinates.windows(2).for_each(|win| {
        let (v1, v2) = match win {
            [v1, v2] => (v1, v2),
            _ => panic!("somehow a window of size not 2"),
        };
        triangles.push([with_center.clone(), v1.clone(), v2.clone()]);
    });

    triangles
}
