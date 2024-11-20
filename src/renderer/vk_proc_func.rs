use crate::FVec2;

use super::vk_core::CustomVertex;

use libm::{cos, sin};
use std::f32::consts::PI;

pub type Triangle = [CustomVertex; 3];
pub type Polygon = Vec<Triangle>;
pub fn generate_polygon_triangles(n: u8, with_center: CustomVertex, with_radius: f32) -> Polygon {
    if vec![0, 1, 2].contains(&n) {
        return vec![[with_center.clone(), with_center.clone(), with_center]];
    }

    let angles: Vec<f32> = vec![0.; n as usize]
        .into_iter()
        .enumerate()
        .map(|(idx, _)| 2.0 * PI / (n as f32) * idx as f32)
        .collect();

    let mut outer_coordinates: Vec<CustomVertex> = angles
        .iter()
        .map(|angle| CustomVertex {
            position_in: FVec2::new(
                with_radius * (cos(angle.clone() as f64) as f32) + with_center.position_in.x,
                with_radius * (sin(angle.clone() as f64) as f32) + with_center.position_in.y,
            ),
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

pub trait PolygonMethods {
    fn destructure_into_list(&self) -> Vec<CustomVertex>;
}
impl PolygonMethods for Polygon {
    fn destructure_into_list(&self) -> Vec<CustomVertex> {
        self.clone().into_flattened()
    }
}

#[cfg(test)]
mod tests {
    use super::PolygonMethods;
    use crate::{
        renderer::{vk_core::CustomVertex, vk_proc_func::Polygon},
        FVec2,
    };

    #[test]
    pub fn check_destructuring() {
        let sample_in: Vec<[CustomVertex; 3]> = vec![[
            CustomVertex {
                position_in: FVec2::new(0., 0.),
            },
            CustomVertex {
                position_in: FVec2::new(0., 0.),
            },
            CustomVertex {
                position_in: FVec2::new(0., 0.),
            },
        ]];

        let function_out = Polygon::destructure_into_list(&sample_in);
        let desired_out: Vec<CustomVertex> = vec![
            CustomVertex {
                position_in: FVec2::new(0., 0.),
            },
            CustomVertex {
                position_in: FVec2::new(0., 0.),
            },
            CustomVertex {
                position_in: FVec2::new(0., 0.),
            },
        ];

        assert_eq!(desired_out, function_out);
    }
}
