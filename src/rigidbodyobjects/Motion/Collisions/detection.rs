mod definition;
mod LinearQueue;
// mod

// objects, their attributes

fn detect_collisions(
    world: World,
    boundary: Vec<f32>,
    time: f32,
    dt: f32,
) -> LinearQueue<Collision> {
    let collisions: LinearQueue<Option<Collision>> = LinearQueue::new();
    let boundary_x: Vec<f32> = boundary.get_xrange();
    let boundary_y: Vec<f32> = boundary.get_yrange();

    for object in &world.get_objects() {
        let position: Vec<f32> = object.get_position();
        
        if position[0] < boundary_x[0] || position[0] > boundary_x[0] {
            collisions.enqueue(Collision { time: time});
        }

        if position[1] < boundary_y[0] || position[1] > boundary_y[1] {
            collisions.enqueue(Collision { time: time })
        }
    }

    collisions
}
