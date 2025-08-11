use crate::vulkan::primitives::{DrawNormal, DrawVertex};

use super::ball::RawBall;
use super::cuboid::RawCuboid;
use super::lib::COEFF_RESTITUTION;

use ecolor::Color32;
use glm::{Vec3, Vec4};
use nalgebra::Rotation3;
use rapier3d::prelude::ColliderBuilder;

#[derive(PartialEq, Clone, Copy)]
pub enum RigidBodySelection {
    None,
    Ball_,
}

impl Default for RigidBodySelection {
    fn default() -> Self {
        Self::None
    }
}

impl RigidBodySelection {
    pub fn to_string(&self) -> &str {
        match self {
            RigidBodySelection::None => "None",
            RigidBodySelection::Ball_ => "Ball",
        }
    }
}

type RBid = u8;
#[derive(Clone, Debug, PartialEq)]
pub enum RigidBody {
    Ball(RawBall, RBid),
    Cuboid(RawCuboid, RBid),
}

impl RigidBody {
    pub fn convert_to_collider(&self) -> ColliderBuilder {
        match self {
            RigidBody::Ball(RawBall { radius, .. }, _) => {
                ColliderBuilder::ball(*radius).restitution(COEFF_RESTITUTION)
            }
            // ball .translation(vector![position.x, position.y]),
            RigidBody::Cuboid(
                RawCuboid {
                    half_extent,
                    init_rotation: rotation,
                    ..
                },
                _,
            ) => ColliderBuilder::cuboid(half_extent.x, half_extent.y, half_extent.z)
                .restitution(COEFF_RESTITUTION)
                .rotation(rotation.scaled_axis()),
            // _ => unreachable!(),
        }
    }

    pub fn get_vertices(
        &self,
        with_rotation: Rotation3<f32>,
        with_translation: Vec3,
    ) -> Vec<DrawVertex> {
        let vertices = match self {
            // RigidBody::Ball_(b, _) => b.get_vertices(),
            RigidBody::Cuboid(c, _) => c.get_vertices(with_rotation, with_translation),
            _ => vec![],
        };

        vertices
            .iter()
            .map(|v| DrawVertex {
                position: *v,
                color: {
                    let [r, g, b, a] = self.get_color().to_array().map(|x| x as f32);
                    Vec4::new(r / 255., g / 255., b / 255., a / 255.)
                },
            })
            .collect()
    }

    pub fn get_normals(
        &self,
        with_rotation: Rotation3<f32>,
        with_translation: Vec3,
    ) -> Vec<DrawNormal> {
        let normals = match self {
            // RigidBody::Ball_(b, _) => b.get_vertices(),
            RigidBody::Cuboid(c, _) => c.get_normals(with_rotation, with_translation),
            _ => vec![],
        };

        normals
            .iter()
            .map(|n| DrawNormal {
                normal: (*n).into(),
            })
            .collect()
    }

    pub fn get_vertex_indices(&self) -> Vec<u16> {
        let indices = match self {
            RigidBody::Cuboid(c, _) => c.get_vertex_indices(),
            _ => vec![],
        };

        indices
    }

    pub fn get_id(&self) -> RBid {
        match self {
            RigidBody::Ball(RawBall { .. }, id) => id.clone(),
            RigidBody::Cuboid(RawCuboid { .. }, id) => id.clone(),
        }
    }

    pub fn get_vertex_count(&self) -> u8 {
        match self {
            RigidBody::Ball(_, _) => 32,
            RigidBody::Cuboid(_, _) => 8,
        }
    }

    pub fn get_color(&self) -> Color32 {
        match self {
            RigidBody::Ball(c, _) => c.color,
            RigidBody::Cuboid(c, _) => c.color,
        }
    }

    pub fn get_init_position(&self) -> Vec3 {
        match self {
            // RigidBody::Ball(c, _) => c.get_init_position(),
            RigidBody::Cuboid(c, _) => c.get_init_position(),
            // FIX.
            _ => Vec3::zeros(),
        }
    }

    pub fn get_init_velocity(&self) -> Vec3 {
        match self {
            RigidBody::Cuboid(c, _) => c.get_init_velocity(),
            // FIX.
            _ => Vec3::zeros(),
        }
    }

    pub fn type_to_string(&self) -> &str {
        match self {
            RigidBody::Ball(_, _) => "Ball",
            RigidBody::Cuboid(_, _) => "Cube",
        }
    }
}
