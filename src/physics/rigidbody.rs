use crate::vulkan::core::CustomVertex;

use super::ball::RawBall;
use super::cube::RawCuboid;
use super::lib::COEFF_RESTITUTION;

use ecolor::Color32;
use glm::Vec3;
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

pub trait GenericObject {
    fn get_debug(&self) -> String;
    fn get_radius(&self) -> f32;
    fn get_color(&self) -> Color32;
    fn get_vertices(&self) -> Vec<CustomVertex>;
    fn get_indices(&self) -> Vec<u16>;
    fn get_init_position(&self) -> Vec3;
    fn get_init_velocity(&self) -> Vec3;
}

type RBid = u8;
#[derive(Clone, Debug, PartialEq)]
pub enum RigidBody {
    Ball(RawBall, RBid),
    Cuboid(RawCuboid, RBid),
}

#[allow(dead_code)]
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
                    rotation,
                    ..
                },
                _,
            ) => ColliderBuilder::cuboid(half_extent.x, half_extent.y, half_extent.z)
                .restitution(COEFF_RESTITUTION)
                .rotation(rotation.scaled_axis()),
            // _ => unreachable!(),
        }
    }

    pub fn get_vertices(&self) -> Vec<CustomVertex> {
        let vertices = match self {
            // RigidBody::Ball_(b, _) => b.get_vertices(),
            RigidBody::Cuboid(c, _) => c.get_vertices(),
            _ => vec![],
        };

        let custom_vertices: Vec<CustomVertex> = vertices
            .iter()
            .map(|v| CustomVertex {
                position: *v,
                color: self.get_color().to_array(),
            })
            .collect();

        custom_vertices
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

    // pub fn get_object(&self) -> Box<dyn GenericObject> {
    //     match self {
    //         RigidBody::Ball_(
    //             RawBall {
    //                 radius,
    //                 init_position: position,
    //                 init_velocity: velocity,
    //                 color,
    //             },
    //             _,
    //         ) => Box::new(RawBall {
    //             radius: *radius,
    //             init_position: *position,
    //             init_velocity: *velocity,
    //             color: *color,
    //         }),
    //         RigidBody::Cuboid_(
    //             RawCuboid {
    //                 half_extent,
    //                 init_position: position,
    //                 init_velocity: velocity,
    //                 rotation: orientation,
    //                 color,
    //             },
    //             _,
    //         ) => Box::new(RawCuboid {
    //             half_extent: *half_extent,
    //             init_position: *position,
    //             init_velocity: *velocity,
    //             rotation: *orientation,
    //             color: *color,
    //         }),
    //     }
    // }

    pub fn get_vertex_count(&self) -> u8 {
        match self {
            RigidBody::Ball(_, _) => 32,
            RigidBody::Cuboid(_, _) => 8,
        }
    }

    // pub fn to_polygon(&self, rotation: f32) -> Polygon {
    //     let inner_object = self.get_object();
    //     let long_radius = inner_object.get_radius();
    //     let position = inner_object.get_init_position();
    //     // TODO: Add z component. Temporary zero.
    //     let center_coordinate = Vec3::new(position.x, position.y, 0.);
    //     generate_polygon_triangles(
    //         self.get_vertex_count(),
    //         CustomVertex {
    //             position: center_coordinate,
    //             color: self.get_color().to_array(),
    //         },
    //         long_radius,
    //         rotation,
    //         self.get_color(),
    //     )
    // }

    pub fn get_color(&self) -> Color32 {
        match self {
            RigidBody::Ball(c, _) => c.color,
            RigidBody::Cuboid(c, _) => c.color,
        }
    }

    pub fn get_radius(&self) -> f32 {
        match self {
            RigidBody::Ball(c, _) => c.radius,
            RigidBody::Cuboid(c, _) => c.get_radius(),
        }
    }

    pub fn get_init_position(&self) -> Vec3 {
        match self {
            RigidBody::Ball(c, _) => c.get_init_position(),
            RigidBody::Cuboid(c, _) => c.get_init_position(),
        }
    }

    pub fn get_init_velocity(&self) -> Vec3 {
        match self {
            RigidBody::Ball(c, _) => c.get_init_velocity(),
            RigidBody::Cuboid(c, _) => c.get_init_velocity(),
        }
    }

    // fn get_debug(&self) -> String {
    //     let inner_object = self.get_object();
    //     inner_object.get_debug()
    // }

    pub fn type_to_string(&self) -> &str {
        match self {
            RigidBody::Ball(_, _) => "Ball",
            RigidBody::Cuboid(_, _) => "Cube",
        }
    }
}
