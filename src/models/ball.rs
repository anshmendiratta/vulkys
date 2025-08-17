use std::{fs::File, io::BufReader, sync::LazyLock};

use glm::Vec3;
use obj::Obj;

static DEFAULT_BALL_OBJ: LazyLock<Obj> = LazyLock::new(|| {
    let file = File::open("resources/default_ball.obj").unwrap();
    obj::load_obj(BufReader::new(file)).unwrap()
}); // OK to unwrap because the file is known to be valid.

// Unit cube.
pub static DEFAULT_BALL_VERTICES: LazyLock<Vec<Vec3>> = LazyLock::new(|| {
    let obj = &*DEFAULT_BALL_OBJ;
    let mut vertices = Vec::new();
    for vertex in obj.vertices.clone() {
        let pos = vertex.position;
        vertices.push(Vec3::new(pos[0], pos[1], pos[2]));
    }
    // dbg!(&vertices);
    vertices
});

pub static DEFAULT_BALL_NORMALS: LazyLock<Vec<Vec3>> = LazyLock::new(|| {
    let obj = &*DEFAULT_BALL_OBJ;
    let mut normals = Vec::new();
    for vertex in obj.vertices.clone() {
        let norm = vertex.normal;
        normals.push(Vec3::new(norm[0], norm[1], norm[2]));
    }
    normals
});

pub static BALL_INDICES: LazyLock<Vec<u16>> = LazyLock::new(|| {
    let obj = &*DEFAULT_BALL_OBJ;
    let mut indices = Vec::new();
    for index in obj.indices.clone() {
        indices.push(index);
    }
    dbg!(&indices);
    indices
});
