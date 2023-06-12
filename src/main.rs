// mod ui;
use NEA::ui::Content;

use NEA::world::*;
// mod world;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    // tracing_subscriber::fmt::init();
    let _world_boundary: Boundary = Boundary {
        x_range: (-1.0, 1.0),
        y_range: (-1.0, 1.0),
    };

    let _world: World = World::new();
    
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Physics engine!",
        options,
        Box::new(|_cc| Box::new(Content::default())),
    )
}
