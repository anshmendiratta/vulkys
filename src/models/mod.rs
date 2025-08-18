use std::sync::LazyLock;

use glm::Vec3;

pub mod ball;
pub mod cuboid;
pub mod cylinder;
pub mod loader;

pub static FLOOR_PLANE_VERTICES: LazyLock<Vec<Vec3>> = LazyLock::new(|| {
    vec![
        Vec3::new(100., 0., 100.),
        Vec3::new(100., 0., -100.),
        Vec3::new(-100., 0., -100.),
        Vec3::new(-100., 0., 100.),
    ]
});

pub static FLOOR_PLANE_INDICES: LazyLock<Vec<u16>> = LazyLock::new(|| vec![0, 1, 2, 2, 3, 0]);
