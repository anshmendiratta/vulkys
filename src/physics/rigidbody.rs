use super::circle::Circle;
use super::lib::COEFF_RESTITUTION;
use super::square::Square;

use crate::vulkan::{
    core::CustomVertex,
    procedural::{Polygon, generate_polygon_triangles},
};
use ecolor::Color32;
use glm::Vec3;
use rapier2d::prelude::ColliderBuilder;

#[derive(PartialEq, Clone, Copy)]
pub enum RigidBodySelection {
    None,
    Circle_,
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
            RigidBodySelection::Circle_ => "Circle",
        }
    }
}

pub trait GenericObject {
    fn get_debug(&self) -> String;
    fn get_radius(&self) -> f32;
    fn get_position(&self) -> Vec3;
    fn get_color(&self) -> Color32;
}

type RBid = u8;
#[derive(Clone, Debug, PartialEq)]
pub enum RigidBody {
    Circle_(Circle, RBid),
    Square_(Square, RBid),
}

pub fn convert_rigidbody_to_collider_builder(rb: RigidBody) -> ColliderBuilder {
    // let cb: ColliderBuilder;
    match rb {
        RigidBody::Circle_(Circle { radius, .. }, _) => {
            ColliderBuilder::ball(radius).restitution(COEFF_RESTITUTION)
        }
        // .translation(vector![position.x, position.y]),
        RigidBody::Square_(
            Square {
                half_extent,
                rotation,
                ..
            },
            _,
        ) => ColliderBuilder::cuboid(half_extent.x, half_extent.y)
            .restitution(COEFF_RESTITUTION)
            .rotation(rotation),
        // _ => unreachable!(),
    }
}

// impl CollisionHandler for RigidBody {
//     fn check_world_collisions(&self) -> Option<Collision> {
//         match self {
//             RigidBody::Circle_(c, _) => c.check_world_collisions(),
//             RigidBody::Circle_(c, _) => c.check_world_collisions(),
//         }
//     }
//     fn resolve_world_collision(&mut self, has_crossed_boundaries: WorldCollisionInfo) {
//         match self {
//             RigidBody::Circle_(c, _) => return c.resolve_world_collision(has_crossed_boundaries),
//         }
//     }
// }

#[allow(dead_code)]
impl RigidBody {
    pub fn get_id(&self) -> RBid {
        match self {
            RigidBody::Circle_(Circle { .. }, id) => id.clone(),
            RigidBody::Square_(Square { .. }, id) => id.clone(),
        }
    }
    pub fn get_object(&self) -> Box<dyn GenericObject> {
        match self {
            RigidBody::Circle_(
                Circle {
                    radius,
                    position,
                    velocity,
                    color,
                },
                _,
            ) => Box::new(Circle {
                radius: *radius,
                position: *position,
                velocity: *velocity,
                color: *color,
            }),
            RigidBody::Square_(
                Square {
                    half_extent,
                    position,
                    velocity,
                    rotation: orientation,
                    color,
                },
                _,
            ) => Box::new(Square {
                half_extent: *half_extent,
                position: *position,
                velocity: *velocity,
                rotation: *orientation,
                color: *color,
            }),
        }
    }
    pub fn get_vertex_count(&self) -> u8 {
        match self {
            RigidBody::Circle_(_, _) => 32,
            RigidBody::Square_(_, _) => 4,
        }
    }
    pub fn to_polygon(&self, rotation: f32) -> Polygon {
        let inner_object = self.get_object();
        let long_radius = inner_object.get_radius();
        let position = inner_object.get_position();
        // TODO: Add z component. Temporary zero.
        let center_coordinate = Vec3::new(position.x, position.y, 0.);
        generate_polygon_triangles(
            self.get_vertex_count(),
            CustomVertex {
                position: center_coordinate,
                color: self.get_color().to_array(),
            },
            long_radius,
            rotation,
            self.get_color(),
        )
    }
    pub fn get_color(&self) -> Color32 {
        match self {
            RigidBody::Circle_(c, _) => c.color,
            RigidBody::Square_(c, _) => c.color,
        }
    }
    pub fn get_radius(&self) -> f32 {
        match self {
            RigidBody::Circle_(c, _) => c.radius,
            RigidBody::Square_(c, _) => c.get_radius(),
        }
    }
    pub fn get_position(&self) -> Vec3 {
        match self {
            RigidBody::Circle_(c, _) => c.position,
            RigidBody::Square_(c, _) => c.position,
        }
    }
    pub fn get_velocity(&self) -> Vec3 {
        match self {
            RigidBody::Circle_(c, _) => c.velocity,
            RigidBody::Square_(c, _) => c.velocity,
        }
    }
    pub fn update_position(&mut self, position: Vec3) {
        match self {
            RigidBody::Circle_(c, _) => c.position = position,
            RigidBody::Square_(c, _) => c.position = position,
        }
    }
    pub fn update_velocity(&mut self, velocity: Vec3) {
        match self {
            RigidBody::Circle_(c, _) => c.velocity = velocity,
            RigidBody::Square_(c, _) => c.velocity = velocity,
        }
    }
    fn get_debug(&self) -> String {
        let inner_object = self.get_object();
        inner_object.get_debug()
    }
    pub fn type_to_string(&self) -> &str {
        match self {
            RigidBody::Circle_(_, _) => "Circle",
            RigidBody::Square_(_, _) => "Square",
        }
    }
}
