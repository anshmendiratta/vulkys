use super::circle::Circle;
use super::collision::{Collision, CollisionHandler};
use crate::renderer::vk_proc_func::{generate_polygon_triangles, Polygon};
use crate::FVec2;
use ecolor::Color32;
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
    fn get_color(&self) -> Color32;
}

type RBid = u8;
#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum RigidBody {
    Circle_(Circle, RBid),
}

impl CollisionHandler for RigidBody {
    fn check_collisions(&self) -> (Option<Vec<Collision>>, (bool, bool)) {
        match self {
            RigidBody::Circle_(c, _) => c.check_collisions(),
        }
    }
    fn resolve_object_collision(&mut self) {
        match self {
            RigidBody::Circle_(c, _) => return c.resolve_object_collision(),
        }
    }
    fn resolve_world_collision(&mut self, has_crossed_boundaries: (bool, bool)) {
        match self {
            RigidBody::Circle_(c, _) => return c.resolve_world_collision(has_crossed_boundaries),
        }
    }
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
                    color,
                },
                _,
            ) => Circle {
                radius: *radius,
                position: *position,
                velocity: *velocity,
                color: *color,
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
            center_coordinate.to_custom_vertex(Some(self.get_color())),
            radius,
            self.get_color(),
        )
    }
    pub fn get_color(&self) -> Color32 {
        match self {
            RigidBody::Circle_(c, _) => c.color,
        }
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
