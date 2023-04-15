mod definition;
mod LinearQueue;

fn detect_world_collision(world: &World) -> LinearQueue<Collision> {
    let collisions: LinearQueue<Collision> = LinearQueue::new();
    let objects = world.get_objects();
    let boundary: Vec<Coordinate> = world.get_boundary();

    for object in objects {
        let position: Coordinate = object.get_position();
        // Can equally use boundary[1]
        if position[0].abs() >= boundary[0].abs() {
            collisions.enqueue(Collision::ObjWorld(object, world))
        } else {
            continue
        }
        if position[1].abs() >= boundary[1].abs() {
            collisions.enqueue(Collision::ObjWorld(object, world))
        } else {
            continue
        }
    }

    collisions
}


fn detect_balls_collision(ball1: RigidBody::Ball, ball2: RigidBody::Ball) -> Option<Collision::ObjObj(ball1, ball2)> {
    let centers_of_masses: Vec<Vec<f64>> = vec![ball1.get_position(), ball2.get_position()];
    let position_differential: Vec<f64> = vec![centers_of_masses[0][0] - centers_of_masses[1][0], centers_of_masses[0][1] - centers_of_masses[1][1]];
    
    if position_differential.magnitude() < ball1.get_radius() + ball2.get_radius() {
        return Collision::ObjObj(ball, ball2)
    }

    ()
}

// SAT collision detection
// Meant for non-circular objects
fn sat_collision_detection() {

}

/// DEPRECATED TO SAT COLLISION DETECTION
// fn detect_body_collision(world: World) -> LinearQueue<Vec<2, RigidBody>> {
//     let objects: Vec<RigidBody> = World.get_objects();

//     for idx in 2..=(objects.len() - 2) {
//         match (objects[idx], objects[idx + 1]) {
    //             (RigidBody::Ball, RigidBody::Ball) =>
    //         }
    // }
//     }