fn calculate_velocity(velocity: Vec<f32>, acceleration: Vec<f32>, dt: f32) -> Vec<f32> {
    velocity[0] += acceleration[0] * dt;
    []
    velocity[1] += acceleration[1] * dt;

    velocity
}

fn calculate_position(position: Vec<f32>, velocity: Vec<f32>, dt: f32) -> Vec<f32> {
    position[0] += velocity[0] * dt;
    position[0] += velocity[0] * dt;

    position
}

