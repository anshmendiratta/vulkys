use crate::core::type_traits::*;
use core::fmt;

pub trait Updateable: HandleData {
    fn update_velocity(&mut self, dt: f64) {
        let mut velocity: (f64, f64) = self.get_velocity();
        let acceleration: (f64, f64) = self.get_acceleration();
        velocity.0 += acceleration.0 * dt;
        velocity.1 += acceleration.1 * dt;

        self.set_velocity(velocity);
    }
    fn update_position(&mut self, dt: f64) {
        let mut position: (f64, f64) = self.get_position();
        let velocity: (f64, f64) = self.get_velocity();
        position.0 += velocity.0 * dt;
        position.1 += velocity.1 * dt;

        self.set_position(position);
    }
    fn update_acceleration(&mut self);
}

#[derive(Debug, PartialEq)]
pub enum RigidBodySelection {
    None,
    RigidBody,
    Ball,
}

impl fmt::Display for RigidBodySelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RigidBodySelection::RigidBody => f.write_str("RigidBody"),
            RigidBodySelection::Ball => f.write_str("Ball"),
            RigidBodySelection::None => f.write_str("None"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Copy)]
pub struct RigidBody {
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub mass: f64,
    pub radius: f64,
}

impl MetaMethods for RigidBody {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ball {
    pub mass: f64,
    pub radius: f64,
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub acceleration: (f64, f64),
    pub angular_velocity: f64,
}

impl AsRef<Ball> for Ball {
    fn as_ref(self: &Ball) -> &Ball {
        self
    }
}

impl MetaMethods for Ball {}

impl Updateable for Ball {
    fn update_velocity(&mut self, _dt: f64) {}
    fn update_position(&mut self, _dt: f64) {}
    fn update_acceleration(&mut self) {}
}

pub trait HandleData {
    fn get_mass(&self) -> f64;
    fn get_radius(&self) -> f64;
    fn set_position(&mut self, new_position: (f64, f64));
    fn get_position(&self) -> (f64, f64);
    fn set_velocity(&mut self, new_velocity: (f64, f64));
    fn get_velocity(&self) -> (f64, f64);
    fn set_angular_velocity(&mut self, new_angular_velocity: f64);
    fn get_angular_velocity(&self) -> f64;
    fn set_acceleration(&mut self, acceleration: (f64, f64));
    fn get_acceleration(&self) -> (f64, f64);
    fn get_object_name(&self) -> &str;
    fn get_debug_print(&self) -> String {
        format!(
            "{}: p=({},{}),v=({},{})",
            self.get_object_name(),
            self.get_position().0,
            self.get_position().1,
            self.get_velocity().0,
            self.get_velocity().1,
        )
    }
    fn get_tangential_velocity(&self) -> f64 {
        self.get_angular_velocity() * self.get_radius()
    }
}

impl HandleData for Ball {
    fn get_radius(&self) -> f64 {
        // NOTE: effective radius used for calculations
        self.radius
    }
    fn set_position(&mut self, new_position: (f64, f64)) {
        self.position = new_position
    }
    fn get_position(&self) -> (f64, f64) {
        self.position
    }
    fn set_velocity(&mut self, new_velocity: (f64, f64)) {
        self.velocity = new_velocity
    }
    fn get_velocity(&self) -> (f64, f64) {
        self.velocity
    }
    fn set_angular_velocity(&mut self, new_angular_velocity: f64) {
        self.angular_velocity = new_angular_velocity
    }
    fn get_angular_velocity(&self) -> f64 {
        self.angular_velocity
    }
    fn set_acceleration(&mut self, new_acceleration: (f64, f64)) {
        self.acceleration = new_acceleration
    }
    fn get_acceleration(&self) -> (f64, f64) {
        self.acceleration
    }
    fn get_object_name(&self) -> &str {
        "Ball"
    }
    fn get_mass(&self) -> f64 {
        self.mass
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            mass: 1.0,
            radius: 1.0,
            position: (0.0, 0.0),
            velocity: (0.0, 0.0),
            acceleration: (0.0, 0.0),
            angular_velocity: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ball_velocity() {
        let v: (f64, f64) = (1.0, 2.0);
        let mut b = Ball::default();
        b.set_velocity(v);

        assert_eq!(b.get_velocity(), v)
    }
}
