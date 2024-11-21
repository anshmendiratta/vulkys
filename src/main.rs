#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::single_match)]

use tracing::{error, info, instrument::WithSubscriber, warn};
use vulkano::pipeline::PipelineShaderStageCreateInfo;
use vulkys::physics::rigidbody::RigidBody;
use vulkys::physics::scene::Scene;
use vulkys::{physics::circle::Circle, renderer::vk_core::WindowEventHandler};

use vulkys::FVec2;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    // Initialize objects
    let circle_1: RigidBody = RigidBody::Circle_(
        Circle {
            radius: 0.25,
            position: FVec2::new(0.5, 0.5),
            velocity: FVec2::new(0., 0.),
        },
        0,
    );
    let circle_2: RigidBody = RigidBody::Circle_(
        Circle {
            radius: 0.5,
            position: FVec2::new(-0.5, -0.5),
            velocity: FVec2::new(0., 0.),
        },
        1,
    );
    let objects: Vec<RigidBody> = vec![circle_1, circle_2];
    // Initialize scene
    let scene: Scene = Scene::with_objects(objects);
    // Running simulation
    scene.run();

    Ok(())
}
