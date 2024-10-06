#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::single_match)]

use tracing::info;
use vulkys::render::vulkano::{draw_mandelbrot_fractal, Context};

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let ctx: Context = Context::default();

    draw_mandelbrot_fractal(ctx);
    info!("WORKED");

    // let options = eframe::NativeOptions::default();
    // eframe::run_native(
    //     "Physics engine!",
    //     options,
    //     Box::new(|_cc| Box::<Content>::default()),
    // );
    Ok(())
}
