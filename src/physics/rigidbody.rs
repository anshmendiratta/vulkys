use super::circle::Circle;
use crate::renderer::vk_proc_func::{generate_polygon_triangles, Polygon};
use crate::FVec2;
use serde::Serialize;

#[derive(PartialEq, Clone, Copy, Serialize)]
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
    fn get_position(&self) -> FVec2;
}

type RBid = u8;
#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum RigidBody {
    Circle_(Circle, RBid),
}

#[allow(dead_code)]
impl RigidBody {
    pub fn get_id(&self) -> RBid {
        match self {
            RigidBody::Circle_(Circle { .. }, id) => id.clone(),
        }
    }
    pub fn get_object(&self) -> impl GenericObject {
        match self {
            RigidBody::Circle_(
                Circle {
                    radius,
                    position,
                    velocity,
                },
                _,
            ) => Circle {
                radius: *radius,
                position: *position,
                velocity: *velocity,
            },
        }
    }
    pub fn get_vertex_count(&self) -> u8 {
        match self {
            RigidBody::Circle_(_, _) => 32,
        }
    }
    pub fn to_polygon(&self) -> Polygon {
        let inner_object = self.get_object();
        let radius = inner_object.get_radius();
        let position = inner_object.get_position();
        let center_coordinate = FVec2::new(position.x, position.y);
        generate_polygon_triangles(
            self.get_vertex_count(),
            center_coordinate.to_custom_vertex(),
            radius,
        )
    }
    pub fn get_radius(&self) -> f32 {
        match self {
            RigidBody::Circle_(c, _) => c.radius,
        }
    }
    pub fn get_position(&self) -> FVec2 {
        match self {
            RigidBody::Circle_(c, _) => c.position,
        }
    }
    pub fn get_velocity(&self) -> FVec2 {
        match self {
            RigidBody::Circle_(c, _) => c.velocity,
        }
    }
    pub fn update_position(&mut self, position: FVec2) {
        match self {
            RigidBody::Circle_(c, _) => c.position = position,
        }
    }
    pub fn update_velocity(&mut self, velocity: FVec2) {
        match self {
            RigidBody::Circle_(c, _) => c.velocity = velocity,
        }
    }
    fn get_debug(&self) -> String {
        let inner_object = self.get_object();
        inner_object.get_debug()
    }
    fn type_to_string(&self) -> &str {
        match self {
            RigidBody::Circle_(_, _) => "Circle",
        }
    }
}
