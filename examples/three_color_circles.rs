#![allow(unused_variables)]

use ecolor::Color32;
use nalgebra::Rotation3;
use nalgebra_glm::Vec3;
use vulkys::{
    physics::{
        ball::RawBall,
        cuboid::RawCuboid,
        rigidbody::RigidBody,
        scene::{Scene, SceneInfo},
    },
    vulkan::core::App,
};
use winit::event_loop::EventLoop;

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
    let circle_1: RigidBody = RigidBody::Ball(
        RawBall {
            radius: 0.25,
            init_position: Vec3::new(0.5, 0.5, 0.5),
            init_velocity: Vec3::new(-1.5, -1.0, 0.),
            color: Color32::from_hex("#23F0C7").unwrap(),
        },
        0,
    );
    let circle_2: RigidBody = RigidBody::Ball(
        RawBall {
            radius: 0.25,
            init_position: Vec3::new(-0.5, 0.65, 1.),
            init_velocity: Vec3::new(1.5, 0., 0.),
            color: Color32::from_hex("#EF767A").unwrap(),
        },
        1,
    );
    let circle_3: RigidBody = RigidBody::Ball(
        RawBall {
            radius: 0.3,
            init_position: Vec3::new(0., -0.5, 0.),
            init_velocity: Vec3::new(2.75, 2.6, 0.),
            color: Color32::from_hex("#7D7ABC").unwrap(),
        },
        2,
    );
    let square_1: RigidBody = RigidBody::Cuboid(
        RawCuboid {
            half_extent: Vec3::new(0.2, 0.2, 0.2),
            init_position: Vec3::new(0., 0., 0.),
            init_velocity: Vec3::new(0., 0., 0.),
            color: Color32::from_hex("#32db49").unwrap(),
            init_rotation: Rotation3::from_euler_angles(0.5, 0.6, 0.6),
        },
        2,
    );
    let objects: Vec<RigidBody> = vec![square_1];
    // Initialize scene
    let scene_info = SceneInfo {
        objects,
        dt: 1e-3,
        gravity: 0.01,
    };
    let scene: Scene = Scene::with_info(scene_info);
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new(&event_loop, scene);
    // Running simulation. Ignore error.
    let _ = event_loop.run_app(&mut app);
}
