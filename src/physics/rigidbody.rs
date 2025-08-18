use crate::vulkan::primitives::{DrawNormal, DrawVertex};

use super::cuboid::RawCuboid;
use super::lib::COEFF_RESTITUTION;
use super::{ball::RawBall, cylinder::RawCylinder};

use ecolor::Color32;
use glm::{Vec3, Vec4};
use nalgebra::Rotation3;
use rapier3d::prelude::ColliderBuilder;

type RBid = u8;
#[derive(Clone, Debug, PartialEq)]
pub enum RigidBody {
    Ball(RawBall, RBid),
    Cuboid(RawCuboid, RBid),
    Cylinder(RawCylinder, RBid),
}

impl RigidBody {
    pub fn convert_to_collider(&self) -> ColliderBuilder {
        match self {
            RigidBody::Ball(
                RawBall {
                    radius,
                    init_rotation,
                    ..
                },
                _,
            ) => ColliderBuilder::ball(*radius)
                .restitution(COEFF_RESTITUTION)
                .rotation(init_rotation.scaled_axis()),
            RigidBody::Cuboid(
                RawCuboid {
                    half_extent,
                    init_rotation,
                    ..
                },
                _,
            ) => ColliderBuilder::cuboid(half_extent.x, half_extent.y, half_extent.z)
                .restitution(COEFF_RESTITUTION)
                .rotation(init_rotation.scaled_axis()),
            RigidBody::Cylinder(
                RawCylinder {
                    half_height,
                    radius,
                    init_rotation,
                    ..
                },
                _,
            ) => ColliderBuilder::cylinder(*half_height, *radius)
                .rotation(init_rotation.scaled_axis()),
        }
    }

    pub fn get_vertices(
        &self,
        with_rotation: Rotation3<f32>,
        with_translation: Vec3,
    ) -> Vec<DrawVertex> {
        let vertices = match self {
            RigidBody::Ball(b, _) => b.get_vertices(with_rotation, with_translation),
            RigidBody::Cuboid(cb, _) => cb.get_vertices(with_rotation, with_translation),
            RigidBody::Cylinder(cl, _) => cl.get_vertices(with_rotation, with_translation),
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
            RigidBody::Ball(b, _) => b.get_normals(with_rotation, with_translation),
            RigidBody::Cuboid(cb, _) => cb.get_normals(with_rotation, with_translation),
            RigidBody::Cylinder(cl, _) => cl.get_normals(with_rotation, with_translation),
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
            RigidBody::Ball(b, _) => b.get_vertex_indices(),
            RigidBody::Cuboid(cb, _) => cb.get_vertex_indices(),
            RigidBody::Cylinder(cl, _) => cl.get_vertex_indices(),
        };

        indices
    }

    pub fn get_id(&self) -> RBid {
        match self {
            RigidBody::Ball(RawBall { .. }, id) => id.clone(),
            RigidBody::Cuboid(RawCuboid { .. }, id) => id.clone(),
            RigidBody::Cylinder(RawCylinder { .. }, id) => id.clone(),
        }
    }

    pub fn get_color(&self) -> Color32 {
        match self {
            RigidBody::Ball(c, _) => c.color,
            RigidBody::Cuboid(cb, _) => cb.color,
            RigidBody::Cylinder(cl, _) => cl.color,
        }
    }

    pub fn get_init_position(&self) -> Vec3 {
        match self {
            RigidBody::Ball(c, _) => c.get_init_position(),
            RigidBody::Cuboid(cb, _) => cb.get_init_position(),
            RigidBody::Cylinder(cl, _) => cl.get_init_position(),
        }
    }

    pub fn get_init_velocity(&self) -> Vec3 {
        match self {
            RigidBody::Ball(b, _) => b.get_init_velocity(),
            RigidBody::Cuboid(cb, _) => cb.get_init_velocity(),
            RigidBody::Cylinder(cl, _) => cl.get_init_velocity(),
        }
    }

    pub fn get_init_rotation(&self) -> Rotation3<f32> {
        match self {
            RigidBody::Ball(b, _) => b.get_init_rotation(),
            RigidBody::Cuboid(cb, _) => cb.get_init_rotation(),
            RigidBody::Cylinder(cl, _) => cl.get_init_rotation(),
        }
    }

    pub fn type_to_string(&self) -> &str {
        match self {
            RigidBody::Ball(_, _) => "Ball",
            RigidBody::Cuboid(_, _) => "Cube",
            RigidBody::Cylinder(_, _) => "Cylinder",
        }
    }
}
