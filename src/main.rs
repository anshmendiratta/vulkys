mod ui;
use crate::ui::Content;
use NEA::boundary::*;
use NEA::ui::*;
use NEA::world::World;
mod world;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    // tracing_subscriber::fmt::init();
    let world_boundary: Boundary = Boundary {
        rangex: vec![-1.0, 1.0],
        rangey: vec![-1.0, 1.0],
    };

    let world: World = World {
        gravity: vec![0, -9.81],
        restitution: 1.0,
        objects: Vec::new(),
        boundary: world_boundary,
        time: 0.0,
    };

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "HELLO!",
        options,
        Box::new(|_cc| Box::new(Content::default())),
    )
}
