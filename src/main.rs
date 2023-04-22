// mod ui;
use NEA::ui::Content;

use NEA::world::*;
// mod world;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    // tracing_subscriber::fmt::init();
    let world_boundary: Boundary = Boundary {
        x_range: (-1.0, 1.0),
        y_range: (-1.0, 1.0),
    };

    let _world: World = World {
        gravity: (0.0, -9.81),
        restitution: 1.0,
        objects: Vec::new(),
        boundary: world_boundary,
        time: 0.0,
        dt: 0.1,
    };

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "HELLO!",
        options,
        Box::new(|_cc| Box::new(Content::default())),
    )
}
