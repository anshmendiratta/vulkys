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
    // let mut eframe_native_options = eframe::NativeOptions::default();
    // eframe_native_options.run_and_return = true;
    // eframe::run_native(
    //     "Physics engine!",
    //     eframe_native_options,
    //     Box::new(|_cc| gui_content),
    // )?;

    info!("now after eframe closed");
    // Running simulation
    let windowcx_handler = WindowEventHandler::new();
    match windowcx_handler.run() {
        Ok(_) => info!("Worked"),
        _ => error!("Couldn't read objects.json file"),
    };

    Ok(())
}
