#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::single_match)]

use ecolor::Color32;
use tracing::{error, info, instrument::WithSubscriber, warn};
use tracing_subscriber::util::SubscriberInitExt;
use vulkano::pipeline::PipelineShaderStageCreateInfo;
use vulkys::physics::rigidbody::RigidBody;
use vulkys::physics::scene::{Scene, SceneInfo};
use vulkys::renderer::vk_core::VulkanoContext;
use vulkys::renderer::vk_primitives::create_memory_allocator;
use vulkys::{physics::circle::Circle, renderer::vk_core::WindowEventHandler};

use vulkys::FVec2;

fn main() -> anyhow::Result<()> {
    let log_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("run.log")?;
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

    Ok(())
}
