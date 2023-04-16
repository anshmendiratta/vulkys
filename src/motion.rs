// #[derive(Debug)]
// pub enum PseudoVector {
//    Value(Vec<f32>),
//    Orientation(bool) 
// }

// impl std::ops::Div for PseudoVector {
//    type Output = Self;

//    fn div(self, rhs: Self) -> Self::Output {
//       match rhs {
//          PseudoVector => return self::Value.map(|x| x/rhs::Value), 
//          f32 => return self::Value/rhs
//       } 
//    }
// }

mod rigidbodies;
use std::f32::consts::PI;

pub mod PhysicsMath {
   type Coordinate: Vec<f64>;

   pub fn calculate_angle_between_coms(com1: Coordinate, com2: Coordinate) -> f64 {
      let x_diff: f64 = com1[0] - com2[0];
      let y_diff: f64 = com1[1] - com2[1];
      atan2(y_diff, x_diff)
   }
   
   fn get_point_of_contact(object1: RigidBody, object2: RigidBody) -> Coordinate {
      let com1: Coordinate = object1.get_position();
      let com2: Coordinate = object2.get_position();
      vec![(com1[0] + com2[0]) / 2.0, (com1[1] + com2[1]) / 2.0]
   }
   
   pub fn calculate_distance_for_torque(object1: RigidBody, object2: RigidBody, contact_point: Coordinate) -> f64 {
      let obj1_position: Coordinate = object1.get_position();
      let obj2_position: Coordinate = object2.get_position();
      let angle: Radians = atan2();
   }
   
   pub fn get_tangential_velocity<RigidBody>(object: RigidBody) -> f32 {
      // 2 * PI * object.get_radius() * 1/CONTACT_TIME
   }
}

pub mod Physics {
   const CONTACT_TIME: f32 = 0.01;
   type Force = Vec<f64>;

   fn calculate_torque<RigidBody>(force: Force, object: RigidBody) -> f32 {
      force
   }
   
   fn calculate_angularvelocity<RigidBody>(object: RigidBody) -> f32 {
      let angularmomentum: f32 = calculate_angularmomentum(object);
      let momentofinertia: f32 = calculate_momentofinertia(object);
   
      angularmomentum/momentofinertia
   }
   
   fn calculate_angularmomentum<RigidBody>(object: RigidBody) -> f32 {
      RigidBody.get_mass() * RigidBody.get_radius() * RigidBody.get_tangential_velocity()
   }
   
   fn calculate_momentofinertia<RigidBody>(object: RigidBody) -> f32 {
      object.get_mass() * object.get_radius().pow(2)
   }
}

#[cfg(test)]
mod test {
   #[test]
   fn test_tangential() {
      let object = RigidBody::Ball {
         mass: 1.0,
         radius: 1.0,
         position: vec![0.0, 0.0],
         velocity: vec![0.0, 0.0]
      };

      assert_eq!(get_tangential_velocity(object))
   }
}