use std::sync::LazyLock;

use glm::Vec3;

pub mod ball;
pub mod cuboid;
pub mod cylinder;

pub static FLOOR_PLANE_VERTICES: LazyLock<Vec<Vec3>> = LazyLock::new(|| {
    vec![
        Vec3::new(100., 0., 100.),
        Vec3::new(100., 0., -100.),
        Vec3::new(-100., 0., -100.),
        Vec3::new(-100., 0., 100.),
    ]
});

pub static FLOOR_PLANE_INDICES: LazyLock<Vec<u16>> = LazyLock::new(|| vec![0, 1, 2, 2, 3, 0]);

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
