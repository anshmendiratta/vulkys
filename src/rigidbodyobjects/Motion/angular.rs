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

use crate::rigidbodyobjects::rigidbodies;

fn calculate_angularvelocity() -> f32 {
    let angularmomentum: f32 = calculate_angularmomentum();
    let momentofinertia: f32 = calculate_momentofinertia();

   angularmomentum/momentofinertia
}

fn calculate_angularmomentum() -> f32 {}

fn calculate_momentofinertia(object: rigidbodies::RigidBody) -> f32 {
   object.rigidbodies::get_mass() * object.rigidbodies::get_radius().pow(2)
}

