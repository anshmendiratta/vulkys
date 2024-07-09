/// A module (grouping of functions and other code) meant for dealing with the necessary math required.
pub mod physics_math {
    use crate::physics::rigidbodies::*;
    /// Useful function meant for finding the angle between the positive x-axis and the line connecting the origin to a coordinate.
    use libm::atan2;

    /// Definition of new datatype consisting of a tuple with two floating points.
    type Coordinate = (f64, f64);
    type Radians = f64;

    /// To be used for defining a component required for the calculation of torque. 'com' stands for center of mass.
    pub fn calculate_angle_between_coms(com1: Coordinate, com2: Coordinate) -> Radians {
        let x_diff: f64 = com1.0 - com2.0;
        let y_diff: f64 = com1.1 - com2.1;
        atan2(y_diff, x_diff)
    }

    /// Meant for calculating how objects would rebound upon collision.
    pub fn get_point_of_contact<T: Updateable>(object1: T, object2: T) -> Coordinate {
        let com1: Coordinate = object1.get_position();
        let com2: Coordinate = object2.get_position();
        ((com1.0 + com2.0) / 2.0, (com1.1 + com2.1) / 2.0)
    }

    // pub fn calculate_distance_for_torque(object1: RigidBody, object2: RigidBody, contact_point: Coordinate) -> f64 {
    //    let obj1_position: Coordinate = object1.get_position();
    //    let obj2_position: Coordinate = object2.get_position();
    //    // let angle: Radians = atan2();
    // }

    // pub fn get_tangential_velocity<RigidBody>(object: RigidBody) -> f64 {
    // 2 * PI * object.get_radius() * 1/CONTACT_TIME
    // }
}

/// Module — or a grouping of functions — that deal with rigid bodies directly, namely mutating their position and velocity.
pub mod physics {
    use crate::physics::rigidbodies::*;
    use libm::atan2;

    // type Force = Vec<f64>;

    /// First occurance of "Box," which is a smart pointer to an allocation of memory on the heap.
    pub fn update_velocity<T>(object: &mut Box<dyn Updateable>, dt: &f64)
    // This is where the Updateable trait is useful in defining what parameters can be passed in to the function.
    where
        T: Updateable,
    {
        let mut velocity = object.get_velocity();
        let acceleration = object.get_acceleration();
        velocity.0 += acceleration.0 * dt;
        velocity.1 += acceleration.1 * dt;

        object.set_velocity(velocity);
        object.set_acceleration(acceleration)
    }

    pub fn update_position<T>(object: &mut Box<dyn Updateable>, dt: &f64) -> (f64, f64)
    where
        T: Updateable,
    {
        let mut position: (f64, f64) = object.get_position();
        let velocity: (f64, f64) = object.get_velocity();
        position.0 += velocity.0 * dt;
        position.1 += velocity.1 * dt;

        object.set_position(position);
        return object.get_position();
    }

    /// Calculating the angle between the centers of mass of the bodies and the horizontal line passing through the body1.
    pub fn calculate_angle(body1: RigidBody, body2: RigidBody) -> f64 {
        let x_diff: f64 = body1.position.0 - body2.position.0;
        let y_diff: f64 = body1.position.0 - body2.position.1;

        atan2(y_diff, x_diff)
    }

    // fn calculate_torque<RigidBody>(force: Force, object: RigidBody) -> f64 {
    // force
    // }

    // pub fn update_angular_velocity<RigidBody>(_object: RigidBody) -> f64 {
    //     let angularmomentum: f64 = calculate_angularmomentum(object);
    //     let momentofinertia: f64 = calculate_momentofinertia(object);

    //     angularmomentum/momentofinertia
    //     0.0
    // }

    // pub fn calculate_angularmomentum<RigidBody>(object: RigidBody) -> f64 {
    // RigidBody.get_mass() * RigidBody.get_radius() * RigidBody.get_tangential_velocity()
    // }

    // pub fn calculate_momentofinertia<RigidBody>(object: RigidBody) -> f64 {
    // object.get_mass() * object.get_radius().pow(2)
    // }
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

        let new_velocity = update_position::<Ball>(&mut object, &dt);

        assert_eq!(new_velocity, (0.5, 1.0))
    }
}
