fn step(objects: Vec<RigidBody>, dt: f32) -> Vec<RigidBody> {
    for object in objects {
        object.velocity = calculate_velocity(object.get_velocity, dt);
        object.position = calculate_position(object.get_position, dt)
    }

    objects
}
