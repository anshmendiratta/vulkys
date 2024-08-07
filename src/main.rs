#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::single_match)]

use crate::render::ui::Content;
use render::vulkano::make_vulkano_device;

mod core;
mod ffmpeg;
mod physics;
mod render;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    make_vulkano_device();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Physics engine!",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    )
}
