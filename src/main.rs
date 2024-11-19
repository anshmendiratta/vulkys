#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::single_match)]

use tracing::{error, info, instrument::WithSubscriber, warn};
use vulkano::pipeline::PipelineShaderStageCreateInfo;
use vulkys::{
    gui::draw::Content,
    renderer::vk_core::{get_compute_pipeline, WindowEventHandler},
};

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    // Opening initial GUI
    // let gui_content = Box::new(Content::default());
    // eframe::run_native(
    //     "Physics engine!",
    //     eframe::NativeOptions::default(),
    //     Box::new(|_cc| gui_content),
    // )?;

    // Running simulation
    let windowcx_handler = WindowEventHandler::new();
    match windowcx_handler.run() {
        Ok(_) => info!("Worked"),
        _ => error!("Couldn't read objects.json file"),
    };

    Ok(())
}
