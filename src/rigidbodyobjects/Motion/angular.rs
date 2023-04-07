/// The return type of `Vec<f32, bool>` is my angular velocity pseudovector represention where the arguments denote the angular velocity in rad/s and the orientation (1 for clockwise, 0 otherwise), respectively.
fn calculate_angularvelocity() -> Vec<f32, bool> {
   calculate_angularmomentum()/calculate_momentofinertia() 
}

fn calculate_angularmomentum() {}

fn calculate_momentofinertia() {}