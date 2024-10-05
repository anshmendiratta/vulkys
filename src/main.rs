#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::single_match)]

use nea::render::vulkano::{
    do_image_creation,
    vulkan_primitives::{create_device_and_queues, create_instance},
};
use tracing::info;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let instance = create_instance();
    let (device, _, queues) = create_device_and_queues(instance.clone());

    do_image_creation(instance, (device, queues));
    info!("WORKED");

    // let options = eframe::NativeOptions::default();
    // eframe::run_native(
    //     "Physics engine!",
    //     options,
    //     Box::new(|_cc| Box::<Content>::default()),
    // );
    Ok(())
}
