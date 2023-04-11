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

const CONTACT_TIME: f32 = 0.01;

fn calculate_torque<RigidBody>(force: Vec<f32>, object: RigidBody) -> f32 {
   force
}

fn get_tangential_velocity<RigidBody>(object: RigidBody) -> f32 {
   2 * PI * object.get_radius() * 1/CONTACT_TIME
}

fn calculate_angularvelocity<RigidBody>(object: RigidBody) -> f32 {
   let angularmomentum: f32 = calculate_angularmomentum(object);
   let momentofinertia: f32 = calculate_momentofinertia(object);

   angularmomentum/momentofinertia
}

fn calculate_angularmomentum<RigidBody>(object: RigidBody) -> f32 {
   RigidBody.get_mass() * RigidBody.get_radius() * RigidBody.get_tangential_velocity()
}

/// A positive sign indicates 
fn calculate_momentofinertia<RigidBody>(object: RigidBody) -> f32 {
   object.get_mass() * object.get_radius().pow(2)
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