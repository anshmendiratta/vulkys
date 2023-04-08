#[derive(Debug)]
pub enum PseudoVector {
   Value(Vec<f32>),
   Orientation(bool) 
}

impl std::ops::Div for PseudoVector {
   type Output = Self;

   fn div(self, rhs: Self) -> Self::Output {
      match rhs {
         PseudoVector => return self::Value.map(|x| x/rhs::Value), 
         f32 => return self::Value/rhs
      } 
   }
}

fn calculate_angularvelocity() -> PseudoVector {
    let angularmomentum: f32 = calculate_angularmomentum();
    let momentofinertia: f32 = calculate_momentofinertia();

   angularmomentum/momentofinertia
}

fn calculate_angularmomentum() -> PseudoVector {}

fn calculate_momentofinertia() -> f32 {}

