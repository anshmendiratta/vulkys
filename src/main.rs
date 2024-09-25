#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::single_match)]

use nea::render::ui::Content;
use nea::render::vulkano::make_vulkano_device;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let _ = make_vulkano_device();

    // let options = eframe::NativeOptions::default();
    // eframe::run_native(
    //     "Physics engine!",
    //     options,
    //     Box::new(|_cc| Box::<Content>::default()),
    // )
    Ok(())
}
