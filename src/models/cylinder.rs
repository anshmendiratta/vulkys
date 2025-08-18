use std::{fs::File, io::BufReader, sync::LazyLock};

use glm::Vec3;
use obj::Obj;

use super::obj_loader;

static DEFAULT_CYLINDER_OBJ: LazyLock<Obj> = LazyLock::new(|| {
    let file = File::open("resources/default_cylinder.obj").unwrap();
    obj::load_obj(BufReader::new(file)).unwrap()
}); // OK to unwrap because the file is known to be valid.

// Unit cube.
pub static DEFAULT_CYLINDER_VERTICES: LazyLock<Vec<Vec3>> = LazyLock::new(|| {
    let obj = &*DEFAULT_CYLINDER_OBJ;
    obj_loader::get_obj_vertices(obj)
});

pub static DEFAULT_CYLINDER_NORMALS: LazyLock<Vec<Vec3>> = LazyLock::new(|| {
    let obj = &*DEFAULT_CYLINDER_OBJ;
    obj_loader::get_obj_normals(obj)
});

pub static CYLINDER_INDICES: LazyLock<Vec<u16>> = LazyLock::new(|| {
    let obj = &*DEFAULT_CYLINDER_OBJ;
    obj_loader::get_obj_indices(obj)
});
