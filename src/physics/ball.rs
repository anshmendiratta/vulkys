use ecolor::Color32;
use glm::Vec3;
use nalgebra::Rotation3;

use crate::models::ball::{BALL_INDICES, DEFAULT_BALL_NORMALS, DEFAULT_BALL_VERTICES};

#[derive(Debug, Clone, PartialEq)]
pub struct RawBall {
    pub radius: f32,
    pub init_position: Vec3,
    pub init_velocity: Vec3,
    pub init_rotation: Rotation3<f32>,
    pub color: Color32,
}

impl RawBall {
    pub fn get_init_position(&self) -> Vec3 {
        self.init_position
    }

    pub fn get_init_velocity(&self) -> Vec3 {
        self.init_velocity
    }

    pub fn get_init_rotation(&self) -> Rotation3<f32> {
        self.init_rotation
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_orientation(&self) -> Rotation3<f32> {
        self.init_rotation
    }

    pub fn get_vertices(&self, with_rotation: Rotation3<f32>, with_translation: Vec3) -> Vec<Vec3> {
        let mut vertices: Vec<Vec3> = DEFAULT_BALL_VERTICES.clone().to_vec();

        // Scales.
        for vertex in vertices.iter_mut() {
            *vertex *= self.radius;
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

    pub fn get_normals(&self, with_rotation: Rotation3<f32>, with_translation: Vec3) -> Vec<Vec3> {
        let mut normals: Vec<Vec3> = DEFAULT_BALL_NORMALS.clone().to_vec();

        // Rotation.
        for normal in normals.iter_mut() {
            *normal = with_rotation * *normal;
        }

        // Translation.
        for normal in normals.iter_mut() {
            *normal += with_translation;
        }

        normals
    }

    pub fn get_vertex_indices(&self) -> Vec<u16> {
        BALL_INDICES.to_vec()
    }
}
