fn resolve_collision(collision: Collision, object: RigidBody) {
    let object = collision.get_object();
    let velocity = &object.get_velocity();

    if collision.get_axis() == "x" {
        velocity[0] *= -1
    } else {
        velocity[1] *= -1
    }
}
