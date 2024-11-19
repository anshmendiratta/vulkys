use super::circle::Circle;
use crate::renderer::vk_proc_func::{generate_polygon_triangles, Polygon, Triangle};
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

type RBid = u8;
#[derive(Serialize, Clone, Debug)]
pub enum RigidBody {
    Circle_(Circle, RBid),
}

pub trait Updateable {
    fn update_position(&mut self, velocity: FVec2);
    fn update_velocity(&mut self, acceleration: FVec2);
}
pub trait GenericObject: Updateable {
    fn get_debug(&self) -> &str;
}

impl RigidBody {
    pub fn get_id(&self) -> Option<RBid> {
        match self {
            RigidBody::Circle_(Circle { .. }, id) => Some(id.clone()),
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
            RigidBody::Circle_(Circle { .. }, _) => 32,
        }
    }
    pub fn to_polygon(&self) -> Polygon {
        let real_self = match self {
            RigidBody::Circle_(circle, ..) => circle,
        };
        let center_coordinate = FVec2::new(real_self.position.x, real_self.position.y);
        generate_polygon_triangles(
            self.get_vertex_count(),
            center_coordinate.to_custom_vertex(),
        )
    }
}
