use super::ball::RawBall;
use super::cube::RawCube;
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
    fn get_init_position(&self) -> Vec3;
    fn get_init_velocity(&self) -> Vec3;
}

type RBid = u8;
#[derive(Clone, Debug, PartialEq)]
pub enum RigidBody {
    Ball_(RawBall, RBid),
    Cube_(RawCube, RBid),
}

pub fn convert_rigidbody_to_collider_builder(rb: RigidBody) -> ColliderBuilder {
    // let cb: ColliderBuilder;
    match rb {
        RigidBody::Ball_(RawBall { radius, .. }, _) => {
            ColliderBuilder::ball(radius).restitution(COEFF_RESTITUTION)
        }
        // ball .translation(vector![position.x, position.y]),
        RigidBody::Cube_(
            RawCube {
                half_extent,
                rotation,
                ..
            },
            _,
        ) => ColliderBuilder::cuboid(half_extent.x, half_extent.y, half_extent.z)
            .restitution(COEFF_RESTITUTION)
            .rotation(rotation),
        // _ => unreachable!(),
    }
}

#[allow(dead_code)]
impl RigidBody {
    pub fn get_id(&self) -> RBid {
        match self {
            RigidBody::Ball_(RawBall { .. }, id) => id.clone(),
            RigidBody::Cube_(RawCube { .. }, id) => id.clone(),
        }
    }
    pub fn get_object(&self) -> Box<dyn GenericObject> {
        match self {
            RigidBody::Ball_(
                RawBall {
                    radius,
                    init_position: position,
                    init_velocity: velocity,
                    color,
                },
                _,
            ) => Box::new(RawBall {
                radius: *radius,
                init_position: *position,
                init_velocity: *velocity,
                color: *color,
            }),
            RigidBody::Cube_(
                RawCube {
                    half_extent,
                    init_position: position,
                    init_velocity: velocity,
                    rotation: orientation,
                    color,
                },
                _,
            ) => Box::new(RawCube {
                half_extent: *half_extent,
                init_position: *position,
                init_velocity: *velocity,
                rotation: *orientation,
                color: *color,
            }),
        }
    }
    pub fn get_vertex_count(&self) -> u8 {
        match self {
            RigidBody::Ball_(_, _) => 32,
            RigidBody::Cube_(_, _) => 8,
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
            RigidBody::Ball_(c, _) => c.color,
            RigidBody::Cube_(c, _) => c.color,
        }
    }
    pub fn get_radius(&self) -> f32 {
        match self {
            RigidBody::Ball_(c, _) => c.radius,
            RigidBody::Cube_(c, _) => c.get_radius(),
        }
    }
    pub fn get_init_position(&self) -> Vec3 {
        match self {
            RigidBody::Ball_(c, _) => c.get_init_position(),
            RigidBody::Cube_(c, _) => c.get_init_position(),
        }
    }
    pub fn get_init_velocity(&self) -> Vec3 {
        match self {
            RigidBody::Ball_(c, _) => c.get_init_velocity(),
            RigidBody::Cube_(c, _) => c.get_init_velocity(),
        }
    }
    fn get_debug(&self) -> String {
        let inner_object = self.get_object();
        inner_object.get_debug()
    }
    pub fn type_to_string(&self) -> &str {
        match self {
            RigidBody::Ball_(_, _) => "Ball",
            RigidBody::Cube_(_, _) => "Cube",
        }
    }
}
