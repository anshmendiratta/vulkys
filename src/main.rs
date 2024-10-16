#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::single_match)]

use tracing::info;
use vulkys::render::vulkano::{create_window, VulkanoContext};

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let vk_ctx = VulkanoContext::new();
    let win_ctx = vk_ctx.into();

    create_window(win_ctx);
    info!("WORKED");

    // let options = eframe::NativeOptions::default();
    // eframe::run_native(
    //     "Physics engine!",
    //     options,
    //     Box::new(|_cc| Box::<Content>::default()),
    // );
    Ok(())
}
