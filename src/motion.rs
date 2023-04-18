
pub mod PhysicsMath {
   // use std::f64::consts::PI;
   use crate::rigidbodies::*;
   use libm::atan2;
   
   type Coordinate = Vec<f64>;
   type Radians = f64;
   
   pub fn calculate_angle_between_coms(com1: Coordinate, com2: Coordinate) -> f64 {
      let x_diff: f64 = com1[0] - com2[0];
      let y_diff: f64 = com1[1] - com2[1];
      atan2(y_diff, x_diff)
   }
   
   pub fn get_point_of_contact(object1: RigidBody, object2: RigidBody) -> Coordinate {
      let com1: Coordinate = object1.get_position().to_vec();
      let com2: Coordinate = object2.get_position().to_vec();
      vec![(com1[0] + com2[0]) / 2.0, (com1[1] + com2[1]) / 2.0]
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

pub mod Physics {
   use crate::rigidbodies::*;
   
   use libm::atan2;
   const CONTACT_TIME: f64 = 0.01;
   type Force = Vec<f64>;

   pub fn update_velocity<RigidBody: HandleData>(object: RigidBody, dt: f64) {
      let velocity = object.get_velocity();
      let acceleration = object.get_acceleration();
      velocity[0] += acceleration[0] * dt;
      velocity[1] += acceleration[1] * dt

      object.set_velocity(velocity)
      object.set_acceleration(acceleration)
   }

   pub fn update_position(object: RigidBody, dt: f64) {
      object.position[0] += object.velocity[0] * dt;
      object.position[1] += object.velocity[1] * dt
   }
  
   pub fn calculate_angle(body1: RigidBody, body2: RigidBody) -> f64 {
      let x_diff: f64 = body1.position[0] - body2.position[0];
      let y_diff: f64 = body1.position[1] - body2.position[1];
  
      atan2(y_diff, x_diff)    
   }

   // fn calculate_torque<RigidBody>(force: Force, object: RigidBody) -> f64 {
      // force
   // }
   
   pub fn update_angular_velocity<RigidBody>(object: RigidBody) -> f64 {
      // let angularmomentum: f64 = calculate_angularmomentum(object);
      // let momentofinertia: f64 = calculate_momentofinertia(object);
      
      // angularmomentum/momentofinertia
      0.0
   }
   
   // pub fn calculate_angularmomentum<RigidBody>(object: RigidBody) -> f64 {
      // RigidBody.get_mass() * RigidBody.get_radius() * RigidBody.get_tangential_velocity()
   // }
   
   // pub fn calculate_momentofinertia<RigidBody>(object: RigidBody) -> f64 {
      // object.get_mass() * object.get_radius().pow(2)
   // }
}

// #[cfg(test)]
// mod test {
//    #[test]
//    fn test_tangential() {
//       let object = RigidBody::Ball {
//          mass: 1.0,
//          radius: 1.0,
//          position: vec![0.0, 0.0],
//          velocity: vec![0.0, 0.0]
//       };

//       assert_eq!(get_tangential_velocity(object))
//    }
// }