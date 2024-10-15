#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::single_match)]

use tracing::info;
use vulkys::render::vulkano::{do_graphics_pipeline, Context};

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let ctx: Context = Context::default();

    do_graphics_pipeline(ctx);
    info!("WORKED");

    // let options = eframe::NativeOptions::default();
    // eframe::run_native(
    //     "Physics engine!",
    //     options,
    //     Box::new(|_cc| Box::<Content>::default()),
    // );
    Ok(())
}
