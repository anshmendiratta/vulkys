use ecolor::Color32;
use nalgebra_glm::Vec3;
use vulkys::physics::{
    circle::Circle,
    rigidbody::RigidBody,
    scene::{Scene, SceneInfo},
    square::Square,
};

fn main() {
    let open = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("run.log")
        .unwrap();
    let log_file = open;
    tracing_subscriber::fmt()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_line_number(true)
                .with_ansi(false)
                .with_source_location(true)
                .with_file(true),
        )
        .with_file(true)
        .with_writer(log_file)
        .init();

    // Initialize objects
    let circle_1: RigidBody = RigidBody::Circle_(
        Circle {
            radius: 0.25,
            position: Vec3::new(0.5, 0.5, 0.5),
            velocity: Vec3::new(-1.5, -1.0, 0.),
            color: Color32::from_hex("#23F0C7").unwrap(),
        },
        0,
    );
    let circle_2: RigidBody = RigidBody::Circle_(
        Circle {
            radius: 0.25,
            position: Vec3::new(-0.5, 0.65, 1.),
            velocity: Vec3::new(1.5, 0., 0.),
            color: Color32::from_hex("#EF767A").unwrap(),
        },
        1,
    );
    // let circle_3: RigidBody = RigidBody::Circle_(
    //     Circle {
    //         radius: 0.3,
    //         position: Vec3::new(0., -0.5),
    //         velocity: Vec3::new(2.75, 2.6),
    //         color: Color32::from_hex("#7D7ABC").unwrap(),
    //     },
    //     2,
    // );
    let square_1: RigidBody = RigidBody::Square_(
        Square {
            half_extent: Vec3::new(0.2, 0.2, 0.2),
            position: Vec3::new(0., -0.5, 1.),
            velocity: Vec3::new(2.75, 2.6, 0.),
            color: Color32::from_hex("#7D7ABC").unwrap(),
            rotation: 0.0,
        },
        2,
    );
    let objects: Vec<RigidBody> = vec![circle_1, circle_2, square_1];
    // Initialize scene
    let scene_info = SceneInfo {
        objects,
        dt: 1e-3,
        gravity: 1.0,
    };
    let scene: Scene = Scene::with_info(scene_info);
    // Running simulation
    scene.run();
}
