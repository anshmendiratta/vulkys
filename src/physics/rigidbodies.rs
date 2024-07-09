use crate::util::type_traits::*;
use strum_macros::{Display, EnumCount, EnumString};

/// A Rust feature allow for inheritance-like behavior. This trait/property is applied to every rigid body and requires them to have their fields be mutated.
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

/// A convenient enum (collection of types) used in ui.rs.
#[derive(Debug, Display, EnumCount, EnumString, PartialEq)]
pub enum RigidBodySelection {
    None(usize),
    RigidBody,
    Ball,
}
/// The parent struct of all rigid bodies with standard fields.
#[derive(Debug, PartialEq, Clone, Default, Copy)]
pub struct RigidBody {
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub mass: f64,
    pub radius: f64,
}

/// Meta-programming methods from type-traits. The most useful here is to print the struct's name.
impl MetaMethods for RigidBody {}

/// The first rigidbody with parent struct RigidBody.
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
        &self
    }
}

impl MetaMethods for Ball {}

/// The aforementioned trait being implemented. This is used less for defining methods and more for filtering out specific objects that can be passed into functions.
impl Updateable for Ball {
    fn update_velocity(&mut self, dt: f64) {}
    fn update_position(&mut self, dt: f64) {}
    fn update_acceleration(&mut self) {}
}

/// A subtrait of Updateable. It defines getters and setters for the rigid body.
pub trait HandleData {
    fn get_mass(&self) -> f64;

    fn get_radius(&self) -> f64;

    fn get_position(&self) -> (f64, f64);
    fn set_position(&mut self, new_position: (f64, f64));

    fn get_velocity(&self) -> (f64, f64);
    fn set_velocity(&mut self, new_velocity: (f64, f64));

    fn get_angular_velocity(&self) -> f64;
    fn set_angular_velocity(&mut self, new_angular_velocity: f64);

    fn get_acceleration(&self) -> (f64, f64);
    fn set_acceleration(&mut self, acceleration: (f64, f64));
}

/// Defining the above trait specifically for rigidbody and outputting the relevant fields.
impl HandleData for Ball {
    fn get_mass(&self) -> f64 {
        self.mass
    }

    fn get_radius(&self) -> f64 {
        self.radius
    }

    fn get_position(&self) -> (f64, f64) {
        self.position
    }
    fn set_position(&mut self, new_position: (f64, f64)) {
        self.position = new_position
    }

    fn get_velocity(&self) -> (f64, f64) {
        self.velocity
    }
    fn set_velocity(&mut self, new_velocity: (f64, f64)) {
        self.velocity = new_velocity
    }

    fn get_angular_velocity(&self) -> f64 {
        self.angular_velocity
    }
    fn set_angular_velocity(&mut self, new_angular_velocity: f64) {
        self.angular_velocity = new_angular_velocity
    }

    fn get_acceleration(&self) -> (f64, f64) {
        self.acceleration
    }
    fn set_acceleration(&mut self, new_acceleration: (f64, f64)) {
        self.acceleration = new_acceleration
    }
}

/// Defining methods for the Ball struct that otherwise can't be implemented in HandleData.
impl Ball {
    pub fn make_from_function(
        &self,
        mass: f64,
        radius: f64,
        position: (f64, f64),
        velocity: (f64, f64),
    ) -> Ball {
        Ball {
            mass,
            radius,
            position,
            velocity,
            acceleration: (0.0, 0.0),
            angular_velocity: 0.0,
        }
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

/// Obligatory implemention of a default state.
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
