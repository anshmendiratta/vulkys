pub mod physics_math {
    use crate::physics::rigidbodies::*;
    use libm::atan2;
    use std::f64::consts::PI;

    type Coordinate = (f64, f64);
    type Radians = f64;

    const CONTACT_TIME: f64 = 0.1;

    pub fn calculate_angle_between_coms(com1: Coordinate, com2: Coordinate) -> Radians {
        let x_diff: f64 = com1.0 - com2.0;
        let y_diff: f64 = com1.1 - com2.1;
        atan2(y_diff, x_diff)
    }

    pub fn get_point_of_contact(object1: impl Updateable, object2: impl Updateable) -> Coordinate {
        let com1: Coordinate = object1.get_position();
        let com2: Coordinate = object2.get_position();
        ((com1.0 + com2.0) / 2.0, (com1.1 + com2.1) / 2.0)
    }

    pub fn calculate_distance_for_torque(
        object1: impl Updateable,
        object2: impl Updateable,
        contact_point: Coordinate,
    ) -> f64 {
        let obj1_position: Coordinate = object1.get_position();
        let obj2_position: Coordinate = object2.get_position();
        // let angle: Radians = atan2();
        todo!()
    }

    pub fn get_tangential_velocity(object: impl Updateable) -> f64 {
        2.0 * PI * object.get_radius() / CONTACT_TIME
    }
}

pub mod physics {
    use crate::physics::rigidbodies::*;
    use libm::atan2;

    type Force = [f64; 2];

    pub fn update_velocity<GenericBody: Updateable + ?Sized>(object: &mut GenericBody, dt: &f64) {
        let mut velocity = object.get_velocity();
        let acceleration = object.get_acceleration();
        velocity.0 += acceleration.0 * dt;
        velocity.1 += acceleration.1 * dt;

        object.set_velocity(velocity);
        object.set_acceleration(acceleration)
    }

    pub fn update_position<GenericBody: Updateable + ?Sized>(
        object: &mut GenericBody,
        dt: &f64,
    ) -> (f64, f64) {
        let mut position: (f64, f64) = object.get_position();
        let velocity: (f64, f64) = object.get_velocity();
        position.0 += velocity.0 * dt;
        position.1 += velocity.1 * dt;

        object.set_position(position);
        object.get_position()
    }

    pub fn calculate_angle(body1: impl Updateable, body2: impl Updateable) -> f64 {
        let x_diff: f64 = body1.get_position().0 - body2.get_position().0;
        let y_diff: f64 = body1.get_position().1 - body2.get_position().1;

        atan2(y_diff, x_diff)
    }

    fn calculate_torque(force: [f64; 2], object: impl Updateable) -> f64 {
        todo!()
    }

    pub fn update_angular_velocity(object: &mut impl Updateable) -> f64 {
        let angularmomentum: f64 = calculate_angularmomentum(object);
        let momentofinertia: f64 = calculate_momentofinertia(object);

        todo!()
    }

    pub fn calculate_angularmomentum(object: &mut impl Updateable) -> f64 {
        object.get_mass() * object.get_radius() * object.get_tangential_velocity()
    }

    pub fn calculate_momentofinertia(object: &mut impl Updateable) -> f64 {
        object.get_mass() * object.get_radius().powf(2.0)
    }
}

// Unit-tests
#[cfg(test)]
mod test {
    use crate::{
        physics::motion::physics::update_position,
        physics::rigidbodies::{Ball, Updateable},
    };

    #[test]
    fn check_velocity() {
        let mut object: Box<dyn Updateable + 'static> = Box::new(Ball::default());
        let p = (0.0, 0.0);
        let v = (1.0, 2.0);
        let dt = 0.5;

        let _ = &object.set_position(p);
        let _ = &object.set_velocity(v);

        let new_velocity = update_position(&mut *object, &dt);

        assert_eq!(new_velocity, (0.5, 1.0))
    }
}
