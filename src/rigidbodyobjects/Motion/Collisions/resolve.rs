fn resolve_collision(collisions: LinearQueue<Collision>) {
    for collision in collisions {
        match collision.get_kind() {
            boundary => resolve_boundary_collision(),
            object => resolve_object_collision()
        }
    }
}

fn resolve_boundary_collision(collision: Collision) {
    (xord, yord) = collision.get;
}