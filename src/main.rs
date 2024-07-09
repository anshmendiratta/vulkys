use nea::physics::world::*;
use nea::render::render::Content;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Physics engine!",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    )
}
