use libm::atan2;

fn calculate_velocity(velocity: Vec<f32>, acceleration: Vec<f32>, dt: f32) -> Vec<f32> {
    velocity[0] += acceleration[0] * dt;
    velocity[1] += acceleration[1] * dt;

    velocity
}

fn calculate_position(position: Vec<f32>, velocity: Vec<f32>, dt: f32) -> Vec<f32> {
    position[0] += velocity[0] * dt;
    position[1] += velocity[1] * dt;

    position
}

fn calculate_angle(centers_of_masses: Vec<2, Vec<f32>>) -> f32 {
    let x_diff: f32 = centers_of_masses[0][0] - centers_of_masses[1][0];
    let y_diff: f32 = centers_of_masses[0][1] - centers_of_masses[1][1];

    atan2(y_diff, x_diff)    
}