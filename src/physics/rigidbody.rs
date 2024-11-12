use egui::Vec2;

#[derive(PartialEq)]
pub enum RigidBodySelection {
    Circle,
}

impl RigidBodySelection {
    pub fn to_string(&self) -> &str {
        match self {
            RigidBodySelection::Circle => "Circle",
        }
    }
}

const DELTA_TIME: f64 = 0.1;
pub trait Updateable {
    fn update_position(&mut self, velocity: Vec2);
    fn update_velocity(&mut self, acceleration: Vec2);
}
pub trait GenericObject: Updateable {
    fn get_debug(&self) -> &str {
        todo!();
    }
}
