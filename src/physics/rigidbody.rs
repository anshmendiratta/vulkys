use super::circle::Circle;
use egui::Vec2;

#[derive(PartialEq)]
pub enum RigidBodySelection {
    Circle_,
}

impl RigidBodySelection {
    pub fn to_string(&self) -> &str {
        match self {
            RigidBodySelection::Circle_ => "Circle",
        }
    }
}

pub enum RigidBody {
    Circle_(Circle),
}

pub trait Updateable {
    fn update_position(&mut self, velocity: Vec2);
    fn update_velocity(&mut self, acceleration: Vec2);
}
pub trait GenericObject: Updateable {
    fn get_debug(&self) -> &str;
}

impl RigidBody {
    pub fn get_inner(&self) -> impl GenericObject {
        match self {
            RigidBody::Circle_(Circle {
                radius,
                position,
                velocity,
            }) => Circle {
                radius: *radius,
                position: *position,
                velocity: *velocity,
            },
        }
    }
    pub fn get_vertex_count(&self) -> u8 {
        match self {
            RigidBody::Circle_(Circle {
                radius: _,
                position: _,
                velocity: _,
            }) => 32,
        }
    }
}
