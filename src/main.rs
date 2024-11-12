#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::single_match)]

use tracing::info;
use vulkano::pipeline::PipelineShaderStageCreateInfo;
use vulkys::renderer::vk_core::{get_compute_pipeline, WindowEventHandler};

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let win_ctx_handler = WindowEventHandler::new();
    // win_ctx_handler.run();

    let compute_pipeline = get_compute_pipeline(win_ctx_handler.vkcx());

    info!("WORKED");

    // let options = eframe::NativeOptions::default();
    // eframe::run_native(
    //     "Physics engine!",
    //     options,
    //     Box::new(|_cc| Box::<Content>::default()),
    // );
    Ok(())
}
