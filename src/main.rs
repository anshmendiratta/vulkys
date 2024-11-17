#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::single_match)]

use tracing::{info, instrument::WithSubscriber};
use vulkano::pipeline::PipelineShaderStageCreateInfo;
use vulkys::{
    gui::draw::Content,
    renderer::vk_core::{get_compute_pipeline, WindowEventHandler},
};

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    // Running simulation
    let windowcx_handler = WindowEventHandler::new();
    windowcx_handler.run_inner();

    info!("WORKED");

    // Opening initial GUI
    // eframe::run_native(
    //     "Physics engine!",
    //     eframe::NativeOptions::default(),
    //     Box::new(|_cc| Box::<Content>::default()),
    // )?;

    Ok(())
}
