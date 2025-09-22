use std::{fs::File, io::BufReader, sync::LazyLock};

use ecolor::Color32;
use glm::{Vec3, Vec4};
use obj::Obj;

pub mod ball;
pub mod cuboid;
pub mod cylinder;

pub const DEFAULT_FLOOR_COLOR: LazyLock<Vec4> = LazyLock::new(|| {
    let [r, g, b, a] = Color32::from_hex("#333333").unwrap().to_array();
    Vec4::new(
        r as f32 / 255.,
        g as f32 / 255.,
        b as f32 / 255.,
        a as f32 / 255.,
    )
});

static DEFAULT_FLOOR_OBJ: LazyLock<Obj> = LazyLock::new(|| {
    let file = File::open("resources/default_floor.obj").unwrap();
    obj::load_obj(BufReader::new(file)).unwrap()
}); // OK to unwrap because the file is known to be valid.

// Unit cube.
pub static DEFAULT_FLOOR_VERTICES: LazyLock<Vec<Vec3>> = LazyLock::new(|| {
    let obj = &*DEFAULT_FLOOR_OBJ;
    obj_loader::get_obj_vertices(obj)
});

pub static DEFAULT_FLOOR_NORMALS: LazyLock<Vec<Vec3>> = LazyLock::new(|| {
    let obj = &*DEFAULT_FLOOR_OBJ;
    obj_loader::get_obj_normals(obj)
});

pub static FLOOR_INDICES: LazyLock<Vec<u16>> = LazyLock::new(|| {
    let obj = &*DEFAULT_FLOOR_OBJ;
    obj_loader::get_obj_indices(obj)
});

pub mod obj_loader {
    use glm::Vec3;
    use obj::Obj;

    pub fn get_obj_vertices(obj: &Obj) -> Vec<Vec3> {
        let mut vertices = Vec::new();
        for vertex in obj.vertices.clone() {
            let [x, y, z] = vertex.position;
            vertices.push(Vec3::new(x, y, z));
        }
        vertices
    }

    pub fn get_obj_normals(obj: &Obj) -> Vec<Vec3> {
        let mut normals = Vec::new();
        for vertex in obj.vertices.clone() {
            let [x, y, z] = vertex.normal;
            normals.push(Vec3::new(x, y, z));
        }
        normals
    }

    pub fn get_obj_indices(obj: &Obj) -> Vec<u16> {
        obj.indices.clone()
    }
}
