use std::{fs::File, io::BufReader, sync::LazyLock};

use glm::Vec3;
use obj::Obj;

static DEFAULT_CUBOID_OBJ: LazyLock<Obj> = LazyLock::new(|| {
    let file = File::open("resources/default_cuboid.obj").unwrap();
    obj::load_obj(BufReader::new(file)).unwrap()
}); // OK to unwrap because the file is known to be valid.

// Unit cube.
pub static DEFAULT_CUBOID_VERTICES: LazyLock<Vec<Vec3>> = LazyLock::new(|| {
    let obj = &*DEFAULT_CUBOID_OBJ;
    let mut vertices = Vec::new();
    for vertex in obj.vertices.clone() {
        let [x, y, z] = vertex.position;
        vertices.push(Vec3::new(x, y, z));
    }
    vertices
});

pub static DEFAULT_CUBOID_NORMALS: LazyLock<Vec<Vec3>> = LazyLock::new(|| {
    let obj = &*DEFAULT_CUBOID_OBJ;
    let mut normals = Vec::new();
    for vertex in obj.vertices.clone() {
        let [x, y, z] = vertex.normal;
        normals.push(Vec3::new(x, y, z));
    }
    normals
});

pub static CUBOID_INDICES: LazyLock<Vec<u16>> = LazyLock::new(|| {
    let obj = &*DEFAULT_CUBOID_OBJ;
    obj.indices.clone()
});
