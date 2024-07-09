use crate::{physics::world::*, render::render::Content};

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();
    let _world_boundary: Boundary = Boundary {
        x_range: (-1.0, 1.0),
        y_range: (-1.0, 1.0),
    };

    let _world: World = World::new();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Physics engine!",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    )
}
