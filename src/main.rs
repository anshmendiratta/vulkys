#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::single_match)]

use ecolor::Color32;
use tracing::{error, info, instrument::WithSubscriber, warn};
use tracing_subscriber::util::SubscriberInitExt;
use vulkano::pipeline::PipelineShaderStageCreateInfo;
use vulkys::physics::rigidbody::RigidBody;
use vulkys::physics::scene::Scene;
use vulkys::{physics::circle::Circle, renderer::vk_core::WindowEventHandler};

use vulkys::FVec2;

fn main() -> anyhow::Result<()> {
    let log_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("sim.log")?;
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
            position: FVec2::new(0.5, 0.5),
            velocity: FVec2::new(1.5, 0.),
            color: Color32::from_hex("#D36135").unwrap(),
        },
        0,
    );
    let circle_2: RigidBody = RigidBody::Circle_(
        Circle {
            radius: 0.25,
            position: FVec2::new(-0.5, 0.0),
            velocity: FVec2::new(0.5, 0.),
            color: Color32::from_hex("#7FB069").unwrap(),
        },
        1,
    );
    let circle_3: RigidBody = RigidBody::Circle_(
        Circle {
            radius: 0.1,
            position: FVec2::new(0., 0.5),
            velocity: FVec2::new(-0.75, 0.),
            color: Color32::from_hex("#ECE4B7").unwrap(),
        },
        2,
    );
    let objects: Vec<RigidBody> = vec![circle_1, circle_2, circle_3];
    // Initialize scene
    let scene: Scene = Scene::with_objects(objects);
    // Running simulation
    scene.run();

    Ok(())
}
