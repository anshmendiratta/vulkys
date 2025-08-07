use ecolor::Color32;
use glm::Vec3;
use nalgebra::Rotation3;

use crate::render::cuboid::{CUBE_INDICES, CUBE_VERTICES};

#[derive(Debug, Clone, PartialEq)]
pub struct RawCuboid {
    pub half_extent: Vec3, // Doubles as the "scalars" for the cuboid.
    pub init_position: Vec3,
    pub init_velocity: Vec3,
    pub init_rotation: Rotation3<f32>,
    pub color: Color32,
}

impl RawCuboid {
    pub fn get_init_position(&self) -> Vec3 {
        self.init_position
    }

    pub fn get_init_velocity(&self) -> Vec3 {
        self.init_velocity
    }

    pub fn get_half_extent(&self) -> Vec3 {
        self.half_extent
    }

    pub fn get_orientation(&self) -> Rotation3<f32> {
        self.init_rotation
    }

    pub fn get_vertices(&self, with_rotation: Rotation3<f32>, with_translation: Vec3) -> Vec<Vec3> {
        let mut vertices: Vec<Vec3> = CUBE_VERTICES.clone().to_vec();
        let scalars = self.half_extent.map(|e| e * 2.);

        // Scales.
        for vertex in vertices.iter_mut() {
            vertex.x *= scalars.x;
            vertex.y *= scalars.y;
            vertex.z *= scalars.z;
        }

        // Rotation.
        for vertex in vertices.iter_mut() {
            *vertex = with_rotation * *vertex;
        }

        // Translation.
        for vertex in vertices.iter_mut() {
            *vertex += with_translation;
        }

        vertices
    }

    pub fn get_vertex_indices(&self) -> Vec<u16> {
        CUBE_INDICES.to_vec()
    }
}
